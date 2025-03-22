use crate::{
    Css, CssValueT,
    value::{self, color, length::Length},
};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum CssProperty {
    Nil,
    Width,
    Height,
    Color,
}

#[const_trait]
pub trait CssPropertyT {
    fn property() -> CssProperty;
}

#[derive(Debug, Clone, Copy)]
pub struct Nil;
pub struct Width;
pub struct Height;
pub struct Color;
pub struct BackgroundColor;

impl CssValueT for Nil {}

impl const CssPropertyT for Nil {
    fn property() -> CssProperty {
        CssProperty::Nil
    }
}

impl const CssPropertyT for Width {
    fn property() -> CssProperty {
        CssProperty::Width
    }
}

impl const CssPropertyT for Height {
    fn property() -> CssProperty {
        CssProperty::Height
    }
}

impl const CssPropertyT for Color {
    fn property() -> CssProperty {
        CssProperty::Color
    }
}

impl const CssPropertyT for BackgroundColor {
    fn property() -> CssProperty {
        CssProperty::Color
    }
}

impl<const LEN: usize> Css<Nil, Nil, LEN> {
    const fn helper<Needed: CssValueT>(self, prop: CssProperty) -> Css<Nil, Needed, { LEN + 1 }> {
        let Self(css, _) = self;
        let mut buffer = [(prop, value::CssValue::None); LEN + 1];
        let mut i = 0;
        while i < LEN {
            buffer[i] = css[i];
            i += 1;
        }
        Css(buffer, PhantomData)
    }
    pub const fn width(self) -> Css<Nil, Length, { LEN + 1 }> {
        self.helper(Width::property())
    }
    pub const fn height(self) -> Css<Nil, Length, { LEN + 1 }> {
        self.helper(Height::property())
    }
    pub const fn color(self) -> Css<Nil, color::Color, { LEN + 1 }> {
        self.helper(Color::property())
    }
    pub const fn background_color(self) -> Css<Nil, color::Color, { LEN + 1 }> {
        self.helper(BackgroundColor::property())
    }
}
