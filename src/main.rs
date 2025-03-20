use styling::css;

fn main() {
    let i = css().width().px(12.).height().pc(2.);
    println!("{:#?}", i);
}
