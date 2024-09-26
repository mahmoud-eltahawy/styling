use crate::{Attribute, Home, Styling};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Length {
    //absolute
    Cm(f32),
    Mm(f32),
    In(f32),
    Px(f32),
    Abs(f32),
    Pt(f32),
    Pc(f32),
    //relative
    Em(f32),
    Ex(f32),
    Ch(f32),
    Rem(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
    Percent(f32),
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Cm(num) => write!(f, "{num}cm"),
            Length::Mm(num) => write!(f, "{num}mm"),
            Length::In(num) => write!(f, "{num}in"),
            Length::Px(num) => write!(f, "{num}px"),
            Length::Abs(num) => write!(f, "{num}"),
            Length::Pt(num) => write!(f, "{num}pt"),
            Length::Pc(num) => write!(f, "{num}pc"),
            Length::Em(num) => write!(f, "{num}em"),
            Length::Ex(num) => write!(f, "{num}ex"),
            Length::Ch(num) => write!(f, "{num}ch"),
            Length::Rem(num) => write!(f, "{num}rem"),
            Length::Vw(num) => write!(f, "{num}vw"),
            Length::Vh(num) => write!(f, "{num}vh"),
            Length::Vmin(num) => write!(f, "{num}vmin"),
            Length::Vmax(num) => write!(f, "{num}vmax"),
            Length::Percent(num) => write!(f, "{num}%"),
        }
    }
}

macro_rules! style_impl {
    ($target:ident) => {
        pub struct $target;

        impl<const SIZE: usize> Styling<$target, SIZE> {
            pub const fn px(self, len: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Px(len)))
            }

            pub const fn abs(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Abs(num)))
            }

            pub const fn cm(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Cm(num)))
            }

            pub const fn percent(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Percent(num)))
            }

            pub const fn mm(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Mm(num)))
            }
            pub const fn inch(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::In(num)))
            }
            pub const fn pt(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Pt(num)))
            }
            pub const fn pc(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Pc(num)))
            }
            pub const fn em(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Em(num)))
            }
            pub const fn ex(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Ex(num)))
            }
            pub const fn ch(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Ch(num)))
            }
            pub const fn rem_(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Rem(num)))
            }
            pub const fn vw(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Vw(num)))
            }
            pub const fn vh(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Vh(num)))
            }
            pub const fn vmin(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Vmin(num)))
            }
            pub const fn vmax(self, num: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::$target(Length::Vmax(num)))
            }
        }
    };
}

style_impl!(FontSize);
style_impl!(Margin);
