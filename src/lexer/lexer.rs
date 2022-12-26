use std::io::{ self, BufRead };
use crate::{ LEXEME_LENGTH, LINE_NUMBER, LEXEME_START };
use crate::lexer::tokens::Token;

pub fn lex() -> Token {
    loop {
        let mut lexeme: String = String::new();
        let mut lexeme_start: i32 = 0;

        match io::stdin().lock().read_line(&mut lexeme) {
            Err(_error) => return Token::EOI,
            Ok(_lexeme_bytes) => {
                let line_length: i32 = lexeme.chars().count() as i32;

                for (i, c) in lexeme.chars().enumerate() {
                    if !c.is_whitespace() {
                        lexeme_start = i as i32;
                        break;
                    }
                }

                unsafe {
                    LINE_NUMBER += 1;
                    LEXEME_START = lexeme_start;
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

                                unsafe {
                                    LEXEME_LENGTH = iter - lexeme_start;
                                }
        
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