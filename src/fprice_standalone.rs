// Standalone fprice module that doesn't depend on rug

/// Formats a number with comma separators (e.g., 1,234,567)
pub fn fprice(value: i64) -> String {
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
