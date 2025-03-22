use color::Color;
use length::Length;

pub mod color;
pub mod length;

#[const_trait]
pub trait CssValueT {}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CssValue {
    None,
    Length(Length),
    Color(Color),
}
