use rustyline::{DefaultEditor, config::Configurer};

mod calculator;
mod fprice;

use calculator::engine::Calculator;

fn main() -> rustyline::Result<()> {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'exit' or 'quit' to exit\n");

    let mut rl = DefaultEditor::new()?;
    let mut calc = Calculator::new();

    loop {
        let input = rl.readline("> ")?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        match calc.evaluate(input) {
            Ok(output) => println!("{}", output),
            Err(e) => println!("Error: {}", e),
        }

        // ⭐ IMPORTANT: store history
        rl.add_history_entry(input)?;
    }

    // Save history
    let _ = rl.save_history("history.txt");
    let _ = rl.set_max_history_size(1000);

    Ok(())
}
