/// Core traits for the calculator system

/// Evaluates mathematical expressions
pub trait Evaluator {
    fn eval(&self, input: &str) -> Result<f64, String>;
}

/// Converts values to different representations
pub trait Converter {
    fn convert_all(&self, value: i64) -> String;
    fn convert_to(&self, value: i64, format: &str) -> Result<String, String>;
}

/// Formats binary/bit representations
pub trait BitFormatter {
    fn format_64bit(&self, value: i64) -> String;
    fn format_bin(&self, value: i64) -> String;
}

/// Formats numeric output with grouping
pub trait NumberFormatter {
    fn format(&self, value: i64) -> String;
}
