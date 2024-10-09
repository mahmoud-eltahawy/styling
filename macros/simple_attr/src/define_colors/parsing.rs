use proc_macro2::{Ident, Punct, TokenStream, TokenTree};

pub fn parse(input: TokenStream) -> Vec<Name> {
    Block::parse(input).into_vec()
}

#[derive(Debug, Clone)]
struct Block {
    stage: Stage,
    names: Vec<Name>,
}

impl Block {
    fn new() -> Self {
        Self {
            stage: Stage::Fresh,
            names: Vec::new(),
        }
    }

    fn handle_ident(&mut self, ident: Ident) {
        match self.stage {
            Stage::Fresh => {
                self.names.push(Name(ident));
                self.stage = Stage::Naming;
            }
            Stage::Naming => {
                self.names.push(Name(ident));
            }
        }
    }

    fn handle_punct(&mut self, punct: Punct) {
        if let ':' = punct.as_char() {
            self.stage = Stage::Fresh;
        }
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
                TokenTree::Literal(_) | TokenTree::Group(_) => unreachable!(),
            };
            block
        })
    }

    fn into_vec(self) -> Vec<Name> {
        let Self { stage: _, names } = self;
        names
    }
}

#[derive(Debug, Clone)]
enum Stage {
    Fresh,
    Naming,
}

#[derive(Debug, Clone)]
pub struct Name(pub Ident);
