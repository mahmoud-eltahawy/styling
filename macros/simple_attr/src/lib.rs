extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;

mod define_attributes;
mod define_colors;

#[proc_macro]
#[proc_macro_error]
pub fn define_attributes(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    define_attributes::define_attributes_impl(input).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn define_colors(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    define_colors::define_colors_impl(input).into()
}

pub(crate) trait NameCases {
    fn snake(&self) -> String;
    fn snake_ident(&self) -> proc_macro2::Ident;
    fn pascal(&self) -> String;
    fn pascal_ident(&self) -> proc_macro2::Ident;
    fn kebab(&self) -> String;
}
