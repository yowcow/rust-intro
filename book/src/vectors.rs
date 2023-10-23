#[cfg(test)]
mod vectors_tests {
    #[test]
    fn new_vector_with_values() {
        let v = vec![1, 2, 3];

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn push_to_vector() {
        let mut v: Vec<i32> = Vec::new(); // vec![]
        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn vector_item() {
        let v = vec![1, 2, 3];
        let item3: &i32 = &v[2];
        let opt3: Option<&i32> = v.get(2);

        assert_eq!(*item3, 3);

        match opt3 {
            None => assert!(false),
            Some(val3) => assert_eq!(*val3, 3),
        }
    }

    #[test]
    fn read_vector_after_mutation() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6);
        let first = &v[0]; // should be read after mutation

        assert_eq!(*first, 1)
    }

    #[test]
    fn doubling_items() {
        let mut v = vec![1, 2, 3];
        for item in &mut v {
            *item *= 2;
        }

        assert_eq!(v[0], 2);
        assert_eq!(v[1], 4);
        assert_eq!(v[2], 6);
    }

    #[test]
    fn enum_types_in_vector() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        impl SpreadsheetCell {
            fn to_string(&self) -> String {
                match self {
                    SpreadsheetCell::Int(i) => format!("{} (i32)", i),
                    SpreadsheetCell::Float(f) => format!("{} (f64)", f),
                    SpreadsheetCell::Text(t) => format!("{} (String)", t),
                }
            }
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("hello")),
        ];

        let item1: Option<&SpreadsheetCell> = row.get(0);
        let item2: Option<&SpreadsheetCell> = row.get(1);
        let item3: Option<&SpreadsheetCell> = row.get(2);

        if let Some(v) = item1 {
            assert_eq!(v.to_string(), "3 (i32)");
        } else {
            panic!("item1 should be i32");
        }

        if let Some(v) = item2 {
            assert_eq!(v.to_string(), "10.12 (f64)");
        } else {
            panic!("item1 should be f64");
        }

        if let Some(v) = item3 {
            assert_eq!(v.to_string(), "hello (String)");
        } else {
            panic!("item1 should be String");
        }
    }
}
