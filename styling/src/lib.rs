mod attribute;
mod background;
mod color;
mod length;
mod simple_props;
pub mod svg;

pub type AttributeGetter<T> = Box<dyn FnOnce(T) -> attribute::Attribute>;
pub type Attributes = Vec<attribute::Attribute>;

pub trait StyleState {}
pub trait PreBaseState<T, R: StyleState + Default>: Sized {
    fn destruct(self) -> (Attributes, AttributeGetter<T>);
    fn base(self, position: T) -> Style<R> {
        let (mut core, fun) = self.destruct();
        let attr = fun(position);
        core.push(attr);
        Style(core, R::default())
    }
}

#[derive(Default)]
pub struct StyleBaseState<T>(T);
pub struct Style<T: StyleState>(Attributes, T);

impl StyleState for StyleBaseState<()> {}
impl<T> StyleState for StyleBaseState<AttributeGetter<T>> {}
impl<T> PreBaseState<T, StyleBaseState<()>> for Style<StyleBaseState<AttributeGetter<T>>> {
    fn destruct(self) -> (Attributes, AttributeGetter<T>) {
        let Self(attrs, StyleBaseState(fun)) = self;
        (attrs, fun)
    }
}

impl<T: StyleState> Style<T> {
    fn get_attributes(self) -> Attributes {
        let Self(attrs, _) = self;
        attrs
    }
}

impl Default for Style<StyleBaseState<()>> {
    fn default() -> Self {
        Self(Vec::new(), Default::default())
    }
}

//-----------------------------------------

pub mod experiment {
    use crate::{color::Color, length::Length};

    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub enum Attribute {
        AccentColor(Color),
        FontSize(Length),
    }

    type Buffer = [Option<Attribute>; 100];

    #[derive(Debug)]
    pub struct Styling(Buffer);

    pub const fn styling() -> Styling {
        Styling([None; 100])
    }

    pub struct AccentColorState(Buffer);

    impl AccentColorState {
        const fn does_exist(buffer: &Buffer, index: usize) -> Option<usize> {
            if index < buffer.len() {
                if matches!(buffer[index], Some(Attribute::AccentColor(_))) {
                    return Some(index);
                } else {
                    return Self::does_exist(buffer, index + 1);
                }
            }
            None
        }
    }
    const fn first_none(buffer: &Buffer, index: usize) -> Option<usize> {
        if index < buffer.len() {
            if buffer[index].is_none() {
                return Some(index);
            } else {
                return first_none(buffer, index + 1);
            }
        }
        None
    }

    impl Styling {
        pub const fn accent_color(self) -> AccentColorState {
            let Self(inner) = self;
            AccentColorState(inner)
        }
    }

    impl AccentColorState {
        pub const fn hex(self, hex: u32) -> Styling {
            let Self(mut inner) = self;
            let index = match Self::does_exist(&inner, 0) {
                Some(index) => index,
                None => match first_none(&inner, 0) {
                    Some(index) => index,
                    None => panic!("we ran out of capacity"),
                },
            };
            inner[index] = Some(Attribute::AccentColor(Color::Hex(hex)));
            Styling(inner)
        }
    }
}
