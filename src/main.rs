mod minesweeper;
mod user_interface;

use iced::{Font, Size};
use user_interface::*;

fn main() -> iced::Result {
    iced::application(
        MinesweeperInterface::title,
        MinesweeperInterface::update,
        MinesweeperInterface::view,
    )
    .subscription(MinesweeperInterface::subscription)
    .font(include_bytes!("../resources/fonts/IBMPlexSans.ttf").as_slice())
    .default_font(Font::with_name("IBM Plex Sans"))
    .antialiasing(false)
    .window_size(Size::new(500.0, 500.0))
    .run()
}
