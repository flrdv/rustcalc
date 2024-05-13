use std::ops::Deref;

mod lex;
mod parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expr = String::from("(1+2)**3");
    let mut lexer = lex::Lexer::new(expr.deref());
    let lexemes = lexer.lex();
    // for lexeme in &lexemes.unwrap() {
    //     println!("lexeme: {lexeme:?}")
    // }

    let stream = parse::stream::Stream::new(lexemes.unwrap());
    let mut parser = parse::Parser::new(stream);
    let branches = parser.parse();

    for node in branches {
        println!("{:?}", node)
    }

    Ok(())
}
