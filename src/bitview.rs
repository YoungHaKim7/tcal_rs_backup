use crate::traits::BitFormatter;

/// Binary/bit view formatter with detailed bit layout
pub struct BitView;

impl BitFormatter for BitView {
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

impl BitView {
    fn group4(s: &str) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("  ")
    }
}

/// Signed/unsigned view for i64 values
pub struct SignedView;

impl SignedView {
    pub fn format_signed_unsigned(value: i64) -> String {
        let unsigned = value as u64;
        format!(
            "Signed (i64):   {}\nUnsigned (u64): {}",
            value, unsigned
        )
    }
}
