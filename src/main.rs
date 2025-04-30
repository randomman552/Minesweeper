mod minesweeper;

use iced::widget::{button, column, text, Column};
use minesweeper::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

struct UserInterface {
    value: i32,
    game: Minesweeper,
}

impl Default for UserInterface {
    fn default() -> Self {
        Self {
            value: 0,
            game: Minesweeper::new(10, 10, 10),
        }
    }
}

impl UserInterface {
    fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Minesweeper", UserInterface::update, UserInterface::view)
}
