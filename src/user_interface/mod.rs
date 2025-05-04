mod assets;
mod styles;

use std::{
    time::{Duration, Instant},
    usize,
};

use crate::minesweeper::*;
use assets::MinesweeperAssets;
use iced::{
    mouse::{self, Interaction},
    padding, time,
    widget::{image, Button, Column, Container, Image, MouseArea, Row, Text},
    window::{self},
    Alignment, Element, Length, Size, Subscription, Task,
};
use styles::ContainerStyles;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewGamePressed,
    NewGameReleased,
    NewGameOpenMenu,
    NewGameStart(GameDifficulty),
    OpenPressed,
    OpenReleased,
    Open(Position),
    Flag(Position),
    Tick(Instant),
}

/// Enum representing possible game difficulties
#[derive(Debug, Clone, Copy)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
pub struct MinesweeperInterface {
    face_pressed: bool,
    open_pressed: bool,
    show_new_game_menu: bool,
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
            show_new_game_menu: false,
            timer: 0,
            timer_enabled: false,
            game: Minesweeper::new(9, 9, 10),
            assets: Default::default(),
        }
    }
}

impl MinesweeperInterface {
    const EDGE_PADDING: u16 = 10;
    const BORDER_PADDING: u16 = 2;
    const FIELD_SIZE: u16 = 16;
    const SCALE_FACTOR: u16 = 2;

    pub fn view(&self) -> Element<Message> {
        // Layout in a column
        return Container::new(
            Column::new()
                .push(
                    // Controls row
                    self.render_wrapper_container(
                        Row::new()
                            .push(self.render_remaining_mines_count())
                            .push(self.render_face())
                            .push(self.render_timer())
                            .into(),
                    ),
                )
                // Game board
                .push(self.render_wrapper_container(self.render_board()))
                .spacing(Self::EDGE_PADDING)
                .align_x(Alignment::Center),
        )
        .style(ContainerStyles::game_container)
        .padding(Self::EDGE_PADDING)
        .into();
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Field open logic
            Message::Open(pos) => {
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
            Message::OpenPressed => {
                self.open_pressed = true;
            }
            Message::OpenReleased => {
                self.open_pressed = false;
            }

            // Field flag logic
            Message::Flag(pos) => {
                self.game.flag(pos);
                self.timer_enabled = true;
                self.open_pressed = false;
                println!("Flag '({}, {})'", pos.0, pos.1);
            }

            // New game logic
            Message::NewGamePressed => {
                self.face_pressed = true;
            }
            Message::NewGameReleased => {
                self.face_pressed = false;
            }
            Message::NewGameOpenMenu => {
                self.show_new_game_menu = true;
                self.face_pressed = false;
                self.timer_enabled = false;
                self.timer = 0;
            }
            Message::NewGameStart(difficulty) => {
                self.show_new_game_menu = false;
                self.game = match difficulty {
                    GameDifficulty::Easy => Minesweeper::new(9, 9, 10),
                    GameDifficulty::Medium => Minesweeper::new(16, 16, 40),
                    GameDifficulty::Hard => Minesweeper::new(30, 16, 99),
                };
                println!("Starting new game with difficulty {:?}", difficulty);

                // Return re-size task
                let size = self.calculate_size();
                return window::get_latest().and_then(move |id| window::resize(id, size));
            }

            // Timer logic
            Message::Tick(_) => {
                if self.timer_enabled && self.game.game_state == GameState::InProgress {
                    self.timer += 1;
                }
            }
        }

        return Task::none();
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(Message::Tick)
    }

    pub fn scale_factor(&self) -> f64 {
        return Self::SCALE_FACTOR.into();
    }

    pub fn calculate_size(&self) -> Size {
        let width = (self.game.width * Self::FIELD_SIZE)
            + (Self::EDGE_PADDING * 2)
            + (Self::BORDER_PADDING * 2);

        let height = ((self.game.height + 1) * Self::FIELD_SIZE)
            + (Self::EDGE_PADDING * 4)
            + (Self::BORDER_PADDING * 3);

        return Size::new(
            (width * Self::SCALE_FACTOR).into(),
            (height * Self::SCALE_FACTOR).into(),
        );
    }

    fn render_board(&self) -> Element<Message> {
        // Build the game board
        let mut board = Column::new().height(Length::Fill).width(Length::Fill);

        if self.show_new_game_menu {
            board = board
                .push(
                    Column::new()
                        .push(self.render_button(
                            String::from("Easy"),
                            Message::NewGameStart(GameDifficulty::Easy),
                        ))
                        .push(self.render_button(
                            String::from("Medium"),
                            Message::NewGameStart(GameDifficulty::Medium),
                        ))
                        .push(self.render_button(
                            String::from("Hard"),
                            Message::NewGameStart(GameDifficulty::Hard),
                        ))
                        .spacing(5)
                        .max_width(200),
                )
                .align_x(Alignment::Center)
                .padding(padding::all(25))
        } else {
            for y in 0..self.game.height {
                let mut row = Row::new();
                for x in 0..self.game.width {
                    // Create a cell for each game grid cell
                    row = row.push(self.render_field(x, y));
                }
                board = board.push(row);
            }
        }

        return board.into();
    }

    fn render_button(&self, text: String, message: Message) -> Element<Message> {
        return MouseArea::new(
            Container::new(
                Container::new(
                    Container::new(
                        Text::new(text)
                            .align_x(Alignment::Center)
                            .width(Length::Fill)
                            .size(11),
                    )
                    .style(ContainerStyles::button_container)
                    .padding(5),
                )
                .style(ContainerStyles::button_container_bottom_right)
                .padding(padding::bottom(Self::BORDER_PADDING).right(Self::BORDER_PADDING)),
            )
            .style(ContainerStyles::button_container_top_left)
            .padding(padding::top(Self::BORDER_PADDING).left(Self::BORDER_PADDING)),
        )
        .interaction(Interaction::Pointer)
        .on_release(message)
        .into();
    }

    fn render_wrapper_container<'a, Message>(
        &self,
        content: Element<'a, Message>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        return Container::new(
            Container::new(
                Container::new(content)
                    .style(ContainerStyles::wrapper_container)
                    .center(Length::Shrink),
            )
            .style(ContainerStyles::wrapper_container_top_left)
            .padding(padding::left(Self::BORDER_PADDING).top(Self::BORDER_PADDING))
            .center(Length::Shrink),
        )
        .style(ContainerStyles::wrapper_container_bottom_right)
        .padding(padding::bottom(Self::BORDER_PADDING).right(Self::BORDER_PADDING))
        .center(Length::Shrink)
        .into();
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
                    .on_release(Message::NewGameOpenMenu)
                    .on_exit(Message::NewGameReleased),
            )
            .align_x(Alignment::Center)
            .width(Length::FillPortion(1))
            .into();
    }

    fn render_field(&self, x: u16, y: u16) -> Element<Message> {
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
