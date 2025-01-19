fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    /// Usually you just want to replace the assert_eq and assert_ne macros with this
    /// those from pretty_assertions. If you do, just leave out the 'as passert_eq' and 'as
    /// passsert_ne'. The below two lines redefine the pretty_assertion macros to not have 
    /// name clashes with the standard asserts.
    use pretty_assertions::assert_eq as passert_eq;
    use pretty_assertions::assert_ne as passert_ne;
    use std::collections::HashMap;

    /// This test should fail, because we are checking whether actual and expected are the same
    /// and they are not.
    /// Remove the #[should_panic] line to see the results.
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
    /// Remove the #[should_panic] line to see the results.
    #[test]
    #[should_panic]
    fn show_default_pretty_assert_eq() {
        let actual = "Hello World";
        let expected = "Hello Rust";
        passert_eq!(actual, expected);
    }

    /// This test should fail because we expect both inputs to be different and they are not.
    /// Remove the #[should_panic] line to see the results.
    #[test]
    #[should_panic]
    fn show_default_pretty_assert_ne() {
        let s1 = "Apple";
        let s2 = "Apple";
        passert_ne!(s1, s2);
    }

    /// This test should fail. Remove the #[should_panic] line to see the results.
    #[test]
    #[should_panic]
    fn compare_multi_line_string() {
        let s1 = "This\nis\nmore\nthan\none\nline";
        let s2 = "Ths\nis\nnot\nmore\nthan\none\nline";
        passert_eq!(s1, s2);
    }

    /// This test should fail. Remove the #[should_panic] line to see the results.
    #[test]
    #[should_panic]
    fn compare_data_structures() {
        let v1 = vec![1, 3, 5, 8];
        let v2 = vec![1, 4, 6, 8];
        passert_eq!(v1, v2);
    }

    /// This test should fail. Remove the #[should_panic] line to see the results.
    #[test]
    #[should_panic]
    fn test_hashmap_equality() {
        let mut map1 = HashMap::new();
        map1.insert(1, "one");
        map1.insert(2, "two");

        let mut map2 = HashMap::new();
        map2.insert(1, "one");
        map2.insert(2, "twoo");

        // They are identical, so this will pass silently
        passert_eq!(map1, map2);
    }
}
