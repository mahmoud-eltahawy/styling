use styling::Style;

simple_attr::simple_attr!(
    color:normal|light|dark|only;
    color_scheme=color;
);

fn main() {
    hello_macro();

    let result = Style::default()
        .accent_color()
        .red()
        .margin()
        .px(20)
        .bottom()
        .cm(30)
        .to_string();
    println!("{result}");
}
