use crate::minesweeper::*;
use iced::{
    widget::{container, Column, Container, MouseArea, Row, Text},
    Element, Font,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Open(Position),
    Flag(Position),
}

pub struct MinesweeperInterface {
    game: Minesweeper,
}

impl Default for MinesweeperInterface {
    fn default() -> Self {
        Self {
            game: Minesweeper::new(10, 10, 10),
        }
    }
}

impl MinesweeperInterface {
    pub fn view(&self) -> Element<Message> {
        // Build each row
        let mut grid = Column::new().spacing(5);
        for y in 0..self.game.height {
            let mut row = Row::new().spacing(5);
            for x in 0..self.game.width {
                // Create a cell for each game grid cell
                row = row.push(self.get_cell(x, y));
            }
            grid = grid.push(row);
        }

        return grid.into();
    }

    pub fn update(&mut self, message: Message) {
        if let Message::Open(pos) = message {
            self.game.open(pos);
        }
        if let Message::Flag(pos) = message {
            self.game.flag(pos);
        }
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn get_cell(&self, x: usize, y: usize) -> Element<Message> {
        let mine_count = self.get_cell_text(x, y);

        MouseArea::new(
            Container::new(Text::new(mine_count).center())
                .width(50)
                .height(50)
                .padding(10)
                .style(container::bordered_box)
                .center(50),
        )
        .on_press(Message::Open((x, y)))
        .on_right_press(Message::Flag((x, y)))
        .into()
    }

    fn get_cell_text(&self, x: usize, y: usize) -> String {
        let mut text = String::from("TEST");

        // Show flagged
        if self.game.is_flagged((x, y)) {
            text = String::from("F");
        }
        // Show the mine count or a mine if the field is open
        else if self.game.is_open((x, y)) {
            if self.game.is_mined((x, y)) {
                text = String::from("*");
            } else {
                text = self.game.neighboring_mines((x, y)).to_string();
            }
        }

        return text;
    }
}
