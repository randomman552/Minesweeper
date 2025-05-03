mod minesweeper;
mod user_interface;

use user_interface::*;

fn main() -> iced::Result {
    iced::run("Minesweeper", UserInterface::update, UserInterface::view)
}
