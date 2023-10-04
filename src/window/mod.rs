use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

mod xorg;

use winit::{dpi::PhysicalSize, window::WindowId};
pub use xorg::XorgWindow;

pub trait RiwerWindow: HasWindowHandle + HasDisplayHandle { 
    fn inner_size(&self) -> PhysicalSize<u32>;

    fn id(&self) -> WindowId;
}
