use std::ptr::NonNull;

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle, XlibDisplayHandle, XlibWindowHandle,
};
use winit::{dpi::PhysicalSize, window::WindowId};
use x11::xlib::{
    Display, XDefaultScreen, XDisplayHeight, XDisplayWidth, XOpenDisplay, XRootWindow,
};

use super::RiwerWindow;

struct DisplayScreen {
    display_ptr: *mut Display,
    default_screen: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XorgWindow;

impl XorgWindow {
    fn display_screen() -> Option<DisplayScreen> {
        let display_ptr = unsafe { XOpenDisplay(std::ptr::null()) };
        if display_ptr.is_null() {
            return None;
        }

        Some(DisplayScreen {
            display_ptr,
            default_screen: unsafe { XDefaultScreen(display_ptr) },
        })
    }
}

impl HasWindowHandle for XorgWindow {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        let data = Self::display_screen().ok_or(HandleError::Unavailable)?;

        let window_id = unsafe { XRootWindow(data.display_ptr, data.default_screen) };
        let raw_handler = XlibWindowHandle::new(window_id);
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Xlib(raw_handler)) })
    }
}

impl HasDisplayHandle for XorgWindow {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        let data = Self::display_screen().ok_or(HandleError::Unavailable)?;
        let display = NonNull::new(data.display_ptr).unwrap().cast();

        let raw_handler = XlibDisplayHandle::new(Some(display), data.default_screen);
        Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Xlib(raw_handler)) })
    }
}

impl RiwerWindow for XorgWindow {
    fn inner_size(&self) -> PhysicalSize<u32> {
        let data = Self::display_screen().expect("Couldn't retrieve display");
        let width = unsafe { XDisplayWidth(data.display_ptr, data.default_screen) };
        let height = unsafe { XDisplayHeight(data.display_ptr, data.default_screen) };

        PhysicalSize {
            width: width as u32,
            height: height as u32,
        }
    }

    fn id(&self) -> WindowId {
        let data = Self::display_screen().expect("Couldn't retrieve display");

        let id = unsafe {XRootWindow(data.display_ptr, data.default_screen)};
        println!("Window-ID: {}", id);
        WindowId::from(id)
    }
}
