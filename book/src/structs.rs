#[allow(dead_code)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub struct Point(i32, i32, i32);

impl Point {
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod structs_tests {
    use super::*;

    #[test]
    fn a_user() {
        let u = User {
            active: false,
            username: String::from("hoge"),
            email: String::from("hoge@hoge.com"),
            sign_in_count: 1,
        };

        assert_eq!(u.active, false);
        assert_eq!(u.username, "hoge");
        assert_eq!(u.email, "hoge@hoge.com");
        assert_eq!(u.sign_in_count, 1);
    }

    #[test]
    fn second_user() {
        let active = false;
        let sign_in_count = 1;

        let u1 = User {
            active,
            username: "hoge".to_string(),
            email: "hoge@hoge.com".to_string(),
            sign_in_count,
        };
        let u2 = User {
            username: "fuga".to_string(),
            email: "fuga@fuga.com".to_string(),
            ..u1
        };

        assert_eq!(active, false);
        assert_eq!(sign_in_count, 1);

        assert_eq!(u1.active, false);
        assert_eq!(u1.username, "hoge");
        assert_eq!(u1.email, "hoge@hoge.com");
        assert_eq!(u1.sign_in_count, 1);

        assert_eq!(u2.active, false);
        assert_eq!(u2.username, "fuga");
        assert_eq!(u2.email, "fuga@fuga.com");
        assert_eq!(u2.sign_in_count, 1);
    }

    #[test]
    fn point() {
        let p1 = Point(1, 2, 3);

        assert_eq!(p1.0, 1);
        assert_eq!(p1.1, 2);
        assert_eq!(p1.2, 3);
        assert_eq!(p1.to_string(), "(1, 2, 3)");
    }
}
