extern crate proc_macro;

use parsing::{parse, StraightLine};
use proc_macro2::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;

mod parsing;

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

fn props_snake_funs(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let name_docs = x
            .header
            .docs
            .as_ref()
            .map(|x| format!("# {x}"))
            .unwrap_or(String::from("# no description found"));
        let pascal = x.header.pascal_ident();
        let snake = x.header.snake_ident();
        let props_docs = x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let result = format!("- {}", x.snake());
            acc.extend(quote! {
                #[doc = #result]
            });
            acc
        });
        acc.extend(quote!(
            #[doc = #name_docs]
            #[doc = "## possible values"]
            #props_docs
            pub fn #snake(self) -> Style<StyleBaseState<AttributeGetter<#pascal>>> {
                self.into_prebase(Box::new(ToAttribute::attribute))
            }
        ));
        acc
    })
}

fn props_pascal_names(lines: &[StraightLine]) -> TokenStream {
    lines
        .iter()
        .map(|x| &x.header)
        .fold(TokenStream::new(), |mut acc, x| {
            let x = x.pascal_ident();
            acc.extend(quote!(#x(#x),));
            acc
        })
}

fn props_display_maps(lines: &[StraightLine]) -> TokenStream {
    lines
        .iter()
        .map(|x| &x.header)
        .fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.pascal_ident();
            let snake = x.snake();
            acc.extend(quote!(
                Self::#pascal(x) => format!("{}:{};",#snake,x),
            ));
            acc
        })
}

fn simple_attrs(lines: &[StraightLine]) -> TokenStream {
    let props_snake_funs = props_snake_funs(&lines);
    let props_pascal_names = props_pascal_names(&lines);
    let props_display_maps = props_display_maps(&lines);

    quote!(
        #[derive(Hash, Eq, PartialEq)]
        pub enum SimpleAttribute {
            #props_pascal_names
        }

        impl std::fmt::Display for SimpleAttribute {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let result = match self {
                    #props_display_maps
                };
                write!(f, "{}", result)
            }

        }

        impl Style<StyleBaseState<()>> {
            #props_snake_funs
        }
    )
}

#[proc_macro]
#[proc_macro_error]
pub fn define_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lines = parse(input);

    let mut tokens = TokenStream::new();

    let simple_attrs = simple_attrs(&lines);

    tokens.extend(simple_attrs);

    for line in lines.iter() {
        let name_pascal = line.header.pascal_ident();
        let varients_pascal =
            line.attrs
                .iter()
                .map(|x| x.pascal_ident())
                .fold(TokenStream::new(), |mut acc, x| {
                    acc.extend(quote! {
                        #x,
                    });
                    acc
                });
        // .collect::<Vec<_>>()
        // .join(",");

        let varients_maps = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.pascal_ident();
            let cleared = clear_trailing_dash(x.kebab());
            acc.extend(quote!(
                Self::#pascal => #cleared,
            ));
            acc
        });

        let varients_funs = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.pascal_ident();
            let snake = x.snake_ident();
            acc.extend(quote!(
                pub fn #snake(self) -> Style<StyleBaseState<()>> {
                    self.base(#name_pascal::#pascal)
                }
            ));
            acc
        });

        let the_enum = quote!(
            #[derive(Hash, Eq, PartialEq)]
            pub enum #name_pascal {
                 #varients_pascal
            }

            impl std::fmt::Display for #name_pascal {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        #varients_maps
                    };
                    write!(f, "{}",result)
                }
            }

            impl ToAttribute for #name_pascal  {
                fn attribute(self) -> Attribute {
                    Attribute::SimpleAttribute(SimpleAttribute::#name_pascal(self))
                }
            }

            impl Style<StyleBaseState<AttributeGetter<#name_pascal>>> {
                #varients_funs
            }
        );

        tokens.extend(the_enum);
    }

    tokens.into()
}
