use crate::{Attribute, Home, Styling};
use std::fmt::Display;

use std::stringify;

use paste::paste;

pub trait ColorAttributer {
    fn color(color: Color) -> Attribute;
}

macro_rules! color_impl {
    ($($color:ident),+) => {
        paste! {
        impl<Subject : ColorAttributer> Styling<Subject> {
            pub fn hex(self, hex: &str) -> Styling<Home> {
                assert!(hex.len() == 6,"hex str is 6 chars only");
                let [a, b, c, d, e, f] = hex.chars().collect::<Vec<_>>()[..] else {
                    unreachable!();
                };
                self.add_attr(Attribute::AccentColor(Color::Hex([a, b, c, d, e, f])))
            }

            pub fn t_hex(self, hex: &str) -> Styling<Home> {
                assert!(hex.len() == 8,"t_hex str is 8 chars only");
                let [a, b, c, d, e, f, g ,h] = hex.chars().collect::<Vec<_>>()[..] else {
                    unreachable!();
                };
                self.add_attr(Attribute::AccentColor(Color::THex([a, b, c, d, e, f, g ,h])))
            }

            pub fn rgb(self, red: f32, green: f32, blue: f32) -> Styling<Home> {
                self.add_attr(Attribute::AccentColor(Color::Rgb(red, green, blue)))
            }

            pub fn rgba(self, red: f32, green: f32, blue: f32, opacity: f32) -> Styling<Home> {
                self.add_attr(Attribute::AccentColor(Color::Rgba(red, green, blue, opacity)))
            }

            pub fn hsl(self, hue: f32, saturation: f32, lightness: f32) -> Styling<Home> {
                self.add_attr(Attribute::AccentColor(Color::Hsl(hue, saturation, lightness)))
            }

            pub fn hsla(
                self,
                hue: f32,
                saturation: f32,
                lightness: f32,
                opacity: f32,
            ) -> Styling<Home> {
                self.add_attr(Attribute::AccentColor(Color::Hsla(hue, saturation, lightness, opacity)))
            }

            $(
                pub fn [<$color:snake>](self) -> Styling<Home> {
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
            Hex([char;6]),
            THex([char;8]),
            Rgb(f32, f32, f32),
            Rgba(f32, f32, f32, f32),
            Hsl(f32, f32, f32),
            Hsla(f32, f32, f32, f32)
            $(,$color)*
        }
        impl Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = match self {
                    Color::Hex(c) => {
                        let result = c.map(|x| x.to_string()).join("");
                        format!("#{result}")
                    }
                    Color::THex(c) => {
                        let result = c.map(|x| x.to_string()).join("");
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
