use std::ops::Deref;

mod lex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expr = String::from("1+2");
    let mut lexer = lex::Lexer::new(expr.deref());
    let lexemes = lexer.lex();
    for lexeme in lexemes.unwrap() {
        println!("lexeme: {lexeme:?}")
    }

    println!("done");

    Ok(())
}
