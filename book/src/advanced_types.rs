#[cfg(test)]
mod synonyms_tests {
    type Kilometers = i32;

    #[test]
    fn test_synonyms() {
        let x: i32 = 5;
        let y: Kilometers = 5;

        assert_eq!(x + y, 10);
    }

    fn takes_and_returns_long_type(
        f: Box<dyn Fn() -> &'static str + Send + 'static>,
    ) -> Box<dyn Fn() -> &'static str + Send + 'static> {
        f
    }

    #[test]
    fn test_typing_multiple_traits() {
        let f: Box<dyn Fn() -> &'static str + Send + 'static> = Box::new(|| "hi");
        let result = takes_and_returns_long_type(f)();

        assert_eq!(result, "hi")
    }

    type Thunk = Box<dyn Fn() -> &'static str + Send + 'static>;

    fn takes_and_returns_alias_type(f: Thunk) -> Thunk {
        f
    }

    #[test]
    fn test_typing_alias() {
        let f: Thunk = Box::new(|| "hi");
        let result = takes_and_returns_alias_type(f)();

        assert_eq!(result, "hi")
    }
}
