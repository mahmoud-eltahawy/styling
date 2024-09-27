use styling::{merge, styling, Home, Styling};

const STYLING1: Styling<Home, 3> = styling()
    .accent_color()
    .hex(0xff0000)
    .accent_color()
    .hex(0x00ff00)
    .accent_color()
    .hex(0x0000ff);

const STYLING2: Styling<Home, 4> = styling()
    .font_size()
    .px(16.)
    .font_size()
    .abs(18.)
    .margin()
    .cm(3.44)
    .accent_color()
    .dark_red();

fn main() {
    let styling = merge!(STYLING1, STYLING2).to_string();
    println!("css : {}", styling);
    //
}

// #[cfg(test)]
// mod tests {
//     use styling::Style;

//     #[test]
//     fn simple_attributes() {
//         let result = Style::default()
//             .all()
//             .initial()
//             .align_content()
//             .center()
//             .align_items()
//             .stretch()
//             .to_string();
//         assert_eq!(
//             "all:initial;align-content:center;align-items:stretch;",
//             &result
//         );

//         let result = Style::default()
//             .background_origin()
//             .padding_box()
//             .background_clip()
//             .border_box()
//             .to_string();
//         assert_eq!(
//             "background-origin:padding-box;background-clip:border-box;",
//             &result
//         );
//         let result = Style::default()
//             .border_left_style()
//             .dotted()
//             .border_right_style()
//             .dashed()
//             .to_string();
//         assert_eq!(
//             "border-left-style:dotted;border-right-style:dashed;",
//             &result
//         );
//         let result = Style::default()
//             .border_top_style()
//             .solid()
//             .border_bottom_style()
//             .ridge()
//             .border_collapse()
//             .separate()
//             .to_string();
//         assert_eq!(
//             "border-top-style:solid;border-bottom-style:ridge;border-collapse:separate;",
//             &result
//         );
//         let result = Style::default()
//             .break_after()
//             .always()
//             .break_before()
//             .avoid()
//             .cursor()
//             .all_scroll()
//             .to_string();
//         assert_eq!(
//             "break-after:always;break-before:avoid;cursor:all-scroll;",
//             &result
//         );
//     }
// }
