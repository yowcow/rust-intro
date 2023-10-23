mod test_util;

#[test]
fn it_works() -> Result<(), String> {
    if test_util::setup() {
        Ok(())
    } else {
        Err(String::from("setup() returned false"))
    }
}
