#[cfg(test)]
mod hashmaps_tests {
    use std::collections::HashMap;

    #[test]
    fn string_key_i32_value() {
        let hoge_key = String::from("hoge");
        let fuga_key = String::from("fuga");

        let mut m = HashMap::new();
        m.insert(hoge_key.clone(), 10);
        m.insert(fuga_key.clone(), 20);
        m.insert(hoge_key.clone(), 5);

        let hoge_value = m.get(&hoge_key).copied().unwrap_or(0);
        let fuga_value = m.get(&fuga_key).copied().unwrap_or(0);
        let foo_value = m.get(&String::from("foo")).copied().unwrap_or(0);

        assert_eq!(hoge_value, 5);
        assert_eq!(fuga_value, 20);
        assert_eq!(foo_value, 0);
    }

    #[test]
    fn string_key_string_value() {
        let hoge_key = String::from("hoge");
        let fuga_key = String::from("fuga");

        let mut m: HashMap<String, String> = HashMap::new();
        m.insert(hoge_key.clone(), String::from("hello"));
        m.insert(fuga_key.clone(), String::from("world"));

        if let Some(v) = m.get(&hoge_key) {
            assert_eq!(*v, "hello");
        } else {
            panic!("key 'hoge' not found")
        }

        if let Some(v) = m.get(&fuga_key) {
            assert_eq!(*v, "world");
        } else {
            panic!("key 'fuga' not found")
        }

        if let None = m.get(&String::from("foo")) {
            assert!(true);
        } else {
            panic!("key 'foo' should not be found")
        }
    }

    #[test]
    fn insert_only_when_key_not_exists() {
        let mut m = HashMap::new();
        m.insert(String::from("hoge"), 10);
        m.entry(String::from("fuga")).or_insert(20);
        m.entry(String::from("hoge")).or_insert(30);

        let hoge_value = m.get(&String::from("hoge")).copied().unwrap_or(0);
        let fuga_value = m.get(&String::from("fuga")).copied().unwrap_or(0);
        let foo_value = m.get(&String::from("foo")).copied().unwrap_or(0);

        assert_eq!(hoge_value, 10);
        assert_eq!(fuga_value, 20);
        assert_eq!(foo_value, 0);
    }

    #[test]
    fn update_existing_value() {
        let mut m = HashMap::new();
        m.insert(String::from("hoge"), 10);
        m.insert(String::from("fuga"), 20);

        for (_, v) in m.iter_mut() {
            *v *= 2;
        }

        let hoge_value = m.get(&String::from("hoge")).copied().unwrap_or(0);
        let fuga_value = m.get(&String::from("fuga")).copied().unwrap_or(0);
        let foo_value = m.get(&String::from("foo")).copied().unwrap_or(0);

        assert_eq!(hoge_value, 20);
        assert_eq!(fuga_value, 40);
        assert_eq!(foo_value, 0);
    }
}
