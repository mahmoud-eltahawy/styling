use core::panic;

use super::{clear_whitespace, Props};

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};

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

#[derive(Debug)]
struct Block {
    stage: Stage,
    lines: Vec<Line>,
}

impl Block {
    fn new() -> Self {
        Self {
            stage: Stage::FreshLine,
            lines: Vec::new(),
        }
    }

    fn last_attr(&mut self) -> Option<&mut Name> {
        self.lines
            .last_mut()
            .and_then(|line| match &mut line.attrs {
                Attrs::List(list) => list.last_mut(),
                Attrs::Reference(name) => Some(name),
            })
    }

    fn handle_ident(&mut self, ident: Ident) {
        match self.stage {
            Stage::FreshLine => {
                self.lines.push(Line::with(ident));
                self.stage = Stage::HeaderNaming;
            }
            Stage::HeaderNaming => match self.lines.last_mut() {
                Some(line) => {
                    line.header.add(ident);
                }
                None => {
                    panic!("did not expect to be the first line due to header naming state")
                }
            },
            Stage::FirstAttrNaming => match self.lines.last_mut() {
                Some(line) => {
                    line.attrs = Attrs::List(vec![Name::with(ident)]);
                    self.stage = Stage::AttrNaming;
                }
                None => {
                    panic!("did not expect to be the first line due to first attr naming state")
                }
            },
            Stage::FreshAttrNaming => match self.lines.last_mut() {
                Some(Line { attrs, .. }) => {
                    if let Attrs::List(list) = attrs {
                        list.push(Name::with(ident));
                        self.stage = Stage::AttrNaming;
                    }
                }
                None => {
                    panic!("did not expect to be the first line due to first attr naming state")
                }
            },
            Stage::FreshReference => match self.lines.last_mut() {
                Some(line) => {
                    line.attrs = Attrs::Reference(Name::with(ident));
                    self.stage = Stage::AttrNaming;
                }
                None => {
                    panic!("did not expect to be the first line due to first attr naming state")
                }
            },
            Stage::AttrNaming => match self.last_attr() {
                Some(name) => {
                    name.add(ident);
                }
                None => {
                    panic!("did not expect to be the first line or name due to attr naming state")
                }
            },
        };
    }

    fn handle_punct(&mut self, c: char) {
        match c {
            ';' => {
                self.stage = Stage::FreshLine;
            }
            ':' => {
                self.stage = Stage::FirstAttrNaming;
            }
            '|' => {
                self.stage = Stage::FreshAttrNaming;
            }
            '=' => {
                self.stage = Stage::FreshReference;
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
enum Stage {
    FreshLine,
    HeaderNaming,
    AttrNaming,
    FirstAttrNaming,
    FreshReference,
    FreshAttrNaming,
}

#[derive(Debug)]
enum Attrs {
    List(Vec<Name>),
    Reference(Name),
}

#[derive(Debug)]
struct Line {
    header: Name,
    attrs: Attrs,
}

impl Line {
    fn with(ident: Ident) -> Self {
        Line {
            header: Name(vec![ident]),
            attrs: Attrs::List(Vec::new()),
        }
    }
}

#[derive(Debug)]
struct Name(Vec<Ident>);

impl Name {
    fn with(ident: Ident) -> Self {
        Self(vec![ident])
    }
    fn add(&mut self, other: Ident) {
        self.0.push(other);
    }

    // fn snake_case(&self) -> String {
    //     self.0
    //         .iter()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<_>>()
    //         .join("_")
    // }

    // fn pascal_case(&self) -> String {
    //     self.0
    //         .iter()
    //         .map(|x| x.to_string())
    //         .map(|x| x[0..1].to_uppercase() + &x[1..])
    //         .collect()
    // }

    // fn kebab_case(&self) -> String {
    //     self.0
    //         .iter()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<_>>()
    //         .join("-")
    // }
}

pub fn experiment(input: TokenStream) {
    let input = proc_macro2::TokenStream::from(input);
    let block = input.into_iter().fold(Block::new(), |mut block, x| {
        //
        match x {
            TokenTree::Ident(ident) => {
                block.handle_ident(ident);
            }
            TokenTree::Punct(x) => {
                block.handle_punct(x.as_char());
            }
            TokenTree::Group(_) => unreachable!(),
            TokenTree::Literal(_) => unreachable!(),
        };
        block
    });
    println!("{:#?}", block);
}

impl SimpleAttrCooked {
    pub fn parse(input: TokenStream) -> Vec<SimpleAttrCooked> {
        experiment(input.clone());
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
