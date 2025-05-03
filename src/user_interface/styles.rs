use iced::{widget::container, Background, Color, Theme};

pub struct ContainerStyles {}
impl ContainerStyles {
    pub fn game_container(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.75, 0.75, 0.75))),
            ..Default::default()
        }
    }

    pub fn wrapper_container(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
            ..Default::default()
        }
    }

    pub fn wrapper_container_top_left(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            ..Default::default()
        }
    }

    pub fn wrapper_container_bottom_right(_: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
            ..Default::default()
        }
    }
}
