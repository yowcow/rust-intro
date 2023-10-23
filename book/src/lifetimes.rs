pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct Something<'a> {
    pub text: &'a str,
}

use std::fmt::Display;

pub fn longest_with_announcement<'a, 'b, T>(x: &'a str, y: &'a str, ann: &'b T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod lifetimes_tests {
    use super::*;

    #[test]
    fn test_longest_with_simple_lifetimes() {
        let str1 = String::from("abcd");
        let str2: &'static str = "xyz"; // let str2 = "xyz";
        let result = longest(str1.as_str(), str2);

        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_longest_with_complex_lifetimes() {
        let string1 = String::from("long string is long");
        let result;

        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());

            assert_eq!(result, "long string is long");
        }

        //assert_eq!(result, "long string is long");
    }

    #[test]
    fn test_lifetime_in_struct_field() {
        let mut s = Something { text: "" };

        {
            let str: &'static str = "hoge"; // let str = "hoge";
            let t = String::from(str);
            s.text = t.as_str();

            assert_eq!(s.text, "hoge");
        }

        //assert_eq!(s.text, "hoge");
    }

    #[test]
    fn test_lifetime_with_type_generics() {
        let str1 = "long string is long";
        let str2 = "short string is short";
        let result;

        {
            let ann = String::from("hello world");
            result = longest_with_announcement(str1, str2, &ann);
        }

        assert_eq!(result, "short string is short");
    }
}
