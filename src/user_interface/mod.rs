pub mod assets;
mod styles;

use std::{
    ops::Deref,
    time::{Duration, Instant},
    usize,
};

use crate::{
    minesweeper::*,
    solver::{Solver, SolverStep},
};
use assets::MinesweeperAssets;
use iced::{
    event,
    keyboard::{self, key::Named, Key},
    mouse::{self, Interaction},
    padding, time,
    widget::{container, image, tooltip, Column, Container, Image, MouseArea, Row, Text},
    window::{self},
    Alignment, Color, Element, Event, Length, Size, Subscription, Task, Theme,
};
use log::info;
use styles::ContainerStyles;

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    NewGamePressed,
    NewGameReleased,
    NewGameOpenMenu,
    NewGameStart(GameDifficulty),
    RestartGame,
    OpenPressed,
    OpenReleased,
    CustomButtonPressed(String),
    CustomButtonReleased(Box<Option<Self>>),
    Open(Position),
    Flag(Position),
    Tick(Instant),
    ShowMineChance,
    HideMineChance,
    SolveStep,
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
    solver: Solver,
    assets: MinesweeperAssets,
    timer: usize,
    timer_enabled: bool,
    hovered_button_id: Option<String>,
    pressed_button_id: Option<String>,
    show_mine_chance: bool,
    difficulty: GameDifficulty,
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
            solver: Solver::new(),
            assets: Default::default(),
            hovered_button_id: None,
            pressed_button_id: None,
            show_mine_chance: false,
            difficulty: GameDifficulty::Easy,
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
        if let Message::Ignore = message {
            return Task::none();
        }

        if let Message::Tick(instant) = message {
            if self.timer_enabled && self.game.game_state == GameState::InProgress {
                log::trace!("Timer tick - {:?}", instant);
                self.timer += 1;
            }
            return Task::none();
        }

        let solve_step = self.solver.solve_step(&self.game);

        match message {
            // Field open logic
            Message::Open(pos) => {
                let result = self.game.open(pos);
                self.timer_enabled = true;
                self.open_pressed = false;
                if !result.is_none() {
                    log::info!(
                        "Opened '({}, {})' with result '{}'",
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
                log::info!("Flagged '({}, {})'", pos.0, pos.1);
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
                log::info!("Showing new game menu")
            }
            Message::NewGameStart(difficulty) => {
                self.show_new_game_menu = false;
                self.game = match difficulty {
                    GameDifficulty::Easy => Minesweeper::new(9, 9, 10),
                    GameDifficulty::Medium => Minesweeper::new(16, 16, 40),
                    GameDifficulty::Hard => Minesweeper::new(30, 16, 99),
                };
                self.difficulty = difficulty;
                self.timer_enabled = false;
                self.timer = 0;
                self.solver = Solver::new();
                log::info!("Starting new game with difficulty {:?}", difficulty);

                // Return re-size task
                let size = self.calculate_size();
                return window::get_latest().and_then(move |id| window::resize(id, size));
            }
            Message::RestartGame => {
                log::info!("Restarting game");
                let difficulty = self.difficulty;
                return Task::perform(async {}, move |_| Message::NewGameStart(difficulty));
            }

            // Custom button logic
            Message::CustomButtonPressed(id) => {
                log::debug!("Custom button with id '{}' pressed", id);
                self.pressed_button_id = Some(id);
            }
            Message::CustomButtonReleased(message_box) => {
                log::debug!(
                    "Custom button with id '{:?}' released",
                    self.pressed_button_id
                );
                self.pressed_button_id = None;
                let message = message_box.deref().to_owned();

                if message.is_some() {
                    return Task::perform(async {}, move |_| message.clone().unwrap());
                }
            }

            // Game solver related
            Message::ShowMineChance => {
                if !self.show_mine_chance {
                    self.show_mine_chance = true;
                    log::info!("Showing solver mine chance");
                }
            }
            Message::HideMineChance => {
                if self.show_mine_chance {
                    self.show_mine_chance = false;
                    log::info!("Hiding solver mine chance");
                }
            }
            Message::SolveStep => {
                info!("Running solver step '{}'", solve_step);
                let message = match solve_step {
                    SolverStep::Flag(pos) => Message::Flag(pos),
                    SolverStep::Open(pos) => Message::Open(pos),
                    SolverStep::None => Message::Ignore,
                };

                return Task::perform(async {}, move |_| message.clone());
            }

            // Ignore any other messages
            _ => {}
        }

        return Task::none();
    }

    pub fn title(&self) -> String {
        String::from("Minesweeper")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let _r_key = Key::Character('r');

        Subscription::batch(vec![
            // Timer
            time::every(Duration::from_secs(1)).map(Message::Tick),
            // Keyboard events
            event::listen().map(|event| match event {
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    keyboard::Event::KeyPressed {
                        key: Key::Named(Named::Alt),
                        location: _,
                        modified_key: _,
                        modifiers: _,
                        physical_key: _,
                        text: _,
                    } => Message::ShowMineChance,
                    keyboard::Event::KeyReleased {
                        key: Key::Named(Named::Alt),
                        location: _,
                        modifiers: _,
                    } => Message::HideMineChance,
                    keyboard::Event::KeyPressed {
                        key: Key::Named(Named::Enter),
                        location: _,
                        modified_key: _,
                        modifiers: _,
                        physical_key: _,
                        text: _,
                    } => Message::SolveStep,
                    keyboard::Event::KeyPressed {
                        key: _r_key,
                        location: _,
                        modified_key: _,
                        modifiers: _,
                        physical_key: _,
                        text: _,
                    } => Message::RestartGame,
                    _ => Message::Ignore,
                },
                _ => Message::Ignore,
            }),
        ])
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
                            String::from("easy-button"),
                            String::from("Easy"),
                            Message::NewGameStart(GameDifficulty::Easy),
                        ))
                        .push(self.render_button(
                            String::from("medium-button"),
                            String::from("Medium"),
                            Message::NewGameStart(GameDifficulty::Medium),
                        ))
                        .push(self.render_button(
                            String::from("hard-button"),
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

    fn render_button(&self, id: String, text: String, message: Message) -> Element<Message> {
        let mut button_container_style: Box<dyn Fn(&Theme) -> container::Style> =
            Box::new(ContainerStyles::button_container);
        let mut button_container_top_left_style: Box<dyn Fn(&Theme) -> container::Style> =
            Box::new(ContainerStyles::button_container_top_left);
        let mut button_container_bottom_right_style: Box<dyn Fn(&Theme) -> container::Style> =
            Box::new(ContainerStyles::button_container_bottom_right);

        // Change style if button pressed
        if self.pressed_button_id == Some(id.clone()) {
            button_container_style = Box::new(ContainerStyles::button_container_pressed);
            button_container_top_left_style =
                Box::new(ContainerStyles::button_container_top_left_pressed);
            button_container_bottom_right_style =
                Box::new(ContainerStyles::button_container_bottom_right_pressed);
        }

        return MouseArea::new(
            Container::new(
                Container::new(
                    Container::new(
                        Text::new(text)
                            .align_x(Alignment::Center)
                            .width(Length::Fill)
                            .size(11),
                    )
                    .style(button_container_style)
                    .padding(5),
                )
                .style(button_container_bottom_right_style)
                .padding(padding::bottom(Self::BORDER_PADDING).right(Self::BORDER_PADDING)),
            )
            .style(button_container_top_left_style)
            .padding(padding::top(Self::BORDER_PADDING).left(Self::BORDER_PADDING)),
        )
        .interaction(Interaction::Pointer)
        .on_press(Message::CustomButtonPressed(id.clone()))
        .on_release(Message::CustomButtonReleased(Box::new(Some(message))))
        .on_exit(Message::CustomButtonReleased(Box::new(None)))
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
        let cell_content: Element<Message> = match field_state {
            FieldState::Unknown => image(&self.assets.closed).into(),
            FieldState::Flagged => image(&self.assets.flag).into(),
            FieldState::Question => image(&self.assets.question_closed).into(),
            FieldState::MineRevealed => image(&self.assets.mine).into(),
            FieldState::NoMine => image(&self.assets.mine_false).into(),
            FieldState::MineDetonated => image(&self.assets.mine_detonated).into(),
            FieldState::Open(count) => match count {
                0 => image(&self.assets.field0).into(),
                1 => image(&self.assets.field1).into(),
                2 => image(&self.assets.field2).into(),
                3 => image(&self.assets.field3).into(),
                4 => image(&self.assets.field4).into(),
                5 => image(&self.assets.field5).into(),
                6 => image(&self.assets.field6).into(),
                7 => image(&self.assets.field7).into(),
                8 => image(&self.assets.field8).into(),
                _ => panic!("Mine count out of range 0 - 8"),
            },
        };

        // Create field content
        let field: Element<Message> = MouseArea::new(Container::new(cell_content))
            .on_press(Message::OpenPressed)
            .on_right_press(Message::OpenPressed)
            .on_exit(Message::OpenReleased)
            .on_release(Message::Open(pos))
            .on_right_release(Message::Flag(pos))
            .interaction(mouse::Interaction::Pointer)
            .into();

        // Add tooltip if mine chance enabled
        if self.show_mine_chance {
            let mine_chance = self.solver.get_mine_chance(pos);
            let mine_chance_string = format!("{}", mine_chance);
            return tooltip(
                field,
                Text::new(mine_chance_string).color(Color::BLACK),
                tooltip::Position::Bottom,
            )
            .into();
        }

        return field;
    }
}
