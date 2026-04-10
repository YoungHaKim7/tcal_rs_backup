use meval::eval_str;

use super::{converter::Converter, formatter::ResultFormatter, parser::Parser};
use crate::fprice::PriceFormatter;

pub struct Calculator {
    last: Option<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self { last: None }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<String, String> {
        if let Some((expr, fmt)) = Self::extract_to(input) {
            if fmt == "unicode" || fmt == "uni" {
                return Converter::string_to_unicode(&expr);
            }

            let processed = Parser::preprocess(&expr, self.last)?;
            let value = eval_str(&processed).map_err(|e| e.to_string())? as i64;

            return Converter::convert(value, &fmt);
        }

        let processed = Parser::preprocess(input, self.last)?;
        let value = eval_str(&processed).map_err(|e| e.to_string())? as i64;

        self.last = Some(value as f64);

        let formatted = PriceFormatter::format(value);

        Ok(ResultFormatter::full_output(value, &formatted))
    }

    fn extract_to(input: &str) -> Option<(String, String)> {
        input.find(" to ").map(|pos| {
            (
                input[..pos].trim().to_string(),
                input[pos + 4..].trim().to_string(),
            )
        })
    }
}
