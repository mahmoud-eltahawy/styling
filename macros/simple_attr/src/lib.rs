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
    props: Props,
}

impl SimpleAttr {
    fn cook(self, ref_list: &[SimpleAttr]) -> SimpleAttrCooked {
        let Self { name, props } = self;

        let props = match props {
            Props::List(list) => list,
            Props::Reference(reference) => {
                let Props::List(list) = ref_list
                    .iter()
                    .find(|x| x.name == reference && x.name != name)
                    .map(|x| x.props.clone())
                    .unwrap()
                else {
                    panic!("can not reference target");
                };
                list
            }
        };

        SimpleAttrCooked { name, props }
    }
}

#[derive(Debug)]
struct SimpleAttrCooked {
    name: String,
    props: Vec<String>,
}

impl SimpleAttrCooked {
    fn parse(item: TokenStream) -> Vec<SimpleAttrCooked> {
        let attrs = item
            .to_string()
            .split(';')
            .map(|x| x.trim().to_string())
            .flat_map(|x| {
                if x.is_empty() {
                    return None;
                }
                let attr = if x.contains('=') {
                    let (header, props) = x.split_once('=').unwrap();
                    SimpleAttr {
                        name: header.trim().to_string(),
                        props: Props::Reference(props.trim().to_string()),
                    }
                } else if x.contains(':') {
                    let (header, props) = x.split_once(':').unwrap();
                    SimpleAttr {
                        name: header.trim().to_string(),
                        props: Props::List(
                            props
                                .split('|')
                                .map(|x| x.trim().to_string())
                                .collect::<Vec<_>>(),
                        ),
                    }
                } else {
                    panic!("neither (:) or (=) are found");
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

//NOTE : assuming it is snake case
fn to_kebab(input: &str) -> String {
    input.replace('_', "-")
}

#[proc_macro]
pub fn simple_attr(item: TokenStream) -> TokenStream {
    let attrs = SimpleAttrCooked::parse(item);

    let mut result = String::new();

    for SimpleAttrCooked { name, props } in attrs {
        let name_pascal = to_pascal(&name);
        let varients_pascal = props
            .iter()
            .map(|x| to_pascal(x.as_str()))
            .collect::<Vec<_>>()
            .join(",");
        let varients_maps = props.iter().fold(String::new(), |acc, x| {
            let pascal = to_pascal(x);
            let kebab = to_kebab(x);
            acc + &format!(r#"Self::{pascal} => "{kebab}","#)
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

            "#
        );

        result.push_str(&the_enum);
    }

    result.parse().unwrap()
}
