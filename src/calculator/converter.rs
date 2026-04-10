pub struct Converter;

impl Converter {
    pub fn convert(value: i64, format: &str) -> Result<String, String> {
        match format {
            "hex" => Ok(format!("0x{:X}", value)),
            "bin" => Ok(format!("0b{:b}", value)),
            "oct" => Ok(format!("0o{:o}", value)),
            "unicode" | "uni" => Self::to_unicode(value),
            _ => Err(format!("Unknown format: {}", format)),
        }
    }

    pub fn string_to_unicode(input: &str) -> Result<String, String> {
        let content = input.trim_matches(|c| c == '"' || c == '\'');

        let result: Vec<String> = content
            .chars()
            .map(|c| {
                let code = c as u32;
                format!("\t'{}' → U+{:04X} ({})", c, code, code)
            })
            .collect();

        Ok(result.join(",\n"))
    }

    fn to_unicode(value: i64) -> Result<String, String> {
        if let Some(c) = char::from_u32(value as u32) {
            Ok(format!("U+{:04X} '{}'", value, c))
        } else {
            Err("Invalid Unicode".to_string())
        }
    }
}
