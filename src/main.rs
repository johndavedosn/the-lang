use random::lexer::lex;

// A test before I make the CLI interface.
fn main() {
    let input = "if foo 42 -3.14 -5 true false +6";
    let tokens = lex(&input);
    for t in tokens {
        println!("{t:?}");
    }
}
