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
                                snake_ident: ident,
                            }],
                            attrs: Attrs::List(Vec::new()),
                        }
                    });
                }
                LhsStage::FreshNaming => match self.lines.last_mut() {
                    Some(Line {
                        headers: header, ..
                    }) => {
                        header.push(Name::with(ident));
                    }
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line due to first header naming state"
                        )
                    }
                },
            },
            Stage::Rhs(rhs) => match rhs {
                RhsStage::FirstNaming => match self.lines.last_mut() {
                    Some(line) => {
                        line.attrs = Attrs::List(vec![Name::with(ident)]);
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
            Stage::Lhs(_) => line.headers.last_mut().unwrap(),
            Stage::Rhs(_) => match &mut line.attrs {
                Attrs::List(list) => list.last_mut().unwrap(),
                Attrs::Group(_) => abort!(literal, "groups can not be documented"),
            },
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
}

#[derive(Debug)]
enum RhsStage {
    FirstNaming,
    FreshNaming,
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
    pub snake_ident: Ident,
}

impl Name {
    fn with(ident: Ident) -> Self {
        Self {
            docs: None,
            snake_ident: ident,
        }
    }

    pub fn kebab(&self) -> String {
        self.snake_ident.kebab()
    }
}
