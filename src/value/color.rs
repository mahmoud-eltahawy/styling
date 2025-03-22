use crate::{Css, CssValueT, Nil, value::CssValue};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Hex(&'static str),
    THex([char; 8]),
    Rgb(f32, f32, f32),
    Rgba(f32, f32, f32, f32),
    Hsl(f32, f32, f32),
    Hsla(f32, f32, f32, f32),
    TodoColors,
}

impl CssValueT for Color {}

impl<const LEN: usize> Css<Nil, Color, LEN> {
    pub const fn helper(self, color: Color) -> Css<Nil, Nil, LEN> {
        let Self(mut css, _) = self;
        css[LEN - 1].1 = CssValue::Color(color);
        Css(css, PhantomData)
    }
    pub const fn hex(self, value: &'static str) -> Css<Nil, Nil, LEN> {
        assert!(value.len() == 6 || value.len() == 7, "wrong hex format");
        self.helper(Color::Hex(value))
    }
}
