mod assets;
mod styles;

use crate::minesweeper::*;
use assets::MinesweeperAssets;
use iced::{
    alignment, mouse,
    widget::{image, Column, Container, MouseArea, Row},
    Element,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewGamePressed,
    NewGameReleased,
    NewGameStart,
    Open(Position),
    Flag(Position),
}

#[derive(Debug)]
pub struct MinesweeperInterface {
    face_pressed: bool,
    game: Minesweeper,
    assets: MinesweeperAssets,
}

impl Default for MinesweeperInterface {
    fn default() -> Self {
        Self {
            face_pressed: false,
            game: Minesweeper::new(10, 10, 10),
            assets: Default::default(),
        }
    }
}

impl MinesweeperInterface {
    pub fn view(&self) -> Element<Message> {
        // Build the game board
        let mut board = Column::new();
        for y in 0..self.game.height {
            let mut row = Row::new();
            for x in 0..self.game.width {
                // Create a cell for each game grid cell
                row = row.push(self.render_field(x, y));
            }
            board = board.push(row);
        }

        // Render the controls row
        let mut controls_row = Row::new();
        let face = self.render_face();
        controls_row = controls_row.push(face);

        // Layout in a column
        let mut column = Column::new();
        column = column.push(controls_row);
        column = column.push(board);

        return column.align_x(alignment::Horizontal::Center).into();
    }

    pub fn update(&mut self, message: Message) {
        if let Message::Open(pos) = message {
            let result = self.game.open(pos);
            if !result.is_none() {
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
        if let Message::NewGamePressed = message {
            self.face_pressed = true;
        }
        if let Message::NewGameReleased = message {
            self.face_pressed = false;
        }
        if let Message::NewGameStart = message {
            self.face_pressed = false;
            self.game = Minesweeper::new(10, 10, 10);
            println!("Starting new game");
        }
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn render_face(&self) -> Element<Message> {
        const FACE_SIZE: u16 = 64;

        // Get face image based on current game state
        let mut face_image = match self.game.game_state {
            GameState::InProgress => image(&self.assets.face),
            GameState::Loss => image(&self.assets.face_lose),
            GameState::Win => image(&self.assets.face_win),
        };

        // Override image if currently pressed down
        if self.face_pressed {
            face_image = image(&self.assets.face_pressed);
        }

        // Create mouse area with interaction logic
        return MouseArea::new(
            Container::new(face_image.width(FACE_SIZE).height(FACE_SIZE))
                .width(FACE_SIZE)
                .height(FACE_SIZE),
        )
        .interaction(mouse::Interaction::Pointer)
        .on_press(Message::NewGamePressed)
        .on_release(Message::NewGameStart)
        .on_exit(Message::NewGameReleased)
        .into();
    }

    fn render_field(&self, x: usize, y: usize) -> Element<Message> {
        const FIELD_SIZE: u16 = 32;
        let pos = (x, y);
        let field_state = self.game.get_field_state(pos);

        // Get the field content
        let cell_content = match field_state {
            FieldState::Unknown => image(&self.assets.closed),
            FieldState::Flagged => image(&self.assets.flag),
            FieldState::MineRevealed => image(&self.assets.mine),
            FieldState::MineDefused => image(&self.assets.mine_defused),
            FieldState::MineDetonated => image(&self.assets.mine_detonated),
            FieldState::Open(count) => match count {
                0 => image(&self.assets.field0),
                1 => image(&self.assets.field1),
                2 => image(&self.assets.field2),
                3 => image(&self.assets.field3),
                4 => image(&self.assets.field4),
                5 => image(&self.assets.field5),
                6 => image(&self.assets.field6),
                7 => image(&self.assets.field7),
                8 => image(&self.assets.field8),
                _ => panic!("Mine count out of range 0 - 8"),
            },
        };

        // Create the field (with interaction logic)
        MouseArea::new(
            Container::new(cell_content.width(FIELD_SIZE).height(FIELD_SIZE))
                .width(FIELD_SIZE)
                .height(FIELD_SIZE),
        )
        .on_press(Message::Open(pos))
        .on_right_press(Message::Flag(pos))
        .interaction(mouse::Interaction::Pointer)
        .into()
    }
}
