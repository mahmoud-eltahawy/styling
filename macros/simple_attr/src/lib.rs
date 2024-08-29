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

#[derive(Debug)]
struct SimpleAttrCooked {
    name: String,
    props: Vec<String>,
}

impl SimpleAttr {
    fn parse(item: TokenStream) -> Vec<Self> {
        item.to_string()
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
                    panic!("neither : or = are found");
                };
                Some(attr)
            })
            .collect()
    }

    fn cook(self, ref_list: &Vec<SimpleAttr>) -> SimpleAttrCooked {
        let Self { name, props } = self;

        let props = match props {
            Props::List(list) => list,
            Props::Reference(reference) => {
                let Props::List(list) = ref_list
                    .iter()
                    .find(|x| x.name == reference)
                    .unwrap()
                    .props
                    .clone()
                else {
                    panic!("can not reference target");
                };
                list
            }
        };

        SimpleAttrCooked { name, props }
    }
}

#[proc_macro]
pub fn simple_attr(item: TokenStream) -> TokenStream {
    let attrs = SimpleAttr::parse(item);
    let _attrs = attrs
        .clone()
        .into_iter()
        .map(|x| x.cook(&attrs))
        .collect::<Vec<_>>();

    let tokens_acc = proc_macro2::TokenStream::new();

    TokenStream::from(tokens_acc)
}
