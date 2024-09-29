use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{define_attributes::parsing::AttrGroup, NameCases};

use super::parsing::FinalLine;

pub(crate) fn transpile(lines: Vec<FinalLine>) -> TokenStream {
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

fn simple_varients_funs(lines: &[FinalLine]) -> TokenStream {
    lines
        .iter()
        .flat_map(|x| match x {
            FinalLine::Straight(line) => Some(line),
            FinalLine::Group { .. } => None,
        })
        .fold(TokenStream::new(), |mut acc, x| {
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

fn transformers(lines: &[FinalLine]) -> TokenStream {
    let result = lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let header = match x {
            FinalLine::Straight(x) => &x.header,
            FinalLine::Group { header, .. } => header,
        };
        let name_docs = header
            .docs
            .as_ref()
            .map(|x| format!("# {x}"))
            .unwrap_or(String::from("# no description found"));
        let pascal = header.atoms.pascal_ident();
        let snake = header.atoms.snake_ident();
        let props_docs = match x {
            FinalLine::Straight(x) => x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
                let result = format!("- {}", x.atoms.snake());
                acc.extend(quote! {
                    #[doc = #result]
                });
                acc
            }),
            FinalLine::Group { .. } => quote! {
                #[doc = "it is a color so it takes all colors attributes"]
            },
        };
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

fn define_varients_types(lines: &[FinalLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let quoted = match line {
            FinalLine::Straight(line) => {
                let header_pascal = line.header.atoms.pascal_ident();
                let varients_pascal =
                    line.attrs
                        .iter()
                        .fold(TokenStream::new(), |mut acc, varient| {
                            let pascal = varient.atoms.pascal_ident();
                            acc.extend(quote! {
                                #pascal,
                            });
                            acc
                        });
                quote!(
                    #[derive(Debug, Clone)]
                    pub enum #header_pascal {
                        #varients_pascal
                    }
                )
            }
            FinalLine::Group { header, group } => {
                let header_pascal = header.atoms.pascal_ident();
                let group_pascal_attributer = format_ident!("{group}Attributer");
                let group_snake = format_ident!("{}", group.to_string().to_lowercase());
                let group_pascal = format_ident!("{}", group.to_string());
                quote! {
                    // hello
                    pub struct #header_pascal;

                    impl #group_pascal_attributer for #header_pascal {
                        fn #group_snake(#group_snake: #group_pascal) -> Attribute {
                            Attribute::#header_pascal(#group_snake)
                        }
                    }

                }
            }
        };
        acc.extend(quoted);
        acc
    })
}

fn display_varients_types(lines: &[FinalLine]) -> TokenStream {
    lines
        .iter()
        .flat_map(|x| match x {
            FinalLine::Straight(line) => Some(line),
            FinalLine::Group { .. } => None,
        })
        .fold(TokenStream::new(), |mut acc, line| {
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

fn main_attributes(lines: &[FinalLine]) -> TokenStream {
    let simple_ones = lines.iter().fold(TokenStream::new(), |mut acc, x| {
        match x {
            FinalLine::Straight(x) => {
                let x = x.header.atoms.pascal_ident();
                acc.extend(quote! {
                    #x(#x),
                });
            }
            FinalLine::Group { header, group } => {
                let outer = header.atoms.pascal_ident();
                let inner = match group {
                    AttrGroup::Color => format_ident!("Color"),
                    AttrGroup::Length => format_ident!("Length"),
                };
                acc.extend(quote! {
                    #outer(#inner),
                });
            }
        }
        acc
    });
    let eq_attrs = lines
        .iter()
        .enumerate()
        .fold(TokenStream::new(), |mut acc, (i, x)| {
            let x = match x {
                FinalLine::Straight(x) => &x.header,
                FinalLine::Group { header, .. } => header,
            };
            let x = x.atoms.pascal_ident();
            acc.extend(quote! {
                #x(_) => #i,
            });
            acc
        });
    let attrs_display = lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let x = match x {
            FinalLine::Straight(x) => &x.header,
            FinalLine::Group { header, .. } => header,
        };
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
            #simple_ones
        }

        impl Attribute {
            fn repr(&self) -> usize {
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
