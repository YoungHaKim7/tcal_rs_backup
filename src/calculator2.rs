use crate::traits::{Evaluator, Converter, NumberFormatter};
use crate::evaluator::{MevalEvaluator, Parser};
use crate::converter::NumberConverter;
use crate::formatter::{ResultFormatter, CommaFormatter};

/// Main calculator using trait-based architecture
pub struct Calculator {
    last: Option<f64>,
    evaluator: MevalEvaluator,
    converter: NumberConverter,
    formatter: CommaFormatter,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            last: None,
            evaluator: MevalEvaluator,
            converter: NumberConverter,
            formatter: CommaFormatter,
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<String, String> {
        // Check for "to" conversion syntax
        if let Some((expr, fmt)) = Self::extract_to(input) {
            if fmt == "unicode" || fmt == "uni" {
                return NumberConverter::string_to_unicode(&expr);
            }

            let processed = Parser::preprocess(&expr, self.last)?;
            let value = self.evaluator.eval(&processed)? as i64;

            return self.converter.convert_to(value, &fmt);
        }

        // Normal evaluation
        let processed = Parser::preprocess(input, self.last)?;
        let value = self.evaluator.eval(&processed)? as i64;

        self.last = Some(value as f64);

        let formatted = self.formatter.format(value);
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
