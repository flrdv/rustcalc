use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use crate::eval::Evaluator;
use crate::lex::error::Error;
use crate::parse::stream::Stream;

mod lex;
mod parse;
mod eval;

fn exec(data: &str) -> Result<Vec<f64>, Error> {
    let mut lexer = lex::Lexer::new(data);
    let lexemes = lexer.lex()?;
    let mut parser = parse::Parser::new(Stream::new(lexemes));
    let eval = Evaluator::new()
        .names(HashMap::from([
            ("pi".to_string(), std::f64::consts::PI),
        ]))
        .functions(HashMap::from([
            ("five".to_string(), Box::new(|_: Vec<f64>| 5f64) as _),
            ("sum".to_string(), Box::new(|args: Vec<f64>| args.iter().sum()) as _)
        ]));
    let mut results = Vec::new();

    for branch in parser.parse() {
        results.push(eval.evaluate(&branch)?)
    }

    Ok(results)
}

fn print(text: &[u8]) {
    let mut stdout = io::stdout();
    stdout.write(text).expect("failed to read from stdout");
    stdout.flush().unwrap();
}

fn repl() {
    const PROMPT: &[u8] = "> ".as_bytes();
    print(PROMPT);

    for line in io::stdin().lock().lines() {
        match exec(line.unwrap().as_str()) {
            Ok(results) => {
                for result in results {
                    println!("{result}")
                }
            },
            Err(err) => println!("error: {err}")
        };

        print(PROMPT)
    }
}

fn main() {
    repl()
}
