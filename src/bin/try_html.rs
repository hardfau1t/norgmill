#![recursion_limit = "512"]
fn main() {
    let mut builder = html::inline_text::Anchor::builder();
    builder.href("path");
    builder.href("to");
    builder.href("file");
    builder.aria_label("huh");
    let para: String = builder.build().to_string();
    println!("para text: {para:?}");

}
