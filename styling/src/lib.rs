// mod attribute;
// mod background;
mod color;
mod length;
// mod simple_props;
// pub mod svg;

//----------stage definition---------------
//-------Styling type-----

#[derive(Debug)]
pub struct Styling<T, const SIZE: usize>([Attribute; SIZE], PhantomData<T>);

pub const fn styling<const SIZE: usize>() -> Styling<Home, SIZE> {
    Styling([Attribute::None; SIZE], PhantomData)
}
//------------------------
pub struct Home;

impl<T, const SIZE: usize> Styling<T, SIZE> {
    const fn transform<S>(self) -> Styling<S, SIZE> {
        let Self(inner, _) = self;
        Styling(inner, PhantomData)
    }
    const fn add_attr(self, attr: Attribute) -> Styling<Home, SIZE> {
        let index = self.target_index(&attr);
        let Self(mut inner, _) = self;
        inner[index] = attr;
        Styling(inner, PhantomData)
    }

    const fn does_exist(&self, other: &Attribute, index: usize) -> Option<usize> {
        if index >= self.0.len() {
            return None;
        };
        if self.0[index].eq(other) {
            Some(index)
        } else {
            self.does_exist(other, index + 1)
        }
    }

    const fn first_none(&self, index: usize) -> usize {
        let buffer = self.0;
        assert!(index < buffer.len(), "low capacity. consider increasing it");
        match buffer[index] {
            Attribute::None => index,
            _ => self.first_none(index + 1),
        }
    }

    const fn target_index(&self, other: &Attribute) -> usize {
        match self.does_exist(other, 0) {
            Some(index) => index,
            None => self.first_none(0),
        }
    }

    const fn size_inner(&self, index: usize) -> usize {
        match self.0[index] {
            Attribute::None => index,
            _ => self.size_inner(index + 1),
        }
    }

    pub const fn size(&self) -> usize {
        self.size_inner(0)
    }
}
//---------------stage functions ---------------

impl<const SIZE: usize> Styling<Home, SIZE> {
    pub const fn accent_color(self) -> Styling<AccentColor, SIZE> {
        self.transform()
    }
    pub const fn font_size(self) -> Styling<FontSize, SIZE> {
        self.transform()
    }
    pub const fn margin(self) -> Styling<Margin, SIZE> {
        self.transform()
    }
}

//------------ attibutes -----------------

use std::{fmt::Display, marker::PhantomData};

use color::{AccentColor, Color};
use length::{FontSize, Length, Margin};

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    AccentColor(Color),
    FontSize(Length),
    Margin(Length),
    None,
}

impl Attribute {
    const fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Attribute::AccentColor(_), Attribute::AccentColor(_))
                | (Attribute::FontSize(_), Attribute::FontSize(_))
        )
    }
}

//--------- display ----------

impl<const SIZE: usize> Display for Styling<Home, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .filter(|x| !matches!(x, Attribute::None))
            .fold(String::new(), |acc, x| acc + &x.to_string());
        write!(f, "{}", result)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attribute::AccentColor(x) => format!("accent-color:{x};"),
            Attribute::FontSize(x) => format!("font-size:{x};"),
            Attribute::Margin(x) => format!("margin:{x};"),
            Attribute::None => "".to_string(),
        };
        write!(f, "{}", result)
    }
}
