mod minesweeper;
mod user_interface;

use iced::Font;
use user_interface::*;

fn main() -> iced::Result {
    iced::application(
        UserInterface::title,
        UserInterface::update,
        UserInterface::view,
    )
    .font(include_bytes!("../resources/fonts/IBMPlexSans-Italic.ttf").as_slice())
    .font(include_bytes!("../resources/fonts/IBMPlexSans.ttf").as_slice())
    .default_font(Font::with_name("IBM Plex Sans"))
    .run()
}
