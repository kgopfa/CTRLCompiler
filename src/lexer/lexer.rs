use std::io::{self, BufRead};
use crate::lexer::tokens::Token;

pub fn lex() -> Token {
    let mut line_number: i32 = 0;
    let mut lexeme_length: i32 = 0;

    loop {
        let mut lexeme: String = String::new();
        let mut lexeme_start: i32 = 0;

        match io::stdin().lock().read_line(&mut lexeme) {
            // @ToDo: Find way to signal end of input as this is C specific
            Err(_error) => return Token::EOI,
            Ok(_lexeme_bytes) => {
                let line_length: i32 = lexeme.chars().count() as i32;
                line_number += 1;

                for (i, c) in lexeme.chars().enumerate() {
                    if !c.is_whitespace() {
                        lexeme_start = i as i32;
                        break;
                    }
                }

                let mut iter = lexeme_start;

                while iter < line_length {
                    let mut current: char = lexeme.as_bytes()[iter as usize] as char;

                    match current {
                        '=' => return Token::ASSIGN,
                        ';' => return Token::SEMI,
                        '+' => return Token::PLUS,
                        '-' => return Token::MINUS,
                        '*' => return Token::TIMES,
                        '/' => return Token::DIVIDE,
                        '(' => return Token::LP,
                        ')' => return Token::RP,
                        '\n' => break,
                        '\t' => break,
                        ' ' => break,
                        _ => {
                            if !current.is_alphanumeric() {
                                eprintln!("Ignoring illegal input: <{}>", current);
                            } else {
                                while current.is_alphanumeric() && iter < line_length {
                                    current = lexeme.as_bytes()[iter as usize] as char;
                                    iter += 1;
                                }
        
                                lexeme_length = iter - lexeme_start;
                                return Token::NUM_OR_ID;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}