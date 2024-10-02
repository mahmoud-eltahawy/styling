use std::fmt::Display;

use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use proc_macro_error2::abort;

use crate::NameCases;

pub fn parse(input: TokenStream) -> Vec<FinalLine> {
    Block::parse(input).straight()
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
                            header: vec![Name {
                                docs: None,
                                atoms: vec![ident],
                            }],
                            attrs: Attrs::List(Vec::new()),
                        }
                    });
                    self.stage = Stage::Lhs(LhsStage::Naming);
                }
                LhsStage::FreshNaming => match self.lines.last_mut() {
                    Some(Line { header, .. }) => {
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
                LhsStage::Naming => match self.lines.last_mut().and_then(|x| x.header.last_mut()) {
                    Some(name) => {
                        name.add(ident);
                    }
                    None => {
                        abort!(
                            ident,
                            "did not expect to be the first line due to header naming state"
                        )
                    }
                },
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
            Stage::Lhs(LhsStage::Naming) => line.header.last_mut().unwrap(),
            Stage::Rhs(RhsStage::Naming) => match line.header.last_mut() {
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

    fn straight(self) -> Vec<FinalLine> {
        let Block { stage: _, lines } = self;
        lines
            .into_iter()
            .map(|line| {
                let header = line.header.clone();
                match line.attrs {
                    Attrs::List(attrs) => FinalLine::Straight(StraightLine { header, attrs }),
                    Attrs::Group(group) => FinalLine::Group { header, group },
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum FinalLine {
    Straight(StraightLine),
    Group { header: Vec<Name>, group: AttrGroup },
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
enum Attrs {
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
            AttrGroup::Color => "Color",
            AttrGroup::Length => "Length",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
struct Line {
    header: Vec<Name>,
    attrs: Attrs,
}

#[derive(Debug)]
pub struct StraightLine {
    pub header: Vec<Name>,
    pub attrs: Vec<Name>,
}

impl Line {
    fn with(ident: Ident) -> Self {
        Line {
            header: vec![Name {
                docs: None,
                atoms: vec![ident],
            }],
            attrs: Attrs::List(Vec::new()),
        }
    }
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
