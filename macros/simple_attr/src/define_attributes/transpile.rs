use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{define_attributes::parsing::AttrGroup, NameCases};

use super::parsing::Line;

pub(crate) fn transpile(lines: Vec<Line>) -> TokenStream {
    let mut tokens = TokenStream::new();

    let attribute = define_attribute(&lines);
    let transformers = transformers(&lines);
    let varients_types = define_varients_types(&lines);
    let varients_funs = simple_varients_funs(&lines);
    let varients_display = display_varients_types(&lines);

    tokens.extend(quote!(
        #attribute
        #transformers
        #varients_types
        #varients_display
        #varients_funs
    ));
    tokens
}

fn simple_varients_funs(lines: &[Line]) -> TokenStream {
    lines
        .iter()
        .fold(TokenStream::new(), |mut acc, x| {
            if x.group.is_some() {
                //TODO : remove this
                return acc;
            }
            for header in x.headers() {
                let pascal_header = header.snake_ident.pascal_ident();

                let funs = x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
                    let snake = &x.snake_ident;
                    let pascal = x.snake_ident.pascal_ident();
                    acc.extend(quote! {
                        pub fn #snake(self) -> Styling<Home> {
                            self.add_attr(Attribute::#pascal_header(AttrValue::Custom(#pascal_header::#pascal)))
                        }
                    });
                    acc
                });
                acc.extend(quote! {
                    impl Styling<#pascal_header> {
                        #funs
                    }
                });
            }
            acc
        })
}

//TODO : to be refactored
fn define_attribute(lines: &[Line]) -> TokenStream {
    let simple_ones = lines.iter().fold(TokenStream::new(), |mut acc, x| {
        for header in x.headers() {
            let header = header.snake_ident.pascal_ident();
            if let Some(group) = &x.group {
                let group = match group {
                    AttrGroup::Color => format_ident!("ColorAttribute"),
                    AttrGroup::Length => format_ident!("LengthAttribute"),
                };
                acc.extend(quote! {#header(AttrValue<#group>),});
            } else {
                acc.extend(quote! {#header(AttrValue<#header>),});
            }
        }
        acc
    });
    let eq_attrs = lines.iter().flat_map(|x| x.headers()).enumerate().fold(
        TokenStream::new(),
        |mut acc, (index, name)| {
            let name_pascal = name.snake_ident.pascal_ident();
            let index = index as u16;
            acc.extend(quote! {#name_pascal(_) => #index,});
            acc
        },
    );
    let attrs_display =
        lines
            .iter()
            .flat_map(|x| x.headers())
            .fold(TokenStream::new(), |mut acc, x| {
                let pascal = x.snake_ident.pascal_ident();
                let kebab = x.kebab();
                acc.extend(quote! {#pascal(x) => format!("{}:{};",#kebab,x),});
                acc
            });
    quote! {
        #[derive(Debug, Clone)]
        pub enum Attribute {
            #simple_ones
        }

        impl Attribute {
            fn repr(&self) -> u16 {
                use Attribute::*;
                match self {
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
                    #attrs_display
                };
                write!(f, "{}", result)
            }
        }

    }
}

fn transformers(lines: &[Line]) -> TokenStream {
    let result = lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let docs_varients = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let result = format!("- {}", x.snake_ident.snake());
            acc.extend(quote! {#[doc = #result]});
            acc
        });
        for header in line.headers() {
            let name_docs = header
                .docs
                .as_deref()
                .map(|x| {
                    let mut lines = x.lines().collect::<Vec<_>>();
                    lines.remove(lines.len() - 1);
                    lines.remove(0);
                    lines.iter().fold(TokenStream::new(), |mut acc, x| {
                        acc.extend(quote! {
                        #[doc = #x]});
                        acc
                    })
                })
                .unwrap_or(quote! {#[doc = "no description found"]});
            let pascal = header.snake_ident.pascal_ident();
            let snake = &header.snake_ident;
            acc.extend(quote!(
                #[doc = "# Definition and Usage"]
                #name_docs
                #[doc = "## possible values"]
                #docs_varients
                pub fn #snake(self) -> Styling<#pascal> {
                    self.transform()
                }
            ));
        }
        acc
    });
    quote! {
        impl Styling<Home> {
            #result
        }
    }
}

//TODO : to be refactored
fn define_varients_types(lines: &[Line]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        if let Some(group) = &line.group {
            for header in line.headers() {
                let header_pascal = header.snake_ident.pascal_ident();
                let group_pascal = format_ident!("{}", group.to_string());
                acc.extend(quote! {
                    pub struct #header_pascal(AttrValue<#group_pascal>);

                    impl Attributer for #header_pascal {
                        type Kind = #group_pascal;

                        fn from(kind: AttrValue<Self::Kind>) -> Self {
                            Self(kind)
                        }

                        fn to_attribute(self) -> Attribute {
                            let Self(group) = self;
                            Attribute::#header_pascal(group)
                        }
                    }

                });
            }
        } else {
            let varients_pascal = line
                .attrs
                .iter()
                .fold(TokenStream::new(), |mut acc, varient| {
                    let pascal = varient.snake_ident.pascal_ident();
                    acc.extend(quote! {
                        #pascal,
                    });
                    acc
                });
            for header in line.headers() {
                let header_pascal = header.snake_ident.pascal_ident();
                acc.extend(quote!(
                    #[derive(Debug, Clone)]
                    pub enum #header_pascal {
                        #varients_pascal
                    }
                ));
            }
        }
        acc
    })
}

//TODO : to be refactored
fn display_varients_types(lines: &[Line]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        for header in line.headers() {
            let header_pascal = header.snake_ident.pascal_ident();
            let varients_display = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
                let pascal = x.snake_ident.pascal_ident();
                let untrailed_kebab = clear_trailing_dash(x.kebab());
                acc.extend(quote!(
                    Self::#pascal => #untrailed_kebab,
                ));
                acc
            });

            if line.group.is_none() {
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
            }
        }
        acc
    })
}

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}
