#[cfg(test)]
mod errors_tests {
    use std::fs::File;
    use std::io::{ErrorKind, Read};

    #[test]
    fn open_and_read_file_ok() {
        let file_result = File::open("files/hello.txt");
        let mut file = match file_result {
            Ok(file) => file,
            Err(error) => panic!("failed opening file: {:?}", error),
        };

        let mut buf = String::new();
        let read_result = file.read_to_string(&mut buf);
        let read_size = match read_result {
            Ok(size) => size,
            Err(error) => panic!("failed reading file: {}", error),
        };

        assert_eq!(read_size, 14);
        assert_eq!(buf, "Hello, world.\n");
    }

    #[test]
    fn match_on_result_error() {
        let file_result = File::open("files/non-existing.txt");
        match file_result {
            Ok(_) => panic!("should fail opening non-existing file"),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => assert!(true),
                other_error => {
                    panic!("unexpected error: {:?}", other_error);
                }
            },
        }
    }

    #[test]
    fn unwrap_result() {
        let mut file = File::open(String::from("files/hello.txt")).unwrap();
        let mut buf = String::new();
        let read_size = file.read_to_string(&mut buf).unwrap();

        assert_eq!(read_size, 14);
        assert_eq!(buf, "Hello, world.\n");
    }

    //#[test]
    //fn expect_result() {
    //    let _ = File::open(String::from("files/non-existing.txt"))
    //        .expect("non-existing.txt should exist");
    //}
}
