use styling::styling;
fn main() {
    let styling1 = styling()
        .align_content()
        .stretch()
        .top()
        .abs(1.)
        .left()
        .px(2.)
        .accent_color()
        .hex("ff0000")
        .accent_color()
        .hex("00ff00");

    let styling2 = styling()
        .padding()
        .px(5.)
        .font_size()
        .px(16.)
        .font_size()
        .abs(18.)
        .margin()
        .cm(3.44)
        .accent_color()
        .dark_red();

    let styling = styling1.extend(styling2);

    println!("css : {}", styling);
    //
}
