// mod attribute;
// mod background;
// mod color;
// mod length;
// mod simple_props;
// pub mod svg;

//----------stage definition---------------
//-------Styling type-----

#[derive(Debug)]
pub struct Styling<T>([Option<Attribute>; 100], PhantomData<T>);

pub const fn styling() -> Styling<Home> {
    Styling([None; 100], PhantomData)
}
//------------------------
pub struct Home;
pub struct AccentColor;
pub struct FontSize;

impl<T> Styling<T> {
    const fn transform<S>(self) -> Styling<S> {
        let Self(inner, _) = self;
        Styling(inner, PhantomData)
    }
    const fn add_attr(self, attr: Attribute) -> Styling<Home> {
        let index = self.target_index(&attr);
        let Self(mut inner, _) = self;
        inner[index] = Some(attr);
        Styling(inner, PhantomData)
    }

    const fn does_exist(&self, other: &Attribute, index: usize) -> Option<usize> {
        let buffer = self.0;
        if index < buffer.len() {
            match buffer[index] {
                Some(x) if x.eq(other) => {
                    return Some(index);
                }
                _ => {
                    return self.does_exist(other, index + 1);
                }
            }
        }
        None
    }

    const fn first_none(&self, index: usize) -> Option<usize> {
        let buffer = self.0;
        if index < buffer.len() {
            if buffer[index].is_none() {
                return Some(index);
            } else {
                return self.first_none(index + 1);
            }
        }
        None
    }

    const fn target_index(&self, other: &Attribute) -> usize {
        match self.does_exist(other, 0) {
            Some(index) => index,
            None => {
                //
                match self.first_none(0) {
                    Some(index) => index,
                    None => panic!("we ran out of capacity"),
                }
            }
        }
    }

    const fn size_inner(&self, index: usize) -> usize {
        match self.0[index] {
            Some(_) => self.size_inner(index + 1),
            None => index,
        }
    }

    pub const fn size(&self) -> usize {
        self.size_inner(0)
    }
}
//---------------stage functions ---------------

impl Styling<Home> {
    pub const fn accent_color(self) -> Styling<AccentColor> {
        self.transform()
    }
    pub const fn font_size(self) -> Styling<FontSize> {
        self.transform()
    }
}

impl Styling<AccentColor> {
    pub const fn hex(self, hex: u32) -> Styling<Home> {
        self.add_attr(Attribute::AccentColor(Color::Hex(hex)))
    }
}

impl Styling<FontSize> {
    pub const fn px(self, len: f32) -> Styling<Home> {
        self.add_attr(Attribute::FontSize(Length::Px(len)))
    }
}

//------------ attibutes -----------------

use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Hex(u32),
    THex(u32),
    Rgb(f32, f32, f32),
    Rgba(f32, f32, f32, f32),
    Hsl(u16, f32, f32),
    Hsla(u16, f32, f32, f32),
}

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

impl Display for Styling<Home> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .flatten()
            .fold(String::new(), |acc, x| acc + &x.to_string());
        write!(f, "{}", result)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attribute::AccentColor(x) => format!("accent-color:{x};"),
            Attribute::FontSize(x) => format!("font-size:{x};"),
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Color::Hex(c) => {
                let result = format!("{c:#08x}")[2..].to_string();
                format!("#{result}")
            }
            Color::THex(c) => {
                let result = format!("{c:#08x}")[2..].to_string();
                format!("#{result}")
            }
            Color::Rgb(red, green, blue) => format!("rgb({red},{green},{blue})"),
            Color::Rgba(red, green, blue, opacity) => {
                format!("rgba({red},{green},{blue},{})", opacity)
            }
            Color::Hsl(hue, saturation, lightness) => {
                format!("hsl({hue},{saturation}%,{lightness}%)")
            }
            Color::Hsla(hue, saturation, lightness, opacity) => {
                format!("hsl({hue},{saturation}%,{lightness}%,{})", opacity)
            }
        };
        write!(f, "{}", result)
    }
}
