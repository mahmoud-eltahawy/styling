use crate::{AttrValue, Attributer, Home, Styling};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum LengthAttribute {
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

impl Display for LengthAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthAttribute::Cm(num) => write!(f, "{num}cm"),
            LengthAttribute::Mm(num) => write!(f, "{num}mm"),
            LengthAttribute::In(num) => write!(f, "{num}in"),
            LengthAttribute::Px(num) => write!(f, "{num}px"),
            LengthAttribute::Abs(num) => write!(f, "{num}"),
            LengthAttribute::Pt(num) => write!(f, "{num}pt"),
            LengthAttribute::Pc(num) => write!(f, "{num}pc"),
            LengthAttribute::Em(num) => write!(f, "{num}em"),
            LengthAttribute::Ex(num) => write!(f, "{num}ex"),
            LengthAttribute::Ch(num) => write!(f, "{num}ch"),
            LengthAttribute::Rem(num) => write!(f, "{num}rem"),
            LengthAttribute::Vw(num) => write!(f, "{num}vw"),
            LengthAttribute::Vh(num) => write!(f, "{num}vh"),
            LengthAttribute::Vmin(num) => write!(f, "{num}vmin"),
            LengthAttribute::Vmax(num) => write!(f, "{num}vmax"),
            LengthAttribute::Percent(num) => write!(f, "{num}%"),
        }
    }
}

impl<Subject: Attributer<Kind = LengthAttribute>> Styling<Subject> {
    pub fn px(self, len: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Px(
            len,
        ))))
    }
    pub fn abs(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Abs(
            num,
        ))))
    }
    pub fn cm(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Cm(
            num,
        ))))
    }
    pub fn percent(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(
            LengthAttribute::Percent(num),
        )))
    }
    pub fn mm(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Mm(
            num,
        ))))
    }
    pub fn inch(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::In(
            num,
        ))))
    }
    pub fn pt(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Pt(
            num,
        ))))
    }
    pub fn pc(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Pc(
            num,
        ))))
    }
    pub fn em(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Em(
            num,
        ))))
    }
    pub fn ex(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Ex(
            num,
        ))))
    }
    pub fn ch(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Ch(
            num,
        ))))
    }
    pub fn rem_(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Rem(
            num,
        ))))
    }
    pub fn vw(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Vw(
            num,
        ))))
    }
    pub fn vh(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(LengthAttribute::Vh(
            num,
        ))))
    }
    pub fn vmin(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(
            LengthAttribute::Vmin(num),
        )))
    }
    pub fn vmax(self, num: f32) -> Styling<Home> {
        self.add_attr(Subject::attribute(AttrValue::Length(
            LengthAttribute::Vmax(num),
        )))
    }
}
