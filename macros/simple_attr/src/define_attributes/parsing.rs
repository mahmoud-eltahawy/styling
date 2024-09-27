use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use proc_macro_error2::abort;
use quote::format_ident;

pub fn parse(input: TokenStream) -> Vec<StraightLine> {
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

    fn straight(self) -> Vec<StraightLine> {
        let Block { stage: _, lines } = self;
        let ref_lines = lines.clone();
        lines
            .into_iter()
            .map(|line| {
                let header = line.header.clone();
                let attrs = match line.attrs {
                    Attrs::List(list) => list,
                    Attrs::Reference(ref_name) => {
                        let snake_ref_name = ref_name.snake();
                        let error_position = ref_name.name_atoms.last().unwrap();
                        match ref_lines
                            .iter()
                            .find(|x| x.header.snake() == snake_ref_name)
                        {
                            Some(line) => match &line.attrs {
                                Attrs::List(list) => list.clone(),
                                Attrs::Reference(name) => {
                                    let error_position =
                                        name.name_atoms.last().unwrap_or(error_position);
                                    abort!(
                                        error_position,
                                        format!(
                                            "{} references another reference {}",
                                            snake_ref_name,
                                            name.snake()
                                        )
                                    );
                                }
                            },
                            None => abort!(
                                error_position,
                                format!("{snake_ref_name} reference not found")
                            ),
                        }
                    }
                };
                StraightLine { header, attrs }
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

#[derive(Debug, Clone)]
enum Attrs {
    List(Vec<Name>),
    Reference(Name),
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
                name_atoms: vec![ident],
            },
            attrs: Attrs::List(Vec::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Name {
    pub docs: Option<String>,
    name_atoms: Vec<Ident>,
}

impl Name {
    fn with(ident: Ident) -> Self {
        Self {
            docs: None,
            name_atoms: vec![ident],
        }
    }

    fn add(&mut self, other: Ident) {
        self.name_atoms.push(other);
    }

    pub fn snake(&self) -> String {
        self.name_atoms
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("_")
    }

    pub fn snake_ident(&self) -> Ident {
        format_ident!("{}", self.snake())
    }

    fn pascal(&self) -> String {
        self.name_atoms
            .iter()
            .map(|x| x.to_string())
            .map(|x| x[0..1].to_uppercase() + &x[1..])
            .collect()
    }

    pub fn pascal_ident(&self) -> Ident {
        format_ident!("{}", self.pascal())
    }

    pub fn kebab(&self) -> String {
        self.name_atoms
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }
}
