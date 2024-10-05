use std::fmt::Display;

use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use proc_macro_error2::abort;

use crate::NameCases;

pub fn parse(input: TokenStream) -> Vec<Line> {
    Block::parse(input).lines()
}

#[derive(Debug)]
struct Block {
    stage: Stage,
    lines: Vec<Line>,
}

impl Block {
    fn new() -> Self {
        Self {
            stage: Stage::Lhs(LhsStage::FirstNaming),
            lines: Vec::new(),
        }
    }

    fn handle_ident(&mut self, ident: Ident) {
        if ident.to_string().as_str() == "are" {
            self.stage = Stage::Rhs(RhsStage::Grouping);
            return;
        }
        match &self.stage {
            Stage::Lhs(lhs) => match lhs {
                LhsStage::FirstNaming => {
                    self.lines.push({
                        Line {
                            headers: vec![Name {
                                docs: None,
                                atoms: vec![ident],
                            }],
                            attrs: Attrs::List(Vec::new()),
                        }
                    });
                    self.stage = Stage::Lhs(LhsStage::Naming);
                }
                LhsStage::FreshNaming => match self.lines.last_mut() {
                    Some(Line {
                        headers: header, ..
                    }) => {
                        header.push(Name::with(ident));
                        self.stage = Stage::Lhs(LhsStage::Naming);
                    }
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line due to first header naming state"
                        )
                    }
                },
                LhsStage::Naming => {
                    match self.lines.last_mut().and_then(|x| x.headers.last_mut()) {
                        Some(name) => {
                            name.add(ident);
                        }
                        None => {
                            abort!(
                                ident,
                                "did not expect to be the first line due to header naming state"
                            )
                        }
                    }
                }
            },
            Stage::Rhs(rhs) => match rhs {
                RhsStage::FirstNaming => match self.lines.last_mut() {
                    Some(line) => {
                        line.attrs = Attrs::List(vec![Name::with(ident)]);
                        self.stage = Stage::Rhs(RhsStage::Naming);
                    }
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line due to first attr naming state"
                        )
                    }
                },
                RhsStage::FreshNaming => match self.lines.last_mut() {
                    Some(Line { attrs, .. }) => match attrs {
                        Attrs::List(list) => {
                            list.push(Name::with(ident));
                            self.stage = Stage::Rhs(RhsStage::Naming);
                        }
                        Attrs::Group(_) => abort!(ident, "did not expect grouping"),
                    },
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line due to first attr naming state"
                        )
                    }
                },
                RhsStage::Naming => {
                    match self
                        .lines
                        .last_mut()
                        .and_then(|line| match &mut line.attrs {
                            Attrs::List(list) => Some(list),
                            Attrs::Group(_) => None,
                        })
                        .and_then(|x| x.last_mut())
                    {
                        Some(name) => {
                            name.add(ident);
                        }
                        None => {
                            abort!(
                            ident,
                            "did not expect to be the first line or name due to attr naming state"
                        )
                        }
                    }
                }
                RhsStage::Grouping => match self.lines.last_mut() {
                    Some(line) => {
                        line.attrs =
                            Attrs::Group(match ident.to_string().to_lowercase().as_str() {
                                "color" => AttrGroup::Color,
                                "length" => AttrGroup::Length,
                                _ => {
                                    abort!(ident, "unrecognized group");
                                }
                            });
                        self.stage = Stage::Rhs(RhsStage::Naming);
                    }
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line to be grouping statement"
                        )
                    }
                },
            },
        };
    }

    fn handle_punct(&mut self, punct: Punct) {
        match punct.as_char() {
            ';' => {
                self.stage = Stage::Lhs(LhsStage::FirstNaming);
            }
            ',' => {
                self.stage = Stage::Lhs(LhsStage::FreshNaming);
            }
            ':' => {
                self.stage = Stage::Rhs(RhsStage::FirstNaming);
            }
            '|' => {
                self.stage = Stage::Rhs(RhsStage::FreshNaming);
            }
            _ => (),
        }
    }

    fn handle_literal(&mut self, literal: proc_macro2::Literal) {
        let Some(line) = self.lines.last_mut() else {
            abort!(literal, "can not add docs at beginning");
        };
        let name = match &self.stage {
            Stage::Lhs(LhsStage::Naming) => line.headers.last_mut().unwrap(),
            Stage::Rhs(RhsStage::Naming) => match line.headers.last_mut() {
                Some(name) => name,
                None => abort!(literal, "did not expect to be first atom in the name"),
            },
            _ => abort!(literal, "docs can be added only after attribute!"),
        };
        let docs = literal
            .to_string()
            .chars()
            .filter(|x| *x != '"')
            .collect::<String>();
        name.docs = Some(docs);
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

    fn lines(self) -> Vec<Line> {
        let Block { stage: _, lines } = self;
        lines
    }
}

#[derive(Debug)]
enum Stage {
    Lhs(LhsStage),
    Rhs(RhsStage),
}

#[derive(Debug)]
enum LhsStage {
    FirstNaming,
    FreshNaming,
    Naming,
}

#[derive(Debug)]
enum RhsStage {
    FirstNaming,
    FreshNaming,
    Naming,
    Grouping,
}

#[derive(Debug, Clone)]
pub enum Attrs {
    List(Vec<Name>),
    Group(AttrGroup),
}

#[derive(Debug, Clone)]
pub enum AttrGroup {
    Color,
    Length,
}

impl Display for AttrGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            AttrGroup::Color => "ColorAttribute",
            AttrGroup::Length => "LengthAttribute",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub headers: Vec<Name>,
    pub attrs: Attrs,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub docs: Option<String>,
    pub atoms: Vec<Ident>,
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

    pub fn kebab(&self) -> String {
        self.atoms.kebab()
    }
}
