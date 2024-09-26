use std::fmt::Display;

use crate::{color::Color, length::Length};

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    AccentColor(Color),
    FontSize(Length),
    Margin(Length),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Padding(Length),
    BackgroundColor(Color),
    BackgroundImage(&'static str),
    // BackgroundPositionX(background::PositionX),
    // BackgroundPositionY(background::PositionY),
    // BackgroundPosition(background::XYPosition),
    // BackgroundSize(background::Size),
    // SimpleAttribute(SimpleAttribute),
    None,
}

impl Attribute {
    pub const fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Attribute::AccentColor(_), Attribute::AccentColor(_))
                | (Attribute::FontSize(_), Attribute::FontSize(_))
                | (Attribute::Top(_), Attribute::Top(_))
                | (Attribute::Bottom(_), Attribute::Bottom(_))
                | (Attribute::Right(_), Attribute::Right(_))
                | (Attribute::Left(_), Attribute::Left(_))
                | (Attribute::Height(_), Attribute::Height(_))
                | (Attribute::Width(_), Attribute::Width(_))
                | (Attribute::Margin(_), Attribute::Margin(_))
                | (Attribute::Padding(_), Attribute::Padding(_))
                | (Attribute::BackgroundColor(_), Attribute::BackgroundColor(_))
                | (Attribute::BackgroundImage(_), Attribute::BackgroundImage(_))
        )
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attribute::AccentColor(x) => format!("accent-color:{x};"),
            Attribute::FontSize(x) => format!("font-size:{x};"),
            Attribute::Top(x) => format!("top:{x};"),
            Attribute::Bottom(x) => format!("bottom:{x};"),
            Attribute::Right(x) => format!("right:{x};"),
            Attribute::Left(x) => format!("left:{x};"),
            Attribute::Height(x) => format!("height:{x};"),
            Attribute::Width(x) => format!("width:{x};"),
            Attribute::Margin(x) => format!("margin:{x};"),
            Attribute::Padding(x) => format!("padding:{x};"),
            Attribute::BackgroundColor(x) => {
                format!("background-color:{x};")
            }
            Attribute::BackgroundImage(x) => format!("background-image:url({x});"),
            Attribute::None => "".to_string(),
            // Attribute::BackgroundPosition(x) => format!("background-position:{x};"),
            // Attribute::BackgroundPositionX(x) => format!("background-position-x:{x};"),
            // Attribute::BackgroundPositionY(x) => format!("background-position-y:{x};"),
            // Attribute::BackgroundSize(x) => format!("background-size:{x};"),
            // Attribute::SimpleAttribute(x) => x.to_string(),
        };
        write!(f, "{}", result)
    }
}
