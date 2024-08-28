extern crate proc_macro;
use proc_macro::TokenStream;

enum Props {
    List(Vec<String>),
    Reference(String),
}

struct SimpleAttr {
    name: String,
    props: Props,
}

impl SimpleAttr {
    fn parse(item: TokenStream) -> Vec<Self> {
        let items = item.to_string();
        items
            .split(';')
            .flat_map(|x| {
                if x.is_empty() {
                    return None;
                }
                let attr = if x.contains('=') {
                    let (header, props) = x.split_once('=').unwrap();
                    SimpleAttr {
                        name: header.to_string(),
                        props: Props::Reference(props.to_string()),
                    }
                } else if x.contains(':') {
                    let (header, props) = x.split_once(':').unwrap();
                    SimpleAttr {
                        name: header.to_string(),
                        props: Props::List(
                            props.split('|').map(|x| x.to_string()).collect::<Vec<_>>(),
                        ),
                    }
                } else {
                    return None;
                };
                Some(attr)
            })
            .collect()
    }
}

#[proc_macro]
pub fn simple_attr(item: TokenStream) -> TokenStream {
    let attrs = SimpleAttr::parse(item);

    r#"
     fn hello_macro() {
         println!("hello macro"); 
     }
     "#
    .parse()
    .unwrap()
}
