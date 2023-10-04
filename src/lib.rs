mod graphic;
mod window;

use std::time::Duration;

use clap::{command, Arg};
use graphic::State;
use window::XorgWindow;

#[derive(thiserror::Error, Debug)]
pub enum RiverError {
    #[error(transparent)]
    Texture(#[from] crate::graphic::texture::Error),
}

pub async fn run() -> ! {
    env_logger::init();

    let args = command!()
        .arg(
            Arg::new("PATH")
                .required(true)
                .help("The absolute path to a JPG or PNG"),
        )
        .get_matches();
    let path: &str = args.get_one::<String>("PATH").unwrap();

    let window = XorgWindow;
    let mut state = State::new(window, path).await.unwrap();

    loop {
        state.render().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
}
