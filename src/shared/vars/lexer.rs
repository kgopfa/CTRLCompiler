use std::sync::{Arc, Mutex};
use std::str;

lazy_static! {
    static ref LEXEME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref LEXEME_LENGTH: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref LINE_NUMBER: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}