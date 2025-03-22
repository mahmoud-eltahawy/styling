#![feature(const_trait_impl)]
#![feature(const_heap)]
#![feature(generic_const_exprs)]

use property::{CssProperty, CssPropertyT, Nil};
use std::marker::PhantomData;
use value::CssValueT;

mod property;

mod value;

#[derive(Debug)]
pub struct Css<Property: CssPropertyT, Value: CssValueT, const LEN: usize>(
    [(CssProperty, value::CssValue); LEN],
    PhantomData<(Property, Value)>,
);

pub const fn css() -> Css<Nil, Nil, 0> {
    Css([], PhantomData)
}
