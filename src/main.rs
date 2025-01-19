fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq as passert_eq;
    use pretty_assertions::assert_ne as passert_ne;

    /// This test should fail, because we are checking whether actual and expected are the same
    /// and they are not.
    #[test]
    #[should_panic]
    fn show_default_assert_eq() {
        let actual = "Hello Rust";
        let expected = "Hello World";
        assert_eq!(actual, expected);
    }

    /// This test should pass, because we compare two values and expect them to be different.
    #[test]
    fn show_default_assert_ne() {
        let s1 = "Apple";
        let s2 = "Orange";
        assert_ne!(s1, s2);
    }

    /// This test should fail similar to the previous test with the standard assert.
    #[test]
    #[should_panic]
    fn show_default_pretty_assert_eq() {
        let actual = "Hello World";
        let expected = "Hello Rust";
        passert_eq!(actual, expected);
    }

    /// This test should fail because we expect both inputs to be different and they are not.
    #[test]
    #[should_panic]
    fn show_default_pretty_assert_ne() {
        let s1 = "Apple";
        let s2 = "Apple";
        passert_ne!(s1, s2);
    }

    #[test]
    #[should_panic]
    fn compare_multi_line_string() {
        let s1 = "This\nis\nmore\nthan\none\nline";
        let s2 = "Ths\nis\nnot\nmore\nthan\none\nline";
        passert_eq!(s1,s2);
    }

}

