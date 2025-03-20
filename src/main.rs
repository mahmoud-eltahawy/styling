use styling::css;

fn main() {
    let i = css().width().px(12.).height().rem(1.5);
    println!("{:#?}", i);
}
