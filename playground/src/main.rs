use styling::Style;

fn main() {
    let result = Style::default()
        .position()
        .static_()
        .accent_color()
        .red()
        .margin()
        .px(20)
        .bottom()
        .cm(30)
        .to_string();
    println!("{result}");
}
