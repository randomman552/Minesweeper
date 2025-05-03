mod assets;
mod styles;

use crate::minesweeper::*;
use assets::MinesweeperAssets;
use iced::{
    mouse,
    widget::{image, Column, Container, MouseArea, Row},
    Element,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Open(Position),
    Flag(Position),
    HoverEnter(Position),
    HoverExit(Position),
}

#[derive(Debug)]
pub struct MinesweeperInterface {
    hovered: Option<Position>,
    game: Minesweeper,
    assets: MinesweeperAssets,
}

impl Default for MinesweeperInterface {
    fn default() -> Self {
        Self {
            hovered: None,
            game: Minesweeper::new(10, 10, 10),
            assets: Default::default(),
        }
    }
}

impl MinesweeperInterface {
    pub fn view(&self) -> Element<Message> {
        // Build minesweeper
        let mut grid = Column::new();
        for y in 0..self.game.height {
            let mut row = Row::new();
            for x in 0..self.game.width {
                // Create a cell for each game grid cell
                row = row.push(self.get_field(x, y));
            }
            grid = grid.push(row);
        }

        return grid.into();
    }

    pub fn update(&mut self, message: Message) {
        if let Message::Open(pos) = message {
            let result = self.game.open(pos);
            if result.is_none() {
                println!("Open '({}, {})' with result '{}'", pos.0, pos.1, "None");
            } else {
                println!(
                    "Open '({}, {})' with result '{}'",
                    pos.0,
                    pos.1,
                    result.unwrap()
                );
            }
        }
        if let Message::Flag(pos) = message {
            self.game.flag(pos);
            println!("Flag '({}, {})'", pos.0, pos.1);
        }
        if let Message::HoverEnter(pos) = message {
            self.hovered = Some(pos);
            println!("HoverEnter '({}, {})'", pos.0, pos.1);
        }
        if let Message::HoverExit(pos) = message {
            self.hovered = None;
            println!("HoverExit '({}, {})'", pos.0, pos.1);
        }
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn get_field(&self, x: usize, y: usize) -> Element<Message> {
        const FIELD_SIZE: u16 = 32;
        let pos = (x, y);
        let field_state = self.game.get_field_state(pos);

        // Get the field content
        let cell_content: Element<Message> = match field_state {
            FieldState::Unknown => image(&self.assets.closed)
                .width(FIELD_SIZE)
                .height(FIELD_SIZE)
                .into(),
            FieldState::Flagged => image(&self.assets.flag)
                .width(FIELD_SIZE)
                .height(FIELD_SIZE)
                .into(),
            FieldState::MineDefused => image(&self.assets.mine)
                .width(FIELD_SIZE)
                .height(FIELD_SIZE)
                .into(),
            FieldState::MineDetonated => image(&self.assets.mine_detonated)
                .width(FIELD_SIZE)
                .height(FIELD_SIZE)
                .into(),
            FieldState::Open(count) => match count {
                0 => image(&self.assets.field0)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                1 => image(&self.assets.field1)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                2 => image(&self.assets.field2)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                3 => image(&self.assets.field3)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                4 => image(&self.assets.field4)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                5 => image(&self.assets.field5)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                6 => image(&self.assets.field6)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                7 => image(&self.assets.field7)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                8 => image(&self.assets.field8)
                    .width(FIELD_SIZE)
                    .height(FIELD_SIZE)
                    .into(),
                _ => panic!("Mine count out of range 0 - 8"),
            },
        };

        // Create the field (with interaction logic)
        MouseArea::new(
            Container::new(cell_content)
                .width(FIELD_SIZE)
                .height(FIELD_SIZE),
        )
        .on_press(Message::Open(pos))
        .on_right_press(Message::Flag(pos))
        .on_enter(Message::HoverEnter(pos))
        .on_exit(Message::HoverExit(pos))
        .interaction(mouse::Interaction::Pointer)
        .into()
    }
}
