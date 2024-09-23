use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use proc_macro_error2::abort;

#[derive(Debug)]
pub struct SimpleAttrCooked {
    pub name: String,
    pub name_docs: Option<String>,
    pub props: Vec<String>,
}

impl SimpleAttrCooked {
    pub fn parse(input: proc_macro::TokenStream) -> Vec<SimpleAttrCooked> {
        let input = TokenStream::from(input);
        Block::parse(input).cook()
    }
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
                    abort!(
                        ident,
                        "did not expect to be the first line due to header naming state"
                    )
                }
            },
            Stage::FirstAttrNaming => match self.lines.last_mut() {
                Some(line) => {
                    line.attrs = Attrs::List(vec![Name::with(ident)]);
                    self.stage = Stage::AttrNaming;
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line due to first attr naming state"
                    )
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
                    abort!(
                        ident,
                        "did not expect to be the first line due to first attr naming state"
                    )
                }
            },
            Stage::FreshReference => match self.lines.last_mut() {
                Some(line) => {
                    line.attrs = Attrs::Reference(Name::with(ident));
                    self.stage = Stage::AttrNaming;
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line due to first attr naming state"
                    )
                }
            },
            Stage::AttrNaming => match self.last_attr() {
                Some(name) => {
                    name.add(ident);
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line or name due to attr naming state"
                    )
                }
            },
        };
    }

    fn handle_punct(&mut self, punct: Punct) {
        match punct.as_char() {
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

    fn handle_literal(&mut self, literal: proc_macro2::Literal) {
        use Stage::*;
        let Some(line) = self.lines.last_mut() else {
            abort!(literal, "can not add docs at beginning");
        };
        let name = match self.stage {
            FreshLine | HeaderNaming => &mut line.header,
            AttrNaming | FirstAttrNaming | FreshAttrNaming => match &mut line.attrs {
                Attrs::List(list) => match list.last_mut() {
                    Some(name) => name,
                    None => abort!(literal, "can not add doc after : sign"),
                },
                Attrs::Reference(_) => abort!(literal, "can not doc reference!"),
            },
            FreshReference => abort!(literal, "can not doc reference!!"),
        };
        name.docs = Some(literal.to_string());
    }

    fn parse(input: TokenStream) -> Self {
        input.into_iter().fold(Block::new(), |mut block, x| {
            match x {
                TokenTree::Ident(ident) => {
                    block.handle_ident(ident);
                }
                TokenTree::Punct(x) => {
                    block.handle_punct(x);
                }
                TokenTree::Literal(literal) => {
                    block.handle_literal(literal);
                }
                TokenTree::Group(_) => unreachable!(),
            };
            block
        })
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
                        let ident = name.atoms.last().unwrap();
                        match self.lines.iter().find(|x| x.header.snake_case() == snake) {
                            Some(line) => match &line.attrs {
                                Attrs::List(list) => {
                                    list.iter().map(|x| x.snake_case()).collect::<Vec<_>>()
                                }
                                Attrs::Reference(name) => {
                                    let ident = name.atoms.last().unwrap();
                                    abort!(
                                        ident,
                                        format!(
                                            "{} references another reference {}",
                                            snake,
                                            name.snake_case()
                                        )
                                    );
                                }
                            },
                            None => abort!(ident, format!("{snake} reference not found")),
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
            header: Name {
                docs: None,
                atoms: vec![ident],
            },
            attrs: Attrs::List(Vec::new()),
        }
    }
}

#[derive(Debug)]
struct Name {
    docs: Option<String>,
    atoms: Vec<Ident>,
}

impl Name {
    fn with(ident: Ident) -> Self {
        Self {
            docs: None,
            atoms: vec![ident],
        }
    }

    fn add(&mut self, other: Ident) {
        self.atoms.push(other);
    }

    fn snake_case(&self) -> String {
        self.atoms
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
