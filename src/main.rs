mod minesweeper;

use std::usize;

use iced::{
    widget::{button, column, row, text, Button, Column, Row, Text},
    Element,
};
use minesweeper::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Open(Position),
    Flag(Position),
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
    fn view(&self) -> Element<Message> {
        // Build each row
        let mut grid = Column::new().spacing(5);
        for y in 0..self.game.height {
            let mut row = Row::new().spacing(5);
            for x in 0..self.game.width {
                let mine_count = self.get_cell_text(x, y);

                // Push a new button for each cell in the grid
                row = row.push(
                    Button::new(Text::new(mine_count).center())
                        .padding(10)
                        .width(50)
                        .height(50)
                        .on_press(Message::Open((x, y))),
                );
            }
            grid = grid.push(row);
        }

        return grid.into();
    }

    fn get_cell_text(&self, x: usize, y: usize) -> String {
        let mut mine_count = String::from("#");
        if self.game.is_open((x, y)) {
            if self.game.is_mined((x, y)) {
                mine_count = String::from("*");
            } else {
                mine_count = self.game.neighboring_mines((x, y)).to_string();
            }
        }

        return mine_count;
    }

    fn update(&mut self, message: Message) {
        if let Message::Open(pos) = message {
            self.game.open(pos);
        }
        if let Message::Flag(pos) = message {
            self.game.flag(pos);
        }
    }
}

fn main() -> iced::Result {
    iced::run("Minesweeper", UserInterface::update, UserInterface::view)
}
