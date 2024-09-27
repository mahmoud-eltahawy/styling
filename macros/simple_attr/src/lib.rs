extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error2::proc_macro_error;

mod define_attributes;

#[proc_macro]
#[proc_macro_error]
pub fn define_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    define_attributes::define_attributes_impl(input).into()
}
