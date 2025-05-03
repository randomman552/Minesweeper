mod minesweeper;
mod user_interface;

use iced::Font;
use user_interface::*;

fn main() -> iced::Result {
    iced::application(
        MinesweeperInterface::title,
        MinesweeperInterface::update,
        MinesweeperInterface::view,
    )
    .font(include_bytes!("../resources/fonts/IBMPlexSans.ttf").as_slice())
    .default_font(Font::with_name("IBM Plex Sans"))
    .antialiasing(true)
    .run()
}
