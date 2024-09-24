extern crate proc_macro;

use parsing::{parse, StraightLine};
use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;

mod parsing;

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

fn props_snake_funs(lines: &[StraightLine]) -> proc_macro2::TokenStream {
    lines
        .iter()
        .fold(proc_macro2::TokenStream::new(), |mut acc, x| {
            let name_docs = x
                .header
                .docs
                .as_ref()
                .map(|x| format!("# {x}"))
                .unwrap_or(String::from("# no description found"));
            let pascal = x.header.pascal_ident();
            let snake = x.header.snake_ident();
            let props_docs = x
                .attrs
                .iter()
                .fold(proc_macro2::TokenStream::new(), |mut acc, x| {
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
                pub fn #snake(self) -> Style<StyleBaseState<AttributeGetter<#pascal>>> {{
                    self.into_prebase(Box::new(ToAttribute::attribute))
                }}
            ));
            acc
        })
}

fn props_pascal_names(lines: &[StraightLine]) -> proc_macro2::TokenStream {
    lines
        .iter()
        .map(|x| &x.header)
        .fold(proc_macro2::TokenStream::new(), |mut acc, x| {
            let x = x.pascal_ident();
            acc.extend(quote!(#x(#x),));
            acc
        })
}

fn props_display_maps(lines: &[StraightLine]) -> proc_macro2::TokenStream {
    lines
        .iter()
        .map(|x| &x.header)
        .fold(proc_macro2::TokenStream::new(), |mut acc, x| {
            let pascal = x.pascal_ident();
            let snake = x.snake();
            acc.extend(quote!(
                Self::#pascal(x) => format!("{}:{};",#snake,x),
            ));
            acc
        })
}

fn simple_attrs(lines: &[StraightLine]) -> proc_macro2::TokenStream {
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
pub fn define_attributes(input: TokenStream) -> TokenStream {
    let lines = parse(input);

    let mut result = String::new();

    let mut tokens = proc_macro2::TokenStream::new();

    let simple_attrs = simple_attrs(&lines);

    tokens.extend(simple_attrs);

    result.push_str(&tokens.to_string());

    for line in lines.iter() {
        let name_pascal = line.header.pascal();
        let varients_pascal = line
            .attrs
            .iter()
            .map(|x| x.pascal())
            .collect::<Vec<_>>()
            .join(",");
        let varients_maps = line.attrs.iter().fold(String::new(), |acc, x| {
            let pascal = x.pascal();
            let cleared = clear_trailing_dash(x.kebab());
            acc + &format!(r#"Self::{pascal} => "{cleared}","#)
        });

        let varients_funs = line.attrs.iter().fold(String::new(), |acc, x| {
            let pascal = x.pascal();
            let snake = x.snake();
            acc + &format!(
                r#"
pub fn {snake}(self) -> Style<StyleBaseState<()>> {{
    self.base({name_pascal}::{pascal})
}}
                    "#
            )
        });

        let the_enum = format!(
            r#"
#[derive(Hash, Eq, PartialEq)]
pub enum {name_pascal} {{ {varients_pascal} }}

impl std::fmt::Display for {name_pascal} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        let result = match self {{
            {varients_maps}
        }};
        write!(f, "{{}}",result)
    }}
}}

impl ToAttribute for {name_pascal}  {{
    fn attribute(self) -> Attribute {{
        Attribute::SimpleAttribute(SimpleAttribute::{name_pascal}(self))
    }}
}}

impl Style<StyleBaseState<AttributeGetter<{name_pascal}>>> {{ {varients_funs} }}
            "#
        );

        result.push_str(&the_enum);
    }

    result.parse().unwrap()
}
