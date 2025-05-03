mod styles;

use crate::minesweeper::*;
use iced::{
    mouse,
    widget::{container, Column, Container, MouseArea, Row, Text},
    Element,
};
use styles::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Open(Position),
    Flag(Position),
    HoverEnter(Position),
    HoverExit(Position),
}

pub struct MinesweeperInterface {
    hovered: Option<Position>,
    game: Minesweeper,
}

impl Default for MinesweeperInterface {
    fn default() -> Self {
        Self {
            hovered: None,
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
        let pos = (x, y);
        let field_state = self.game.get_field_state(pos);

        // Get the field content
        let cell_content: Element<Message> = match field_state {
            FieldState::Unknown => Text::new(String::from("#")).into(),
            FieldState::Flagged => Text::new(String::from("F")).into(),
            FieldState::MineDefused => Text::new(String::from("*")).into(),
            FieldState::MineDetonated => Text::new(String::from("*")).into(),
            FieldState::Open(count) => Text::new(count.to_string()).into(),
        };

        // Get the style of the field container
        let container_style = match field_state {
            FieldState::Open(_) => ContainerStyles::open_field_style,
            FieldState::MineDetonated => ContainerStyles::exploded_field_style,
            _ => ContainerStyles::closed_field_style,
        };

        // Create the field (with interaction logic)
        MouseArea::new(
            Container::new(cell_content)
                .width(50)
                .height(50)
                .padding(10)
                .style(container_style)
                .center(50),
        )
        .on_press(Message::Open(pos))
        .on_right_press(Message::Flag(pos))
        .on_enter(Message::HoverEnter(pos))
        .on_exit(Message::HoverExit(pos))
        .interaction(mouse::Interaction::Pointer)
        .into()
    }
}
