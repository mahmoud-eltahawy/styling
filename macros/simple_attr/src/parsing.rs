use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};

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

    fn parse(input: TokenStream) -> Self {
        let input = proc_macro2::TokenStream::from(input);
        let block = input.into_iter().fold(Block::new(), |mut block, x| {
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
        block
    }

    fn cook(&self) -> Vec<SimpleAttrCooked> {
        self.lines
            .iter()
            .map(|line| {
                let name = line.header.snake_case();
                let props = match &line.attrs {
                    Attrs::List(list) => list.iter().map(|x| x.snake_case()).collect::<Vec<_>>(),
                    Attrs::Reference(name) => {
                        let snake = name.snake_case();
                        match self.lines.iter().find(|x| x.header.snake_case() == snake) {
                            Some(line) => match &line.attrs {
                                Attrs::List(list) => {
                                    list.iter().map(|x| x.snake_case()).collect::<Vec<_>>()
                                }
                                Attrs::Reference(name) => panic!(
                                    "{} references another reference {}",
                                    snake,
                                    name.snake_case()
                                ),
                            },
                            None => panic!("{snake} reference not found"),
                        }
                    }
                };
                SimpleAttrCooked {
                    name,
                    name_docs: None,
                    props,
                }
            })
            .collect()
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

    fn snake_case(&self) -> String {
        self.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }

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

impl SimpleAttrCooked {
    pub fn parse(input: TokenStream) -> Vec<SimpleAttrCooked> {
        Block::parse(input).cook()
    }
}
