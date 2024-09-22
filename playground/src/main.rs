use styling::Style;

fn main() {
    let result = Style::default()
        .background_origin()
        .padding_box()
        .all()
        .initial()
        .accent_color()
        .dark_red()
        .accent_color()
        .red()
        .margin()
        .px(20)
        .bottom()
        .cm(30)
        .to_string();
    println!("{result}");
}
