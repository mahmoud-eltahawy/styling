use super::parsing;
use crate::NameCases;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn transpile(names: Vec<parsing::Name>) -> TokenStream {
    let pascal_colors = pascal_colors(&names);
    let colors_display = colors_display(&names);
    let funs = funs(names);

    let fixed_colors = fixed_colors();
    let fixed_display = fixed_display();
    let fixed_funs = fixed_funs();

    quote! {
        #[derive(Debug, Clone, Copy)]
        pub enum ColorAttribute {
            #fixed_colors
            #pascal_colors
        }

        impl std::fmt::Display for ColorAttribute {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = match self {
                    #fixed_display
                    #colors_display
                };
                write!(f, "{}", result)
            }
        }

        impl<Subject : Attributer<Kind = ColorAttribute>> Styling<Subject> {
            #fixed_funs
            #funs
        }
    }
}

fn fixed_funs() -> TokenStream {
    quote! {
        pub fn hex(self, hex: &str) -> Styling<Home> {
            assert!(hex.len() == 6,"hex str is 6 chars only");
            let [a, b, c, d, e, f] = hex.chars().collect::<Vec<_>>()[..] else {
                unreachable!();
            };
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::Hex([a, b, c, d, e, f]))))
        }

        pub fn t_hex(self, hex: &str) -> Styling<Home> {
            assert!(hex.len() == 8,"t_hex str is 8 chars only");
            let [a, b, c, d, e, f, g ,h] = hex.chars().collect::<Vec<_>>()[..] else {
                unreachable!();
            };
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::THex([a, b, c, d, e, f, g ,h]))))
        }

        pub fn rgb(self, red: f32, green: f32, blue: f32) -> Styling<Home> {
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::Rgb(red, green, blue))))
        }

        pub fn rgba(self, red: f32, green: f32, blue: f32, opacity: f32) -> Styling<Home> {
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::Rgba(red, green, blue, opacity))))
        }

        pub fn hsl(self, hue: f32, saturation: f32, lightness: f32) -> Styling<Home> {
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::Hsl(hue, saturation, lightness))))
        }

        pub fn hsla(
            self,
            hue: f32,
            saturation: f32,
            lightness: f32,
            opacity: f32,
        ) -> Styling<Home> {
            self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::Hsla(hue, saturation, lightness, opacity))))
        }
    }
}

fn fixed_display() -> TokenStream {
    let fixed_display = quote! {
        ColorAttribute::Hex(c) => {
            let result = c.map(|x| x.to_string()).join("");
            format!("#{result}")
        }
        ColorAttribute::THex(c) => {
            let result = c.map(|x| x.to_string()).join("");
            format!("#{result}")
        }
        ColorAttribute::Rgb(red, green, blue) => format!("rgb({red},{green},{blue})"),
        ColorAttribute::Rgba(red, green, blue, opacity) => {
            format!("rgba({red},{green},{blue},{})", *opacity / 100.)
        }
        ColorAttribute::Hsl(hue, saturation, lightness) => {
            format!("hsl({hue},{saturation}%,{lightness}%)")
        }
        ColorAttribute::Hsla(hue, saturation, lightness, opacity) => {
            format!(
                "hsl({hue},{saturation}%,{lightness}%,{})",
                *opacity / 100.
            )
        }
    };
    fixed_display
}

fn fixed_colors() -> TokenStream {
    let fixed_colors = quote! {
        Hex([char;6]),
        THex([char;8]),
        Rgb(f32, f32, f32),
        Rgba(f32, f32, f32, f32),
        Hsl(f32, f32, f32),
        Hsla(f32, f32, f32, f32),
    };
    fixed_colors
}

fn funs(names: Vec<parsing::Name>) -> TokenStream {
    let funs = names.iter().fold(TokenStream::new(), |mut acc, x| {
        let snake = &x.0;
        let pascal = x.0.pascal_ident();
        acc.extend(quote! {
             pub fn #snake(self) -> Styling<Home> {
                 self.add_attr(Subject::attribute(AttrValue::Custom(ColorAttribute::#pascal)))
             }
        });
        acc
    });
    funs
}

fn colors_display(names: &[parsing::Name]) -> TokenStream {
    let colors_display = names.iter().fold(TokenStream::new(), |mut acc, x| {
        let pascal_ident = x.0.pascal_ident();
        let pascal = x.0.pascal();
        acc.extend(quote! {
            ColorAttribute::#pascal_ident => #pascal.to_string(),
        });
        acc
    });
    colors_display
}

fn pascal_colors(names: &[parsing::Name]) -> TokenStream {
    let pascal_colors =
        names
            .iter()
            .map(|x| x.0.pascal_ident())
            .fold(TokenStream::new(), |mut acc, x| {
                acc.extend(quote! {
                    #x,
                });
                acc
            });
    pascal_colors
}
