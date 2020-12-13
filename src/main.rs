use std::process;

fn main() {
    let err = match expense_categorizer::run() {
        Err(it) => it,
        _ => return,
    };
    println!("error running example: {}", err);
    process::exit(1);
}
