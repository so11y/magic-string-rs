use magic_string_rs::MagicString;

fn main() {
    let mut m = MagicString::new("hello world");
    let _ = m.overwrite(0, 5, "hi");
    println!("{}", m.to_string());

    let mut m = MagicString::new("哈嘻嘻哈 嘻哈哈嘻");
    let _ = m.remove(0, 5);
    let _ = m.overwrite(6, 8, "嘻嘻");
    println!("{}", m.to_string());
}
