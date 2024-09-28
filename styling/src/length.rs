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

pub trait LengthAttributer {
    fn length(len: Length) -> Attribute;
}

impl<Subject: LengthAttributer> Styling<Subject> {
    pub fn px(self, len: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Px(len)))
    }
    pub fn abs(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Abs(num)))
    }
    pub fn cm(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Cm(num)))
    }
    pub fn percent(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Percent(num)))
    }
    pub fn mm(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Mm(num)))
    }
    pub fn inch(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::In(num)))
    }
    pub fn pt(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Pt(num)))
    }
    pub fn pc(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Pc(num)))
    }
    pub fn em(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Em(num)))
    }
    pub fn ex(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Ex(num)))
    }
    pub fn ch(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Ch(num)))
    }
    pub fn rem_(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Rem(num)))
    }
    pub fn vw(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Vw(num)))
    }
    pub fn vh(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Vh(num)))
    }
    pub fn vmin(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Vmin(num)))
    }
    pub fn vmax(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::length(Length::Vmax(num)))
    }
}
