pub fn first_word(s: &str) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod slices_tests {
    use super::*;

    #[test]
    fn get_the_first_word() {
        let s = String::from("hoge fuga");
        let w = first_word(&s);

        assert_eq!(w, "hoge")
    }

    #[test]
    fn get_the_whole_word() {
        let s = String::from("hoge-fuga");
        let w = first_word(&s);

        assert_eq!(w, "hoge-fuga")
    }

    #[test]
    fn string_literal_as_input() {
        let s = "hoge fuga";
        let w = first_word(&s);

        assert_eq!(w, "hoge")
    }

    #[test]
    fn a_slice_of_string_as_input() {
        let s = String::from("hoge fuga");
        let w = first_word(&s[5..]);

        assert_eq!(w, "fuga")
    }

    #[test]
    fn primitive_type_slice() {
        let a = [1, 2, 3, 4, 5];
        let b = &a[1..3];

        assert_eq!(b, [2, 3])
    }
}
