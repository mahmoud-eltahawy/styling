use simple_attr::simple_attr;
use styling::Style;

simple_attr!(
    color-scheme=color;
    color:normal|light-blue|dark-green|only;
);

fn main() {
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
