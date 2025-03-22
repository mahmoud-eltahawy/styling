#![feature(const_trait_impl)]
#![feature(const_heap)]
#![feature(generic_const_exprs)]

use color::Color;
use length::Length;
use property::{CssProperty, CssPropertyT, Nil};
use std::marker::PhantomData;

mod property;

mod color;
mod length;
#[const_trait]
pub trait CssValueT {}
#[derive(Debug, Clone, Copy)]
enum CssValue {
    None,
    Length(Length),
    Color(Color),
}

#[derive(Debug)]
pub struct Css<Property: CssPropertyT, Value: CssValueT, const LEN: usize>(
    [(CssProperty, CssValue); LEN],
    PhantomData<(Property, Value)>,
);

pub const fn css() -> Css<Nil, Nil, 0> {
    Css([], PhantomData)
}
