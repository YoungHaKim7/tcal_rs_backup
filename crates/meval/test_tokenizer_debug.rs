// Simple test to understand the tokenizer state machine
fn main() {
    let s = "2 +(3--2) ";
    println!("String: '{}'", s);
    println!("Length: {}", s.len());
    for (i, c) in s.chars().enumerate() {
        println!("Position {}: '{}' (byte: {})", i, c, c as u32);
    }
    
    // What should happen:
    println!("\nExpected tokenization:");
    println!("1. Position 0: '2' -> Number(2.0), state: AfterRExpr");
    println!("2. Position 1: ' ' -> skip whitespace");
    println!("3. Position 2: '+' -> Binary(Plus), state: LExpr");
    println!("4. Position 3: '(' -> LParen, state: LExpr");
    println!("5. Position 4: '3' -> Number(3.0), state: AfterRExpr");
    println!("6. Position 5: '-' -> Binary(Minus), state: LExpr");
    println!("7. Position 6: '-' -> Unary(Minus), state: LExpr");
    println!("8. Position 7: '2' -> Number(2.0), state: AfterRExpr");
    println!("9. Position 8: ')' -> RParen, state: AfterRExpr");
    println!("10. Position 9: ' ' -> skip whitespace");
    println!("11. Position 10: end of string -> Done!");
}
