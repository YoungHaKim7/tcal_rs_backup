use meval::eval_str;

pub struct Parser;

impl Parser {
    pub fn preprocess(expr: &str, last: Option<f64>) -> Result<String, String> {
        let mut result = expr.to_lowercase();

        if let Some(res) = last {
            result = result.replace("res", &res.to_string());
        }

        result = Self::replace_unicode_ops(&result);
        result = Self::convert_literals(&result)?;
        result = Self::process_power(&result)?;

        Ok(result)
    }

    fn replace_unicode_ops(expr: &str) -> String {
        expr.replace('¬', "~")
            .replace('∨', "|")
            .replace('∧', "&")
            .replace('⊻', "^")
            .replace("xor", "^^")
            .replace("**", "^")
    }

    fn convert_literals(expr: &str) -> Result<String, String> {
        let mut out = expr.to_string();
        out = Self::convert_hex(&out)?;
        out = Self::convert_bin(&out)?;
        out = Self::convert_oct(&out)?;
        Ok(out)
    }

    fn convert_hex(expr: &str) -> Result<String, String> {
        let mut result = expr.to_string();
        while let Some(pos) = result.find("0x") {
            let end = pos + 2;
            let hex: String = result[end..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit())
                .collect();

            if let Ok(v) = i64::from_str_radix(&hex, 16) {
                result.replace_range(pos..end + hex.len(), &v.to_string());
            } else {
                break;
            }
        }
        Ok(result)
    }

    fn convert_bin(expr: &str) -> Result<String, String> {
        let mut result = expr.to_string();
        while let Some(pos) = result.find("0b") {
            let end = pos + 2;
            let bin: String = result[end..]
                .chars()
                .take_while(|c| *c == '0' || *c == '1')
                .collect();

            if let Ok(v) = i64::from_str_radix(&bin, 2) {
                result.replace_range(pos..end + bin.len(), &v.to_string());
            } else {
                break;
            }
        }
        Ok(result)
    }

    fn convert_oct(expr: &str) -> Result<String, String> {
        let mut result = expr.to_string();
        while let Some(pos) = result.find("0o") {
            let end = pos + 2;
            let oct: String = result[end..]
                .chars()
                .take_while(|c| *c >= '0' && *c <= '7')
                .collect();

            if let Ok(v) = i64::from_str_radix(&oct, 8) {
                result.replace_range(pos..end + oct.len(), &v.to_string());
            } else {
                break;
            }
        }
        Ok(result)
    }

    fn process_power(expr: &str) -> Result<String, String> {
        let mut out = expr.to_string();

        while let Some(pos) = out.find('^') {
            let (l, r, start, end) = Self::extract_operands(&out, pos)?;

            let lv: f64 = eval_str(&l).map_err(|e| e.to_string())?;
            let rv: f64 = eval_str(&r).map_err(|e| e.to_string())?;

            let res = lv.powf(rv);
            out.replace_range(start..end, &res.to_string());
        }

        Ok(out)
    }

    fn extract_operands(s: &str, pos: usize) -> Result<(String, String, usize, usize), String> {
        let left = s[..pos].trim();
        let right = s[pos + 1..].trim();

        Ok((left.to_string(), right.to_string(), 0, s.len()))
    }
}
