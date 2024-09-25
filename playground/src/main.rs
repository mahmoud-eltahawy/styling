use styling::experiment::styling;

fn main() {
    let i = styling().accent_color().hex(3).accent_color().hex(5);

    println!("{:#?}", i);
    //
}

#[cfg(test)]
mod tests {
    use styling::Style;

    #[test]
    fn simple_attributes() {
        let result = Style::default()
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

        let result = Style::default()
            .background_origin()
            .padding_box()
            .background_clip()
            .border_box()
            .to_string();
        assert_eq!(
            "background-origin:padding-box;background-clip:border-box;",
            &result
        );
        let result = Style::default()
            .border_left_style()
            .dotted()
            .border_right_style()
            .dashed()
            .to_string();
        assert_eq!(
            "border-left-style:dotted;border-right-style:dashed;",
            &result
        );
        let result = Style::default()
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
        let result = Style::default()
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
}
