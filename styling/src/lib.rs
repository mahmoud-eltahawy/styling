// mod attribute;
// mod background;
mod color;
// mod length;
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
pub struct FontSize;

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
}

// impl<const SIZE: usize> Styling<AccentColor, SIZE> {}

impl<const SIZE: usize> Styling<FontSize, SIZE> {
    pub const fn px(self, len: f32) -> Styling<Home, SIZE> {
        self.add_attr(Attribute::FontSize(Length::Px(len)))
    }
}

//------------ attibutes -----------------

use std::{fmt::Display, marker::PhantomData};

use color::{AccentColor, Color};

// #[derive(Debug, Clone, Copy)]
// pub enum Color {
//     Hex(u32),
//     THex(u32),
//     Rgb(f32, f32, f32),
//     Rgba(f32, f32, f32, f32),
//     Hsl(u16, f32, f32),
//     Hsla(u16, f32, f32, f32),
// }

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

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    AccentColor(Color),
    FontSize(Length),
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
            Attribute::None => "".to_string(),
        };
        write!(f, "{}", result)
    }
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

// impl Display for Color {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let result = match self {
//             Color::Hex(c) => {
//                 let result = format!("{c:#08x}")[2..].to_string();
//                 format!("#{result}")
//             }
//             Color::THex(c) => {
//                 let result = format!("{c:#08x}")[2..].to_string();
//                 format!("#{result}")
//             }
//             Color::Rgb(red, green, blue) => format!("rgb({red},{green},{blue})"),
//             Color::Rgba(red, green, blue, opacity) => {
//                 format!("rgba({red},{green},{blue},{})", opacity)
//             }
//             Color::Hsl(hue, saturation, lightness) => {
//                 format!("hsl({hue},{saturation}%,{lightness}%)")
//             }
//             Color::Hsla(hue, saturation, lightness, opacity) => {
//                 format!("hsl({hue},{saturation}%,{lightness}%,{})", opacity)
//             }
//         };
//         write!(f, "{}", result)
//     }
// }
