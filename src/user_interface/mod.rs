mod assets;
mod styles;

use std::{
    time::{Duration, Instant},
    usize,
};

use crate::minesweeper::*;
use assets::MinesweeperAssets;
use iced::{
    mouse, time,
    widget::{image, Column, Container, Image, MouseArea, Row},
    Alignment, Element, Length, Subscription,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewGamePressed,
    NewGameReleased,
    NewGameStart,
    OpenPressed,
    OpenReleased,
    Open(Position),
    Flag(Position),
    Tick(Instant),
}

#[derive(Debug)]
pub struct MinesweeperInterface {
    face_pressed: bool,
    open_pressed: bool,
    game: Minesweeper,
    assets: MinesweeperAssets,
    timer: usize,
    timer_enabled: bool,
}

impl Default for MinesweeperInterface {
    fn default() -> Self {
        Self {
            face_pressed: false,
            open_pressed: false,
            timer: 0,
            timer_enabled: false,
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
        let controls_row = Row::new()
            .push(self.render_remaining_mines_count())
            .push(self.render_face())
            .push(self.render_timer());

        // Layout in a column
        let mut column = Column::new();
        column = column.push(controls_row);
        column = column.push(board);

        return column.align_x(Alignment::Center).into();
    }

    pub fn update(&mut self, message: Message) {
        // Field open logic
        if let Message::Open(pos) = message {
            let result = self.game.open(pos);
            self.timer_enabled = true;
            self.open_pressed = false;
            if !result.is_none() {
                println!(
                    "Open '({}, {})' with result '{}'",
                    pos.0,
                    pos.1,
                    result.unwrap()
                );
            }
        }
        if let Message::OpenPressed = message {
            self.open_pressed = true;
        }
        if let Message::OpenReleased = message {
            self.open_pressed = false;
        }

        // Field flag logic
        if let Message::Flag(pos) = message {
            self.game.flag(pos);
            self.timer_enabled = true;
            self.open_pressed = false;
            println!("Flag '({}, {})'", pos.0, pos.1);
        }

        // New game logic
        if let Message::NewGamePressed = message {
            self.face_pressed = true;
        }
        if let Message::NewGameReleased = message {
            self.face_pressed = false;
        }
        if let Message::NewGameStart = message {
            self.face_pressed = false;
            self.timer_enabled = false;
            self.game = Minesweeper::new(10, 10, 10);
            println!("Starting new game");
        }

        // Timer logic
        if let Message::Tick(_) = message {
            if self.timer_enabled && self.game.game_state == GameState::InProgress {
                self.timer += 1;
            }
        }
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(Message::Tick)
    }

    fn render_remaining_mines_count(&self) -> Element<Message> {
        let mine_count = self.game.remaining_mines();

        return self
            .render_seven_seg_number(mine_count, 3)
            .align_x(Alignment::Start)
            .width(Length::FillPortion(1))
            .into();
    }

    fn render_timer(&self) -> Element<Message> {
        return self
            .render_seven_seg_number(self.timer, 3)
            .align_x(Alignment::End)
            .width(Length::FillPortion(1))
            .into();
    }

    /// Render the given number in seven segment digits (with padding to min_length)
    fn render_seven_seg_number(&self, number: usize, min_length: usize) -> Column<Message> {
        // Create string and pad to the minimum length
        let mut number_string = number.to_string();
        while number_string.len() < min_length {
            number_string = String::from("0") + &number_string;
        }

        return self.render_seven_seg_string(number_string);
    }

    /// Render the given string in seven segment digits
    fn render_seven_seg_string(&self, str: String) -> Column<Message> {
        let mut row = Row::new();
        for (_, c) in str.chars().enumerate() {
            let image: Image = match c {
                '0' => image(&self.assets.score0),
                '1' => image(&self.assets.score1),
                '2' => image(&self.assets.score2),
                '3' => image(&self.assets.score3),
                '4' => image(&self.assets.score4),
                '5' => image(&self.assets.score5),
                '6' => image(&self.assets.score6),
                '7' => image(&self.assets.score7),
                '8' => image(&self.assets.score8),
                '9' => image(&self.assets.score9),
                '-' => image(&self.assets.score_dash),
                _ => image(&self.assets.score_empty),
            };
            row = row.push(Container::new(image))
        }

        return Column::new().push(row);
    }

    fn render_face(&self) -> Element<Message> {
        // Get face image based on current game state
        let mut face_image = match self.game.game_state {
            GameState::InProgress => image(&self.assets.face),
            GameState::Loss => image(&self.assets.face_lose),
            GameState::Win => image(&self.assets.face_win),
        };

        // Override image if currently pressed down
        if self.face_pressed {
            face_image = image(&self.assets.face_pressed);
        } else if self.open_pressed {
            face_image = image(&self.assets.face_open);
        }

        // Create mouse area with interaction logic
        return Column::new()
            .push(
                MouseArea::new(Container::new(face_image))
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(Message::NewGamePressed)
                    .on_release(Message::NewGameStart)
                    .on_exit(Message::NewGameReleased),
            )
            .align_x(Alignment::Center)
            .width(Length::FillPortion(1))
            .into();
    }

    fn render_field(&self, x: usize, y: usize) -> Element<Message> {
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
        MouseArea::new(Container::new(cell_content))
            .on_press(Message::OpenPressed)
            .on_right_press(Message::OpenPressed)
            .on_exit(Message::OpenReleased)
            .on_release(Message::Open(pos))
            .on_right_release(Message::Flag(pos))
            .interaction(mouse::Interaction::Pointer)
            .into()
    }
}
