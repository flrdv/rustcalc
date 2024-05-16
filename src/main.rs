use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::ops::Deref;
use crate::eval::Evaluator;
use crate::lex::error::Error;
use crate::parse::stream::Stream;

mod lex;
mod parse;
mod eval;

fn repl() {
    const PROMPT: &str = "> ";
    print!("{}", PROMPT);

    for line in io::stdin().lock().lines() {
        match exec(line.unwrap().as_str()) {
            Ok(results) => {
                for result in results {
                    println!("{result}")
                }
            },
            Err(err) => println!("error: {err}")
        };

        print!("{}", PROMPT)
    }
}

fn exec(data: &str) -> Result<Vec<f64>, Error> {
    let mut lexer = lex::Lexer::new(data);
    let lexemes = lexer.lex()?;
    let mut parser = parse::Parser::new(Stream::new(lexemes));
    let eval = Evaluator::new(HashMap::from([
        ("pi".to_string(), std::f64::consts::PI),
    ]));
    let mut results = Vec::new();

    for branch in parser.parse() {
        results.push(eval.evaluate(&branch)?)
    }

    Ok(results)
}

fn main() {
    for result in exec("pi").unwrap() {
        println!("{result}")
    }
    // repl()
}
