mod parser {
    pub fn parse(input: &str) -> Result<(), String> {
        if input.is_empty() {
            Err("Input cannot be empty".to_string())
        } else {
            Ok(())
        }
    }
}
