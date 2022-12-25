use std::io::{self, BufRead};

// @ToDo: Find way to signal end of input as this is C specific

const EOI: i32 = 0;
const SEMI: i32 = 1;
const PLUS: i32 = 2;
const MINUS: i32 = 3;
const TIMES: i32 = 4;
const DIVIDE: i32 = 5;
const ASSIGN: i32 = 6;
const LP: i32 = 7;
const RP: i32 = 8;
const NUM_OR_ID: i32 = 9;


pub fn lex() -> i32 {
    let mut line_number: i32 = 0;
    let mut lexeme_length: i32 = 0;

    loop {
        let mut lexeme: String = String::new();
        let mut lexeme_start: i32 = 0;

        match io::stdin().lock().read_line(&mut lexeme) {
            Err(_error) => return EOI,
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
                        '=' => return ASSIGN,
                        ';' => return SEMI,
                        '+' => return PLUS,
                        '-' => return MINUS,
                        '*' => return TIMES,
                        '/' => return DIVIDE,
                        '(' => return LP,
                        ')' => return RP,
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
                                return NUM_OR_ID;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}