use magic_string_rs::MagicString;

fn main() {
    let mut m = MagicString::new("hello world");
    let _ = m.overwrite(0, 5, "hi");
    println!("{}", m.to_string());

    let mut m = MagicString::new("hello world");
    let _ = m.remove(0, 5);
    let _ = m.overwrite(5, 7, "hi");
    println!("{}", m.to_string());
}
