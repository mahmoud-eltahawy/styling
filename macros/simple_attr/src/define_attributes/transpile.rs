use proc_macro2::TokenStream;
use quote::quote;

use crate::{define_attributes::parsing::StraightLine, NameCases};

pub(crate) fn transpile(lines: Vec<StraightLine>) -> TokenStream {
    let mut tokens = TokenStream::new();

    let main_attributes_types = main_attributes(&lines);
    let transformers = transformers(&lines);
    let varients_types = define_varients_types(&lines);
    let varients_display = display_varients_types(&lines);
    let varients_funs = simple_varients_funs(&lines);

    tokens.extend(quote!(
        #main_attributes_types
        #varients_types
        #varients_funs
        #varients_display
        #transformers
    ));
    tokens
}

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

fn simple_varients_funs(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let pascal_header = x.header.atoms.pascal_ident();

        let funs = x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let snake = x.atoms.snake_ident();
            let pascal = x.atoms.pascal_ident();
            acc.extend(quote! {
                pub fn #snake(self) -> Styling<Home> {
                    self.add_attr(Attribute::#pascal_header(#pascal_header::#pascal))
                }
            });
            acc
        });
        acc.extend(quote! {
            impl Styling<#pascal_header> {
                #funs
            }
        });
        acc
    })
}

fn transformers(lines: &[StraightLine]) -> TokenStream {
    let result = lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let name_docs = x
            .header
            .docs
            .as_ref()
            .map(|x| format!("# {x}"))
            .unwrap_or(String::from("# no description found"));
        let pascal = x.header.atoms.pascal_ident();
        let snake = x.header.atoms.snake_ident();
        let props_docs = x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let result = format!("- {}", x.atoms.snake());
            acc.extend(quote! {
                #[doc = #result]
            });
            acc
        });
        acc.extend(quote!(
            #[doc = #name_docs]
            #[doc = "## possible values"]
            #props_docs
            pub fn #snake(self) -> Styling<#pascal> {
                self.transform()
            }
        ));
        acc
    });
    quote! {
        impl Styling<Home> {
            #result
        }
    }
}

fn define_varients_types(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let header_pascal = line.header.atoms.pascal_ident();
        let varients_pascal = line
            .attrs
            .iter()
            .fold(TokenStream::new(), |mut acc, varient| {
                let pascal = varient.atoms.pascal_ident();
                acc.extend(quote! {
                    #pascal,
                });
                acc
            });
        acc.extend(quote!(
            #[derive(Debug, Clone)]
            pub enum #header_pascal {
                #varients_pascal
            }
        ));
        acc
    })
}

fn display_varients_types(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let header_pascal = line.header.atoms.pascal_ident();
        let varients_display = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.atoms.pascal_ident();
            let untrailed_kebab = clear_trailing_dash(x.kebab());
            acc.extend(quote!(
                Self::#pascal => #untrailed_kebab,
            ));
            acc
        });

        acc.extend(quote!(
            impl std::fmt::Display for #header_pascal {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        #varients_display
                    };
                    write!(f, "{}",result)
                }
            }
        ));
        acc
    })
}

fn main_attributes(lines: &[StraightLine]) -> TokenStream {
    let simple_ones = lines.iter().map(|x| x.header.atoms.pascal_ident()).fold(
        TokenStream::new(),
        |mut acc, x| {
            acc.extend(quote! {
                #x(#x),
            });
            acc
        },
    );
    let eq_attrs = lines
        .iter()
        .map(|x| x.header.atoms.pascal_ident())
        .enumerate()
        .fold(TokenStream::new(), |mut acc, (i, x)| {
            let i = i + 12;
            acc.extend(quote! {
                #x(_) => #i,
            });
            acc
        });
    let attrs_display = lines
        .iter()
        .map(|x| &x.header)
        .fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.atoms.pascal_ident();
            let kebab = x.kebab();
            acc.extend(quote! {
                #pascal(x) => format!("{}:{};",#kebab,x),
            });
            acc
        });
    quote! {
        #[derive(Debug, Clone)]
        pub enum Attribute {
            AccentColor(Color),
            FontSize(Length),
            Margin(Length),
            Top(Length),
            Bottom(Length),
            Right(Length),
            Left(Length),
            Height(Length),
            Width(Length),
            Padding(Length),
            BackgroundColor(Color),
            BackgroundImage(String),
            #simple_ones
        }

        impl Attribute {
            fn repr(&self) -> usize {
                use Attribute::*;
                match self {
                    AccentColor(_) => 0,
                    FontSize(_) => 1,
                    Margin(_) => 2,
                    Top(_) => 3,
                    Bottom(_) => 4,
                    Right(_) => 5,
                    Left(_) => 6,
                    Height(_) => 7,
                    Width(_) => 8,
                    Padding(_) => 9,
                    BackgroundColor(_) => 10,
                    BackgroundImage(_) => 11,
                    #eq_attrs
                }
            }

            pub fn eq(&self, other: &Self) -> bool {
                self.repr() == other.repr()
            }
        }

        impl std::fmt::Display for Attribute {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use Attribute::*;
                let result = match self {
                    AccentColor(x) => format!("accent-color:{x};"),
                    FontSize(x) => format!("font-size:{x};"),
                    Top(x) => format!("top:{x};"),
                    Bottom(x) => format!("bottom:{x};"),
                    Right(x) => format!("right:{x};"),
                    Left(x) => format!("left:{x};"),
                    Height(x) => format!("height:{x};"),
                    Width(x) => format!("width:{x};"),
                    Margin(x) => format!("margin:{x};"),
                    Padding(x) => format!("padding:{x};"),
                    BackgroundColor(x) => {
                        format!("background-color:{x};")
                    }
                    BackgroundImage(x) => format!("background-image:url({x});"),
                    #attrs_display
                };
                write!(f, "{}", result)
            }
        }

    }
}
