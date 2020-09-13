use std::process;

fn main() {
    if let Err(err) = expense_categorizer::run() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
