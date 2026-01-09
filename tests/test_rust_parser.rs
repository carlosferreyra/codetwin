#[cfg(test)]
mod tests {
    use codetwin::drivers::rust::RustDriver;
    use codetwin::drivers::trait_def::Driver;

    #[test]
    fn test_parse_simple_struct() {
        let code = r#"
pub struct User {
    name: String,
    age: u32,
}
"#;

        let driver = RustDriver;
        let blueprint = driver.parse(code).expect("Failed to parse");

        println!("✅ Parsed blueprint: {:?}", blueprint);
        assert_eq!(blueprint.language, "rust");
        assert!(
            !blueprint.elements.is_empty(),
            "Should have parsed the struct"
        );
    }

    #[test]
    fn test_parse_struct_with_impl() {
        let code = r#"
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
"#;

        let driver = RustDriver;
        let blueprint = driver.parse(code).expect("Failed to parse");

        println!("✅ Parsed Point struct: {:?}", blueprint);
        assert!(
            !blueprint.elements.is_empty(),
            "Should have parsed Point struct"
        );
    }
}
