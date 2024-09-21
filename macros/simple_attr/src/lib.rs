extern crate proc_macro;
use core::panic;

use proc_macro::TokenStream;

#[derive(Debug, Clone)]
enum Props {
    List(Vec<String>),
    Reference(String),
}

#[derive(Debug, Clone)]
struct SimpleAttr {
    name: String,
    name_docs: Option<String>,
    props: Props,
}

impl SimpleAttr {
    fn cook(self, ref_list: &[SimpleAttr]) -> SimpleAttrCooked {
        let Self {
            name,
            name_docs,
            props,
        } = self;

        let props = match props {
            Props::List(list) => list,
            Props::Reference(reference) => {
                let list = match ref_list
                    .iter()
                    .find(|x| x.name == reference)
                    .map(|x| x.props.clone())
                    .unwrap_or_else(|| panic!("can not find the reference : {reference}"))
                {
                    Props::List(list) => list,
                    Props::Reference(ref_ref) => {
                        panic!("reference {reference} points to another reference {ref_ref}");
                    }
                };
                list
            }
        };

        SimpleAttrCooked {
            name,
            name_docs,
            props,
        }
    }
}

#[derive(Debug)]
struct SimpleAttrCooked {
    name: String,
    name_docs: Option<String>,
    props: Vec<String>,
}

impl SimpleAttrCooked {
    fn parse(input: TokenStream) -> Vec<SimpleAttrCooked> {
        let attrs = input
            .to_string()
            .split(';')
            .flat_map(|x| {
                if x.is_empty() {
                    return None;
                }
                let attr = if let Some((header, props)) = x.split_once('=') {
                    SimpleAttr {
                        name: clear_whitespace(header),
                        name_docs: None,
                        props: Props::Reference(clear_whitespace(props)),
                    }
                } else if let Some((header, props)) = x.split_once(':') {
                    let (header, name_docs) = if let Some((_, header)) = header.split_once("///") {
                        let (docs, header) = header.split_once('\n').unwrap();
                        (header.trim(), Some(docs.trim().to_string()))
                    } else {
                        (header.trim(), None)
                    };
                    SimpleAttr {
                        name: clear_whitespace(header),
                        name_docs,
                        props: Props::List(
                            props.split('|').map(clear_whitespace).collect::<Vec<_>>(),
                        ),
                    }
                } else {
                    panic!("neither (:) or (=) are found on : {x}");
                };
                Some(attr)
            })
            .collect::<Vec<_>>();

        attrs.clone().into_iter().map(|x| x.cook(&attrs)).collect()
    }
}

//NOTE : assuming it is snake case
fn to_pascal(input: &str) -> String {
    input
        .split('_')
        .map(|x| x[0..1].to_uppercase() + &x[1..])
        .collect::<String>()
}

//NOTE : assuming it is kebab case
fn to_kebab(input: &str) -> String {
    input.replace('_', "-")
}

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

fn clear_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join("")
}

#[proc_macro]
pub fn define_attributes(item: TokenStream) -> TokenStream {
    let attrs = SimpleAttrCooked::parse(item);

    let mut result = String::new();

    let props_pascal_names = attrs
        .iter()
        .map(|x| to_pascal(x.name.as_str()))
        .fold(String::new(), |acc, x| acc + &format!("{x}({x}),"));

    let props_snake_funs = attrs.iter().fold(String::new(), |acc, x| {
        let SimpleAttrCooked {
            name, name_docs, ..
        } = x;
        let name_docs = name_docs
            .as_ref()
            .map(|x| format!("{name} is `{x}`"))
            .unwrap_or(name.clone());
        let pascal = to_pascal(name);
        let props_docs = x
            .props
            .iter()
            .fold(String::from("/// ## possible values"), |acc, x| {
                acc + "\n" + &format!("/// - {x}")
            });
        acc + &format!(
            r#"
/// # {name_docs}
{props_docs}
pub fn {name}(self) -> Style<StyleBaseState<AttributeGetter<{pascal}>>> {{
    self.into_prebase(Box::new(ToAttribute::attribute))
}}
            "#
        )
    });

    let props_display_maps = attrs.iter().map(|x| &x.name).fold(String::new(), |acc, x| {
        let pascal = to_pascal(x);
        acc + &format!(r#"Self::{pascal}(x) => format!("{x}:{{x}};"),"#)
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

    for SimpleAttrCooked { name, props, .. } in attrs.iter() {
        let name_pascal = to_pascal(name);
        let varients_pascal = props
            .iter()
            .map(|x| to_pascal(x.as_str()))
            .collect::<Vec<_>>()
            .join(",");
        let varients_maps = props.iter().fold(String::new(), |acc, x| {
            let pascal = to_pascal(x);
            let cleared = clear_trailing_dash(x.to_string());
            acc + &format!(r#"Self::{pascal} => "{cleared}","#)
        });

        let varients_funs = props.iter().fold(String::new(), |acc, x| {
            let pascal = to_pascal(x);
            acc + &format!(
                r#"
pub fn {x}(self) -> Style<StyleBaseState<()>> {{
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
