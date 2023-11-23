#[cfg(test)]
mod tests {
    use crate::MagicString;

    #[test]
    fn test_overwrite() {
        let mut m = MagicString::new("hello world");
        assert!(m.overwrite(0, 5, "hi").is_ok());
        assert_eq!(m.to_string(), "hi world");
    }

    #[test]
    fn test_remove() {
        let mut m = MagicString::new("hello world");
        assert!(m.remove(0, 5).is_ok());
        assert_eq!(m.to_string(), " world");
    }
    #[test]
    fn test_prepend() {
        let mut m = MagicString::new("world");
        m.prepend("hello ");
        assert_eq!(m.to_string(), "hello world");
    }

    #[test]
    fn test_append() {
        let mut m = MagicString::new("hello");
        m.append(" world");
        assert_eq!(m.to_string(), "hello world");
    }

    #[test]
    fn test_append_left() {
        let mut m = MagicString::new("hello");
        m.append_left(3, " world").unwrap();
        assert_eq!(m.to_string(), "hel worldlo");
    }
//    越界的单词是

    #[test]
    fn test_append_left_overstep_a_boundary() {
        let mut m = MagicString::new("hello");
        m.append_left(10, " world").unwrap();
        assert_eq!(m.to_string(), " worldhello");
    }
}
