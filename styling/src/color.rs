use crate::{Attribute, Home, Styling};
use std::fmt::Display;

use std::stringify;

use paste::paste;

macro_rules! color_impl {
    ($($color:ident),+) => {
        paste! {
        pub struct AccentColor;

        impl<const SIZE: usize> Styling<AccentColor, SIZE> {
            pub const fn hex(self, hex: u32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::Hex(hex)))
            }

            pub const fn t_hex(self, hex: u32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::THex(hex)))
            }

            pub const fn rgb(self, red: f32, green: f32, blue: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::Rgb(red, green, blue)))
            }

            pub const fn rgba(self, red: f32, green: f32, blue: f32, opacity: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::Rgba(red, green, blue, opacity)))
            }

            pub const fn hsl(self, hue: u16, saturation: f32, lightness: f32) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::Hsl(hue, saturation, lightness)))
            }

            pub const fn hsla(
                self,
                hue: u16,
                saturation: f32,
                lightness: f32,
                opacity: f32,
            ) -> Styling<Home, SIZE> {
                self.add_attr(Attribute::AccentColor(Color::Hsla(hue, saturation, lightness, opacity)))
            }

            $(
                pub const fn [<$color:snake>](self) -> Styling<Home, SIZE> {
                    self.add_attr(Attribute::AccentColor(Color::[<$color>]))
                }
            )*
        }
        }
    };
}

macro_rules! color_define {
    ($($color:ident),+) => {

        #[derive(Debug, Clone, Copy)]
        pub enum Color {
            Hex(u32),
            THex(u32),
            Rgb(f32, f32, f32),
            Rgba(f32, f32, f32, f32),
            Hsl(u16, f32, f32),
            Hsla(u16, f32, f32, f32)
            $(,$color)*
        }
        impl Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = match self {
                    Color::Hex(c) => {
                        let result = format!("{c:#06x}")[2..].to_string();
                        format!("#{result}")
                    }
                    Color::THex(c) => {
                        let result = format!("{c:#08x}")[2..].to_string();
                        format!("#{result}")
                    }
                    Color::Rgb(red, green, blue) => format!("rgb({red},{green},{blue})"),
                    Color::Rgba(red, green, blue, opacity) => {
                        format!("rgba({red},{green},{blue},{})", *opacity as f32 / 100.)
                    }
                    Color::Hsl(hue, saturation, lightness) => {
                        format!("hsl({hue},{saturation}%,{lightness}%)")
                    }
                    Color::Hsla(hue, saturation, lightness, opacity) => {
                        format!(
                            "hsl({hue},{saturation}%,{lightness}%,{})",
                            *opacity as f32 / 100.
                        )
                    }
                    $(
                        Color::$color => stringify!($color).to_string(),
                    )*
                };
                write!(f, "{}", result)
            }
        }
        color_impl!($($color),*);
    };
}

color_define!(
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Grey,
    Green,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen
);
