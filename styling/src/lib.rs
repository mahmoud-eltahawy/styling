mod attribute;
// mod background;
mod color;
mod length;
mod simple_props;
// pub mod svg;

use attribute::Attribute;
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug)]
pub struct Styling<T>(Vec<Attribute>, PhantomData<T>);

pub fn styling() -> Styling<Home> {
    Styling(Vec::new(), PhantomData)
}

impl<T> Styling<T> {
    fn transform<S>(self) -> Styling<S> {
        let Self(inner, _) = self;
        Styling(inner, PhantomData)
    }

    fn add_attr(self, attr: Attribute) -> Styling<Home> {
        let Self(mut inner, _) = self;
        match inner
            .iter()
            .enumerate()
            .find(|(_, x)| x.eq(&attr))
            .map(|(i, _)| i)
        {
            Some(index) => {
                inner[index] = attr;
            }
            None => inner.push(attr),
        };
        Styling(inner, PhantomData)
    }
}

pub struct Home;
pub struct FontSize;
pub struct Margin;
pub struct Top;
pub struct Bottom;
pub struct Left;
pub struct Right;

pub struct AccentColor;

impl color::ColorAttributer for AccentColor {
    fn color(color: color::Color) -> Attribute {
        Attribute::AccentColor(color)
    }
}

impl length::LengthAttributer for FontSize {
    fn length(len: length::Length) -> Attribute {
        Attribute::FontSize(len)
    }
}

impl length::LengthAttributer for Margin {
    fn length(len: length::Length) -> Attribute {
        Attribute::Margin(len)
    }
}

impl length::LengthAttributer for Top {
    fn length(len: length::Length) -> Attribute {
        Attribute::Top(len)
    }
}

impl length::LengthAttributer for Bottom {
    fn length(len: length::Length) -> Attribute {
        Attribute::Bottom(len)
    }
}

impl length::LengthAttributer for Right {
    fn length(len: length::Length) -> Attribute {
        Attribute::Right(len)
    }
}

impl length::LengthAttributer for Left {
    fn length(len: length::Length) -> Attribute {
        Attribute::Left(len)
    }
}

impl Styling<Home> {
    pub fn extend(self, other: Styling<Home>) -> Styling<Home> {
        let Styling(other, _) = other;
        let mut base = self;
        for x in other.into_iter() {
            base = base.add_attr(x);
        }
        base
    }

    pub fn accent_color(self) -> Styling<AccentColor> {
        self.transform()
    }
    pub fn font_size(self) -> Styling<FontSize> {
        self.transform()
    }
    pub fn margin(self) -> Styling<Margin> {
        self.transform()
    }
    pub fn top(self) -> Styling<Top> {
        self.transform()
    }
    pub fn bottom(self) -> Styling<Bottom> {
        self.transform()
    }
    pub fn left(self) -> Styling<Left> {
        self.transform()
    }
    pub fn right(self) -> Styling<Right> {
        self.transform()
    }
}

impl Display for Styling<Home> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string());
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::styling;
    #[test]
    fn multiple_props() {
        let styling1 = styling()
            .margin()
            .px(4.)
            .font_size()
            .px(16.)
            .accent_color()
            .rgb(100., 100., 100.);
        let expected = "margin:4px;font-size:16px;accent-color:rgb(100,100,100);";
        assert_eq!(String::from(expected), styling1.to_string());
        //test cascading
        let styling2 = styling()
            .accent_color()
            .red()
            .font_size()
            .px(10.)
            .accent_color()
            .blue();
        assert_eq!(
            styling2.to_string(),
            String::from("accent-color:Blue;font-size:10px;")
        );

        //test cascading merging
        let styling3 = styling1.extend(styling2);
        let expected = "margin:4px;font-size:10px;accent-color:Blue;";
        assert_eq!(String::from(expected), styling3.to_string());
    }
}
