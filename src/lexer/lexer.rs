use std::io::{self, BufRead};
use std::str;

mod constants;
use constants::{EOI, SEMI, PLUS, TIMES, LP, RP, NUM_OR_ID};

fn lex(lexeme: String, lexeme_length: i32, line_number: i32) -> i32 {
    let mut input_buffer: String = String::new();
    let mut line_number: i32 = 0;

    loop {
        let mut input_buffer: String = String::new();
        let mut lexeme_start: i32 = 0;

        match io::stdin().lock().read_line(&mut input_buffer) {
            Err(error) => return EOI,
            Ok(lexeme) => {
                let line_length: i32 = lexeme.chars().count();
                line_number += 1;

                for (i, c) in lexeme.chars().enumerate() {
                    if !c.is_whitespace() {
                        lexeme_start = i;
                        break;
                    }
                }

                for i in lexeme_start..line_length-1 {
                    let b: u8 = lexeme.as_bytes()[i];
                    let current: char = b as char;

                    match current {
                        'EOF' => return EOI,
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
                            if !isalnum(current) {
                                eprintln!("Ignoring illegal input: <{}>", current);
                            } else {
                                // @ToDo: Re-evaluate this logic
                                while isalnum(current) {
                                    current += 1;
                                }
        
                                lexeme_length = current - LEXEME;
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