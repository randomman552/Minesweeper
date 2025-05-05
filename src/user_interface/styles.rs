use iced::{widget::container, Background, Color, Theme};

/// Struct containing styles for [container]
pub struct ContainerStyles {}
impl ContainerStyles {
    /// Container style for the top level game container
    pub fn game_container(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(191, 191, 191))),
            ..Default::default()
        }
    }

    /// Container style for wrapper containers
    ///
    /// Surrounds element with borders similar to class minesweeper
    ///
    /// This style provides the background
    pub fn wrapper_container(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(76, 76, 76))),
            ..Default::default()
        }
    }

    /// Container style for wrapper containers
    ///
    /// Surrounds element with borders similar to class minesweeper
    ///
    /// This style provides the top left border
    pub fn wrapper_container_top_left(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(127, 127, 127))),
            ..Default::default()
        }
    }

    /// Container style for wrapper containers
    ///
    /// Surrounds element with borders similar to class minesweeper
    ///
    /// This style provides the bottom right border
    pub fn wrapper_container_bottom_right(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button
    pub fn button_container(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(191, 191, 191))),
            text_color: Some(Color::from_rgb8(0, 0, 0)),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button
    ///
    /// This is the top left border style
    pub fn button_container_top_left(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button
    ///
    /// This is the bottom right border style
    pub fn button_container_bottom_right(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(127, 127, 127))),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button being pressed
    pub fn button_container_pressed(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(127, 127, 127))),
            text_color: Some(Color::from_rgb8(0, 0, 0)),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button being pressed
    ///
    /// This is the top left border style
    pub fn button_container_top_left_pressed(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(127, 127, 127))),
            ..Default::default()
        }
    }

    /// Container style for a windows 95 style bevelled button being pressed
    ///
    /// This is the bottom right border style
    pub fn button_container_bottom_right_pressed(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(127, 127, 127))),
            ..Default::default()
        }
    }
}
