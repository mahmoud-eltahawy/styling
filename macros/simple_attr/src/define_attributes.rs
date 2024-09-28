use parsing::parse;
use parsing::StraightLine;
use proc_macro2::TokenStream;
use quote::quote;

mod parsing;

fn clear_trailing_dash(input: String) -> String {
    if input.chars().last().is_some_and(|x| x == '_') {
        return input[0..input.len() - 1].to_string();
    };
    input
}

fn transformers(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, x| {
        let name_docs = x
            .header
            .docs
            .as_ref()
            .map(|x| format!("# {x}"))
            .unwrap_or(String::from("# no description found"));
        let pascal = x.header.pascal_ident();
        let snake = x.header.snake_ident();
        let props_docs = x.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let result = format!("- {}", x.snake());
            acc.extend(quote! {
                #[doc = #result]
            });
            acc
        });
        acc.extend(quote!(
            #[doc = #name_docs]
            #[doc = "## possible values"]
            #props_docs
            pub fn #snake(self) -> Styling<#pascal> {
                self.transform()
            }
        ));
        acc
    })
}

fn define_varients_types(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let header_pascal = line.header.pascal_ident();
        let varients_pascal = line
            .attrs
            .iter()
            .fold(TokenStream::new(), |mut acc, varient| {
                let pascal = varient.pascal_ident();
                acc.extend(quote! {
                    #pascal,
                });
                acc
            });
        acc.extend(quote!(
            pub enum #header_pascal {
                #varients_pascal
            }
        ));
        acc
    })
}

fn display_varients_types(lines: &[StraightLine]) -> TokenStream {
    lines.iter().fold(TokenStream::new(), |mut acc, line| {
        let header_pascal = line.header.pascal_ident();
        let varients_display = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
            let pascal = x.pascal_ident();
            let untrailed_kebab = clear_trailing_dash(x.kebab());
            acc.extend(quote!(
                Self::#pascal => #untrailed_kebab,
            ));
            acc
        });

        acc.extend(quote!(
            impl std::fmt::Display for #header_pascal {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let result = match self {
                        #varients_display
                    };
                    write!(f, "{}",result)
                }
            }
        ));
        acc
    })
}

pub(crate) fn define_attributes_impl(input: TokenStream) -> TokenStream {
    let lines = parse(input);

    let mut tokens = TokenStream::new();

    // for line in lines.iter() {
    //     let header_pascal = line.header.pascal_ident();
    //     let varients_pascal =
    //         line.attrs
    //             .iter()
    //             .map(|x| x.pascal_ident())
    //             .fold(TokenStream::new(), |mut acc, x| {
    //                 acc.extend(quote! {
    //                     #x,
    //                 });
    //                 acc
    //             });

    //     let varients_funs = line.attrs.iter().fold(TokenStream::new(), |mut acc, x| {
    //         let pascal = x.pascal_ident();
    //         let snake = x.snake_ident();
    //         acc.extend(quote!(
    //             pub fn #snake(self) -> Style<StyleBaseState<()>> {
    //                 self.base(#header_pascal::#pascal)
    //             }
    //         ));
    //         acc
    //     });
    // }
    let transformers = transformers(&lines);
    let varients_types = define_varients_types(&lines);
    let varients_display = display_varients_types(&lines);

    tokens.extend(quote!(

        #varients_types
        #varients_display

        impl Styling<Home> {
            #transformers
        }
    ));
    tokens
}
