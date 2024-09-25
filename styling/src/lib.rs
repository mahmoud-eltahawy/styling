// mod attribute;
// mod background;
// mod color;
// mod length;
// mod simple_props;
// pub mod svg;

// pub type AttributeGetter<T> = Box<dyn FnOnce(T) -> attribute::Attribute>;
// pub type Attributes = Vec<attribute::Attribute>;

// pub trait StyleState {}
// pub trait PreBaseState<T, R: StyleState + Default>: Sized {
//     fn destruct(self) -> (Attributes, AttributeGetter<T>);
//     fn base(self, position: T) -> Style<R> {
//         let (mut core, fun) = self.destruct();
//         let attr = fun(position);
//         core.push(attr);
//         Style(core, R::default())
//     }
// }

// #[derive(Default)]
// pub struct StyleBaseState<T>(T);
// pub struct Style<T: StyleState>(Attributes, T);

// impl StyleState for StyleBaseState<()> {}
// impl<T> StyleState for StyleBaseState<AttributeGetter<T>> {}
// impl<T> PreBaseState<T, StyleBaseState<()>> for Style<StyleBaseState<AttributeGetter<T>>> {
//     fn destruct(self) -> (Attributes, AttributeGetter<T>) {
//         let Self(attrs, StyleBaseState(fun)) = self;
//         (attrs, fun)
//     }
// }

// impl<T: StyleState> Style<T> {
//     fn get_attributes(self) -> Attributes {
//         let Self(attrs, _) = self;
//         attrs
//     }
// }

// impl Default for Style<StyleBaseState<()>> {
//     fn default() -> Self {
//         Self(Vec::new(), Default::default())
//     }
// }

//-----------------------------------------

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Hex(f32),
    THex(f32),
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

#[derive(Debug)]
pub struct Styling<T>([Option<Attribute>; 100], PhantomData<T>);
pub struct Home;

pub const fn styling() -> Styling<Home> {
    Styling([None; 100], PhantomData {})
}

impl<T> Styling<T> {
    const fn does_exist(&self, index: usize) -> Option<usize> {
        let buffer = self.0;
        if index < buffer.len() {
            if matches!(buffer[index], Some(Attribute::AccentColor(_))) {
                return Some(index);
            } else {
                return self.does_exist(index + 1);
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

    const fn target_index(&self) -> usize {
        match self.does_exist(0) {
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
}

pub struct AccentColor;

impl Styling<Home> {
    pub const fn accent_color(self) -> Styling<AccentColor> {
        let Self(inner, _) = self;
        Styling(inner, PhantomData {})
    }
}

impl Styling<AccentColor> {
    pub const fn hex(self, hex: f32) -> Styling<Home> {
        let index = self.target_index();
        let Self(mut inner, _) = self;
        inner[index] = Some(Attribute::AccentColor(Color::Hex(hex)));
        Styling(inner, PhantomData)
    }
}
