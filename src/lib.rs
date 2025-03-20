#![feature(const_trait_impl)]
#![feature(const_heap)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use length::Length;
mod length;

#[const_trait]
pub trait CssPropertyT {
    fn property() -> CssProperty;
}
#[const_trait]
pub trait CssValueT {}

#[derive(Debug, Clone, Copy)]
pub enum CssProperty {
    None,
    Width,
    Height,
}
#[derive(Debug, Clone, Copy)]
enum CssValue {
    None,
    Length(Length),
}

#[derive(Debug, Clone, Copy)]
pub struct Nil;

impl CssValueT for Nil {}
impl const CssPropertyT for Nil {
    fn property() -> CssProperty {
        CssProperty::None
    }
}

#[derive(Debug)]
pub struct Css<Property: CssPropertyT, Needs: CssValueT, const LEN: usize>(
    [(CssProperty, CssValue); LEN],
    PhantomData<(Property, Needs)>,
);

pub struct Width;
pub struct Height;

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

impl<const LEN: usize> Css<Nil, Nil, LEN> {
    const fn helper(self, prop: CssProperty) -> Css<Nil, Length, { LEN + 1 }> {
        let Self(css, _) = self;
        let mut buffer = [(prop, CssValue::None); LEN + 1];
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
}

pub const fn css() -> Css<Nil, Nil, 0> {
    Css([], PhantomData)
}
