use proc_macro2::TokenStream;

mod parsing;
mod transpile;

pub(crate) fn define_colors_impl(input: TokenStream) -> TokenStream {
    let names = parsing::parse(input);
    transpile::transpile(names)
}
