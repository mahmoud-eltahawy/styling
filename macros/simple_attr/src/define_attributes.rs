use proc_macro2::TokenStream;

mod parsing;
mod transpile;

pub(crate) fn define_attributes_impl(input: TokenStream) -> TokenStream {
    let lines = parsing::parse(input);
    transpile::transpile(lines)
}
