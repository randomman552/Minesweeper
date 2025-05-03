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
    .scale_factor(MinesweeperInterface::scale_factor)
    .resizable(false)
    .font(include_bytes!("../resources/fonts/IBMPlexSans.ttf").as_slice())
    .default_font(Font::with_name("IBM Plex Sans"))
    .antialiasing(false)
    .window_size(MinesweeperInterface::default().calculate_size())
    .run()
}
