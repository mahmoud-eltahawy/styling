use super::{clear_whitespace, Props};

use proc_macro::TokenStream;

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
pub struct SimpleAttrCooked {
    pub(crate) name: String,
    pub(crate) name_docs: Option<String>,
    pub(crate) props: Vec<String>,
}

impl SimpleAttrCooked {
    pub fn parse(input: TokenStream) -> Vec<SimpleAttrCooked> {
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
