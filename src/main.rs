mod lexer;

static mut LINE_NUMBER: i32 = 0;
static mut LEXEME_LENGTH: i32 = 0;
static mut LEXEME_START: i32 = 0;

fn main() {
    lexer::lexer::lex();

    // For testing
    /*
    unsafe {
        println!("LEXEME START {}", LEXEME_START);
        println!("LEXEME LENGTH {}", LEXEME_LENGTH);
        println!("LINE_NUMBER {}", LINE_NUMBER);
    }
    */
}