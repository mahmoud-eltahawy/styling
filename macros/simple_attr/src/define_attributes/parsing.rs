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
            stage: Stage::Lhs(LhsStage::Header),
            lines: Vec::new(),
        }
    }

    fn handle_ident(&mut self, ident: Ident) {
        match &self.stage {
            Stage::Lhs(LhsStage::Header) => self.lines.push(Line::with(ident)),
            Stage::Lhs(LhsStage::Minion) => match self.lines.last_mut() {
                Some(line) => {
                    line.minions.push(Name::with(ident));
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line due to first header naming state"
                    )
                }
            },
            Stage::Rhs(RhsStage::Varients) => match self.lines.last_mut() {
                Some(Line { attrs, .. }) => {
                    attrs.push(Name::with(ident));
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line due to first attr naming state"
                    )
                }
            },
            Stage::Rhs(RhsStage::Grouping) => match self.lines.last_mut() {
                Some(line) => {
                    match line.group {
                        Some(_) => abort!(ident, "can not have multiple groups"),
                        None => {
                            line.group = Some(AttrGroup::from(ident));
                        }
                    }
                    self.stage = Stage::Rhs(RhsStage::Grouping);
                }
                None => {
                    abort!(
                        ident,
                        "did not expect to be the first line to be grouping statement"
                    )
                }
            },
        };
    }

    fn handle_literal(&mut self, literal: proc_macro2::Literal) {
        let Some(line) = self.lines.last_mut() else {
            abort!(literal, "can not add docs at beginning");
        };
        let name = match &self.stage {
            Stage::Lhs(LhsStage::Header) => &mut line.header,
            Stage::Lhs(LhsStage::Minion) => match line.minions.last_mut() {
                Some(minion) => minion,
                None => abort!(literal, "docs comes after header identifier not before"),
            },
            Stage::Rhs(_) => match line.attrs.last_mut() {
                Some(attr) => attr,
                None => abort!(literal, "docs can not come before attributes"),
            },
        };
        let docs = literal.to_string();
        name.docs = Some(docs);
    }

    fn parse(input: TokenStream) -> Self {
        input.into_iter().fold(Block::new(), |mut block, x| {
            match x {
                TokenTree::Ident(ident) => {
                    block.handle_ident(ident);
                }
                TokenTree::Punct(punct) => {
                    block.stage = Stage::from(punct);
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

impl From<Punct> for Stage {
    fn from(value: Punct) -> Self {
        match value.as_char() {
            ';' => Stage::Lhs(LhsStage::Header),
            ',' => Stage::Lhs(LhsStage::Minion),
            ':' => Stage::Rhs(RhsStage::Varients),
            '$' => Stage::Rhs(RhsStage::Grouping),
            p => abort!(value, format!("{p} unrecognized punct")),
        }
    }
}

#[derive(Debug)]
enum LhsStage {
    Header,
    Minion,
}

#[derive(Debug)]
enum RhsStage {
    Varients,
    Grouping,
}

#[derive(Debug, Clone)]
pub enum AttrGroup {
    Color,
    Length,
}

impl From<Ident> for AttrGroup {
    fn from(value: Ident) -> Self {
        match value.to_string().to_lowercase().as_str() {
            "color" => AttrGroup::Color,
            "length" => AttrGroup::Length,
            _ => {
                abort!(value, "unrecognized group");
            }
        }
    }
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
    pub header: Name,
    pub minions: Vec<Name>,
    pub group: Option<AttrGroup>,
    pub attrs: Vec<Name>,
}

impl Line {
    fn with(ident: Ident) -> Self {
        Self {
            header: Name {
                docs: None,
                snake_ident: ident,
            },
            minions: Vec::new(),
            group: None,
            attrs: Vec::new(),
        }
    }
    pub fn headers(&self) -> Vec<&Name> {
        let mut result = Vec::from([&self.header]);
        result.extend(self.minions.iter().collect::<Vec<_>>());
        result
    }
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
