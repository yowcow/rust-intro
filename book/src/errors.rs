use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_from_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

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

    #[test]
    fn propagating_errors() -> Result<(), Box<dyn Error>> {
        let result = read_from_file("files/hello.txt");
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn propagating_errors_on_failure() -> Result<(), &'static str> {
        let result = read_from_file("files/hello.txt0");
        match result {
            Ok(_) => Err("should fail on no such file error"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn no_early_return_on_no_error() {
        let text = "hoge fuga\nfoo bar\n";
        let opt = last_char_of_first_line(&text);
        if let Some(c) = opt {
            assert_eq!(c, 'a');
        } else {
            panic!("expected 'a' but got None");
        }
    }

    #[test]
    fn early_return_on_error() {
        let text = "";
        let opt = last_char_of_first_line(&text);
        if let Some(_) = opt {
            panic!("expected None but got Some");
        }
    }
}
