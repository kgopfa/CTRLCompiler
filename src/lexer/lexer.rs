use std::io;
use std::sync::{Arc, Mutex};
use std::str;

mod constants;
use constants::{EOI, SEMI, PLUS, TIMES, LP, RP, NUM_OR_ID};

include!("../shared/vars/lexers.rs");

lazy_static! {
    // Initialize the shared variables
    static ref LEXEME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref LEXEME_LENGTH: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref LINE_NUMBER: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

fn lex() -> i32 {
    let mut input_buffer = String::new();
    let mut current;

    current = LEXEME + LEXEME_LENGTH;

    loop {
        while !*current {
            current = &input_buffer;
            if !io::stdin().read_line(&mut input_buffer).unwrap() {
                *current = '\0';
                return EOI;
            }
            LINE_NUMBER += 1;

            while is_space(*current) {
                current += 1;
            }
        }

        while *current {
            LEXEME = current;
            LEXEME_LENGTH = 1;

            match *current {
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
                    if !isalnum(*current) {
                        eprintln!("Ignoring illegal input: <{}>", *current);
                    } else {
                        while isalnum(*current) {
                            current += 1;
                        }

                        LEXEME_LENGTH = current - LEXEME;
                        return NUM_OR_ID;
                    }

                    break;
                }
            }
        }
    }
}