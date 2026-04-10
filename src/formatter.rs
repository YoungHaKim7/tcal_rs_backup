use crate::traits::{BitFormatter, NumberFormatter};

/// Result formatter for displaying calculator outputs
pub struct ResultFormatter;

impl ResultFormatter {
    /// Create full output with all representations
    pub fn full_output(value: i64, formatted: &str) -> String {
        let hex = format!("0x{:X}", value);
        let oct = format!("0o{:o}", value);
        let formatter = ResultFormatter;
        let bin = formatter.format_bin(value);
        let binary_64 = formatter.format_64bit(value);

        format!(
            "\t{}\n\
━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
HEX : \"{}\"\n\
DEC : \"{}\"\n\
OCT : \"{}\"\n\
BIN : \"{}\"\n\
{}\n",
            formatted, hex, formatted, oct, bin, binary_64
        )
    }
}

impl BitFormatter for ResultFormatter {
    fn format_64bit(&self, value: i64) -> String {
        let bits = format!("{:064b}", value);

        let upper = &bits[0..32];
        let lower = &bits[32..64];

        format!(
            "{}\n63                      47                  32\n\n{}\n31                      15                   0",
            Self::group4(upper),
            Self::group4(lower)
        )
    }

    fn format_bin(&self, value: i64) -> String {
        let raw = format!("{:b}", value);
        let padding = (4 - raw.len() % 4) % 4;
        let padded = format!("{}{}", "0".repeat(padding), raw);

        let grouped = padded
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ");

        format!("0b{}", grouped)
    }
}

impl ResultFormatter {
    fn group4(s: &str) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("  ")
    }
}

/// Number formatter with comma separators
pub struct CommaFormatter;

impl NumberFormatter for CommaFormatter {
    fn format(&self, value: i64) -> String {
        let s = value.to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }

        result
    }
}
