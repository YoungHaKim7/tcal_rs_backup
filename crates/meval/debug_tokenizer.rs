fn main() {
    let s = "-2^(4 - 3) * (3 + 4)";
    println!("String: {}", s);
    println!("Length: {}", s.len());
    for (i, c) in s.chars().enumerate() {
        println!("Position {}: '{}'", i, c);
    }
}
