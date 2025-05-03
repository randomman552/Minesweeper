use iced::{Border, Theme};

pub struct ContainerStyles {}
impl ContainerStyles {
    /// Style for a field that has yet to be opened
    pub fn closed_field_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(palette.background.strong.color.into()),
            border: Border {
                width: 1.0,
                radius: 5.0.into(),
                color: palette.background.strong.color,
            },
            ..Default::default()
        }
    }

    /// Style for a field that has been opened
    pub fn open_field_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 1.0,
                radius: 5.0.into(),
                color: palette.background.strong.color,
            },
            ..Default::default()
        }
    }

    /// Style for a field that has been opened, and contains a mine (ouch)
    pub fn exploded_field_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(palette.background.base.color.into()),
            border: Border {
                width: 1.0,
                radius: 5.0.into(),
                color: palette.background.strong.color,
            },
            ..Default::default()
        }
    }
}
