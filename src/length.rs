use super::{Css, CssValue, CssValueT, Nil};
use std::marker::PhantomData;

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

impl CssValueT for Length {}

impl<const LEN: usize> Css<Nil, Length, LEN> {
    pub(crate) const fn helper(self, len: Length) -> Css<Nil, Nil, LEN> {
        let Self(mut css, _) = self;
        css[LEN - 1].1 = CssValue::Length(len);
        Css(css, PhantomData)
    }
    pub const fn px(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Px(value))
    }
    pub const fn rem(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Rem(value))
    }
    pub const fn cm(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Cm(value))
    }
    pub const fn mm(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Mm(value))
    }
    pub const fn in_(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::In(value))
    }
    pub const fn abs(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Abs(value))
    }
    pub const fn pt(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Pt(value))
    }
    pub const fn pc(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Pc(value))
    }
    pub const fn em(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Em(value))
    }
    pub const fn ex(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Ex(value))
    }
    pub const fn ch(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Ch(value))
    }
    pub const fn vw(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Vw(value))
    }
    pub const fn vh(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Vh(value))
    }
    pub const fn vmin(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Vmin(value))
    }
    pub const fn vmax(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Vmax(value))
    }
    pub const fn percent(self, value: f32) -> Css<Nil, Nil, LEN> {
        self.helper(Length::Percent(value))
    }
}
