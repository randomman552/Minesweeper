mod minesweeper;
mod solver;
mod user_interface;

use iced::{
    window::{icon, Settings},
    Font,
};
use user_interface::*;

fn main() -> iced::Result {
    let settings = Settings {
        icon: Some(
            icon::from_file_data(
                user_interface::assets::MinesweeperAssets::default().application_icon,
                None,
            )
            .unwrap(),
        ),
        ..Default::default()
    };

    iced::application(
        MinesweeperInterface::title,
        MinesweeperInterface::update,
        MinesweeperInterface::view,
    )
    .window(settings)
    .subscription(MinesweeperInterface::subscription)
    .scale_factor(MinesweeperInterface::scale_factor)
    .resizable(false)
    .default_font(Font::MONOSPACE)
    .antialiasing(false)
    .window_size(MinesweeperInterface::default().calculate_size())
    .run()
}
