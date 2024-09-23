extern crate proc_macro;

use parsing::parse;
use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;

mod parsing;

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

#[proc_macro]
#[proc_macro_error]
pub fn define_attributes(input: TokenStream) -> TokenStream {
    let lines = parse(input);

    let mut result = String::new();

    let props_pascal_names = lines
        .iter()
        .map(|x| x.header.pascal())
        .fold(String::new(), |acc, x| acc + &format!("{x}({x}),"));

    let props_snake_funs = lines.iter().fold(String::new(), |acc, x| {
        let name_docs = x
            .header
            .docs
            .as_ref()
            .map(|x| format!("# {x}"))
            .unwrap_or(String::from("# no description found"));
        let pascal = x.header.pascal();
        let snake = x.header.snake();
        let props_docs = x
            .attrs
            .iter()
            .fold(String::from("/// ## possible values"), |acc, x| {
                acc + "\n" + &format!("/// - {}", x.snake())
            });
        acc + &format!(
            r#"
/// # {name_docs}
{props_docs}
pub fn {snake}(self) -> Style<StyleBaseState<AttributeGetter<{pascal}>>> {{
    self.into_prebase(Box::new(ToAttribute::attribute))
}}
            "#
        )
    });

    let props_display_maps = lines
        .iter()
        .map(|x| &x.header)
        .fold(String::new(), |acc, x| {
            let pascal = x.pascal();
            let snake = x.snake();
            acc + &format!(r#"Self::{pascal}(x) => format!("{snake}:{{x}};"),"#)
        });

    let simple_attrs = format!(
        r#"
impl Style<StyleBaseState<()>> {{ {props_snake_funs} }}
        
#[derive(Hash, Eq, PartialEq)]
pub enum SimpleAttribute {{
    {props_pascal_names}
}}

impl std::fmt::Display for SimpleAttribute {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        let result = match self {{
            {props_display_maps}
        }};
        write!(f, "{{}}", result)
    }}

}}
        "#
    );
    result.push_str(&simple_attrs);

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
