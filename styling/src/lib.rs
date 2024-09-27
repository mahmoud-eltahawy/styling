mod attribute;
// mod background;
mod color;
mod length;
// mod simple_props;
// pub mod svg;

use std::{fmt::Display, marker::PhantomData};

use attribute::Attribute;
use color::AccentColor;
use length::{FontSize, Margin};

#[derive(Debug)]
pub struct Styling<T, const SIZE: usize>([Attribute; SIZE], PhantomData<T>);

pub const fn styling<const SIZE: usize>() -> Styling<Home, SIZE> {
    Styling([Attribute::None; SIZE], PhantomData)
}

pub struct Home;

#[macro_export]
macro_rules! merge {
    ($base:ident,$other:ident) => {{
        const OTHER_SIZE: usize = $other.capacity();
        const SIZE_SUM: usize = OTHER_SIZE + $base.capacity();
        $base.merge::<OTHER_SIZE, SIZE_SUM>($other)
    }};
}

impl<T, const SIZE: usize> Styling<T, SIZE> {
    pub const fn merge<const OTHER_SIZE: usize, const SUM_SIZE: usize>(
        self,
        other: Styling<Home, OTHER_SIZE>,
    ) -> Styling<Home, SUM_SIZE> {
        let Self(arr1, _) = self;
        let Styling(arr2, _) = other;
        let mut result = [Attribute::None; SUM_SIZE];
        assert!(
            result.len() == arr1.len() + arr2.len(),
            "make sure that new Styling capacity is the sum of the two merged values"
        );
        let mut i = 0;
        while i < arr1.len() {
            match arr1[i] {
                Attribute::None => {
                    break;
                }
                val => {
                    result[i] = val;
                }
            }
            i += 1;
        }
        let mut result = Styling(result, PhantomData);
        let mut i = 0;
        while i < arr2.len() {
            match arr2[i] {
                Attribute::None => {
                    break;
                }
                val => {
                    result = result.add_attr(val);
                }
            }
            i += 1;
        }
        result
    }

    pub const fn capacity(&self) -> usize {
        self.0.len()
    }

    pub const fn size(&self) -> usize {
        let mut result = 0;
        while result < self.0.len() && !matches!(self.0[result], Attribute::None) {
            result += 1;
        }
        result
    }

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

    const fn does_exist(&self, other: &Attribute) -> Option<usize> {
        let mut index = 0;
        while index < self.0.len() {
            if self.0[index].eq(other) {
                return Some(index);
            };
            index += 1
        }
        None
    }

    const fn first_none(&self) -> usize {
        let buffer = self.0;
        let mut index = 0;
        while index < buffer.len() {
            if let Attribute::None = buffer[index] {
                return index;
            }
            index += 1;
        }
        panic!("not enough capacity");
    }

    const fn target_index(&self, attr: &Attribute) -> usize {
        match self.does_exist(attr) {
            Some(index) => index,
            None => self.first_none(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{styling, Home, Styling};
    #[test]
    fn multiple_props() {
        const STYLING1: Styling<Home, 3> = styling()
            .margin()
            .px(4.)
            .font_size()
            .px(16.)
            .accent_color()
            .rgb(100., 100., 100.);
        let expected = "margin:4px;font-size:16px;accent-color:rgb(100,100,100);";
        assert_eq!(String::from(expected), STYLING1.to_string());
        //test cascading
        const STYLING2: Styling<Home, 3> = styling()
            .accent_color()
            .red()
            .accent_color()
            .green()
            .font_size()
            .px(10.)
            .accent_color()
            .blue();
        assert_eq!(
            STYLING2.to_string(),
            String::from("accent-color:Blue;font-size:10px;")
        );
        //test merging

        const STYLING3: Styling<Home, 6> = merge!(STYLING1, STYLING2);
        let expected = "margin:4px;font-size:10px;accent-color:Blue;";
        assert_eq!(String::from(expected), STYLING3.to_string());
    }
}
