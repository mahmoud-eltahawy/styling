use std::fmt::Display;

use crate::{color::Color, length::Length};

#[derive(Debug, Clone)]
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
    BackgroundImage(String),
    // BackgroundPositionX(background::PositionX),
    // BackgroundPositionY(background::PositionY),
    // BackgroundPosition(background::XYPosition),
    // BackgroundSize(background::Size),
    // SimpleAttribute(SimpleAttribute),
}

impl Attribute {
    fn repr(&self) -> u8 {
        match self {
            Attribute::AccentColor(_) => 0,
            Attribute::FontSize(_) => 1,
            Attribute::Margin(_) => 2,
            Attribute::Top(_) => 3,
            Attribute::Bottom(_) => 4,
            Attribute::Right(_) => 5,
            Attribute::Left(_) => 6,
            Attribute::Height(_) => 7,
            Attribute::Width(_) => 8,
            Attribute::Padding(_) => 9,
            Attribute::BackgroundColor(_) => 10,
            Attribute::BackgroundImage(_) => 11,
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.repr() == other.repr()
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
            // Attribute::BackgroundPosition(x) => format!("background-position:{x};"),
            // Attribute::BackgroundPositionX(x) => format!("background-position-x:{x};"),
            // Attribute::BackgroundPositionY(x) => format!("background-position-y:{x};"),
            // Attribute::BackgroundSize(x) => format!("background-size:{x};"),
            // Attribute::SimpleAttribute(x) => x.to_string(),
        };
        write!(f, "{}", result)
    }
}
