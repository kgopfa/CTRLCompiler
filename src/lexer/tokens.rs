#![allow(non_camel_case_types)]

#[derive(PartialEq)]
pub enum Token {
    EOI,
    SEMI,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    ASSIGN,
    LP,
    RP,
    NUM_OR_ID,
    EMPTY,
}