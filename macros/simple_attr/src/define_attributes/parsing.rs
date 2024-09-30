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
                Attrs::Group(_) => None,
            })
    }

    fn handle_ident(&mut self, ident: Ident) {
        if ident.to_string().as_str() == "is" {
            self.stage = Stage::Grouping;
            return;
        }
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
            Stage::Grouping => match self.lines.last_mut() {
                Some(line) => {
                    line.attrs = Attrs::Group(match ident.to_string().as_str() {
                        "color" => AttrGroup::Color,
                        "length" => AttrGroup::Length,
                        _ => {
                            abort!(ident, "unrecognized group");
                        }
                    });
                    self.stage = Stage::AttrNaming;
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
                Attrs::Group(_) => abort!(literal, "can not doc group!"),
            },
            FreshReference => abort!(literal, "can not doc reference!!"),
            Grouping => abort!(literal, "can not doc group!"),
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
        let ref_lines = lines.clone();
        lines
            .into_iter()
            .map(|line| {
                let header = line.header.clone();
                match line.attrs {
                    Attrs::List(attrs) => FinalLine::Straight(StraightLine { header, attrs }),
                    Attrs::Reference(ref_name) => {
                        let snake_ref_name = ref_name.atoms.snake();
                        let error_position = ref_name.atoms.last().unwrap();
                        match ref_lines
                            .iter()
                            .find(|x| x.header.atoms.snake() == snake_ref_name)
                        {
                            Some(line) => match &line.attrs {
                                Attrs::List(attrs) => FinalLine::Straight(StraightLine {
                                    header,
                                    attrs: attrs.clone(),
                                }),
                                Attrs::Reference(name) => {
                                    let error_position =
                                        name.atoms.last().unwrap_or(error_position);
                                    abort!(
                                        error_position,
                                        format!(
                                            "{} references another reference {}",
                                            snake_ref_name,
                                            name.atoms.snake()
                                        )
                                    );
                                }
                                Attrs::Group(group) => FinalLine::Group {
                                    header,
                                    group: group.clone(),
                                },
                            },
                            None => abort!(
                                error_position,
                                format!("{snake_ref_name} reference not found")
                            ),
                        }
                    }
                    Attrs::Group(group) => FinalLine::Group { header, group },
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum FinalLine {
    Straight(StraightLine),
    Group { header: Name, group: AttrGroup },
}

#[derive(Debug)]
enum Stage {
    FreshLine,
    HeaderNaming,
    AttrNaming,
    FirstAttrNaming,
    FreshReference,
    FreshAttrNaming,
    Grouping,
}

#[derive(Debug, Clone)]
enum Attrs {
    List(Vec<Name>),
    Reference(Name),
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
    header: Name,
    attrs: Attrs,
}

#[derive(Debug)]
pub struct StraightLine {
    pub header: Name,
    pub attrs: Vec<Name>,
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
