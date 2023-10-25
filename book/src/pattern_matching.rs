#[cfg(test)]
mod tests {
    #[test]
    fn test_match() {
        let f = |v: i32| -> &str {
            match v {
                1 | 2 => "one or two",
                3..=5 => "between three and five",
                x if x < 1 => "less than one",
                _ => "more than five",
            }
        };

        let input = vec![0, 1, 2, 3, 4, 5, 6];
        let mut output = vec![];
        input.iter().for_each(|v| output.push(f(*v)));

        assert_eq!(
            output,
            vec![
                "less than one",
                "one or two",
                "one or two",
                "between three and five",
                "between three and five",
                "between three and five",
                "more than five",
            ]
        );
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_match_break_apart() {
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;

        assert_eq!(x, 0);
        assert_eq!(y, 7);
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    #[test]
    fn test_match_enum() {
        let f = |v: &Message| -> String {
            match v {
                Message::Quit => String::from("quit"),
                Message::Move {
                    x: prohibited_range @ 1..=3,
                    ..
                } => format!("move to prohibited range x: {prohibited_range}"),
                Message::Move { x, y } => format!("move to: ({x}, {y})"),
                Message::Write(s) => format!("write: {s}"),
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    format!("change color: red {r}, green {g}, blue {b}")
                }
                Message::ChangeColor(Color::Hsv(h, s, v)) => {
                    format!("change color: hue {h}, saturation {s}, value {v}")
                }
            }
        };

        let input = vec![
            Message::Quit,
            Message::Move { x: 1, y: 2 },
            Message::Move { x: 4, y: 2 },
            Message::Write(String::from("Hello")),
            Message::ChangeColor(Color::Rgb(0, 127, 255)),
            Message::ChangeColor(Color::Hsv(0, 127, 255)),
        ];
        let mut output = vec![];
        input.iter().for_each(|v| output.push(f(v)));

        assert_eq!(
            output,
            vec![
                "quit",
                "move to prohibited range x: 1",
                "move to: (4, 2)",
                "write: Hello",
                "change color: red 0, green 127, blue 255",
                "change color: hue 0, saturation 127, value 255",
            ]
        );
    }
}
