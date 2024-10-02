use parsing::parse;
use proc_macro2::TokenStream;
// use transpile::transpile;

mod parsing;
// mod transpile;

pub(crate) fn define_attributes_impl(input: TokenStream) -> TokenStream {
    parse(input);
    "".parse().unwrap()
}
