mod color;
mod length;
mod simple_props;
// pub mod svg;

use simple_props::Attribute;
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug)]
pub struct Styling<T>(Vec<Attribute>, PhantomData<T>);

pub fn styling() -> Styling<Home> {
    Styling(Vec::new(), PhantomData)
}

impl<T> Styling<T> {
    fn transform<S>(self) -> Styling<S> {
        let Self(inner, _) = self;
        Styling(inner, PhantomData)
    }

    fn add_attr(self, attr: Attribute) -> Styling<Home> {
        let Self(mut inner, _) = self;
        match inner
            .iter()
            .enumerate()
            .find(|(_, x)| x.eq(&attr))
            .map(|(i, _)| i)
        {
            Some(index) => {
                inner[index] = attr;
            }
            None => inner.push(attr),
        };
        Styling(inner, PhantomData)
    }
}

pub struct Home;

impl Styling<Home> {
    pub fn extend(self, other: Styling<Home>) -> Styling<Home> {
        let Styling(other, _) = other;
        let mut base = self;
        for x in other.into_iter() {
            base = base.add_attr(x);
        }
        base
    }
}

impl Display for Styling<Home> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string());
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::styling;
    #[test]
    fn multiple_props() {
        let styling1 = styling()
            .margin()
            .px(4.)
            .font_size()
            .px(16.)
            .accent_color()
            .rgb(100., 100., 100.);
        let expected = "margin:4px;font-size:16px;accent-color:rgb(100,100,100);";
        assert_eq!(String::from(expected), styling1.to_string());
        //test cascading
        let styling2 = styling()
            .accent_color()
            .red()
            .font_size()
            .px(10.)
            .accent_color()
            .blue();
        assert_eq!(
            styling2.to_string(),
            String::from("accent-color:Blue;font-size:10px;")
        );

        //test cascading merging
        let styling3 = styling1.extend(styling2);
        let expected = "margin:4px;font-size:10px;accent-color:Blue;";
        assert_eq!(String::from(expected), styling3.to_string());
    }

    #[test]
    fn simple_attributes() {
        let result = styling()
            .all()
            .initial()
            .align_content()
            .center()
            .align_items()
            .stretch()
            .to_string();
        assert_eq!(
            "all:initial;align-content:center;align-items:stretch;",
            &result
        );

        let result = styling()
            .background_origin()
            .padding_box()
            .background_clip()
            .border_box()
            .to_string();
        assert_eq!(
            "background-origin:padding-box;background-clip:border-box;",
            &result
        );
        let result = styling()
            .border_left_style()
            .dotted()
            .border_right_style()
            .dashed()
            .to_string();
        assert_eq!(
            "border-left-style:dotted;border-right-style:dashed;",
            &result
        );
        let result = styling()
            .border_top_style()
            .solid()
            .border_bottom_style()
            .ridge()
            .border_collapse()
            .separate()
            .to_string();
        assert_eq!(
            "border-top-style:solid;border-bottom-style:ridge;border-collapse:separate;",
            &result
        );
        let result = styling()
            .break_after()
            .always()
            .break_before()
            .avoid()
            .cursor()
            .all_scroll()
            .to_string();
        assert_eq!(
            "break-after:always;break-before:avoid;cursor:all-scroll;",
            &result
        );
    }

    #[test]
    fn test_colors() {
        let result = styling()
            .border_color()
            .red()
            .background_color()
            .hex("ff0000")
            .accent_color()
            .rgb(255., 0., 0.)
            .color()
            .green()
            .to_string();
        assert_eq!(
            "border-color:Red;background-color:#ff0000;accent-color:rgb(255,0,0);color:Green;",
            &result
        );
    }
}
