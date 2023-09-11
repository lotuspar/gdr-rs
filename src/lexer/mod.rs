use std::str::Chars;
use string_interner::backend::StringBackend;
use string_interner::StringInterner;
use string_interner::symbol::SymbolU32;
use crate::lexer::core::error::Error;
use crate::lexer::core::token::{Token, TokenKind};
use crate::lexer::features::annotations::FEATURE_ANNOTATION;
use crate::lexer::features::comments::FEATURE_COMMENT;
use crate::lexer::features::strings::{FEATURE_LONG_STRING, FEATURE_SHORT_STRING, FEATURE_STRING};

pub mod core;
pub(crate) mod features;

pub struct Lexer<'a> {
    current_error: Error,
    current_token: Token,
    string_interner: StringInterner<StringBackend<SymbolU32>>,
    chars: Chars<'a>,
    chars_at_construct_time: Chars<'a>,
    source_length: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer {
        let source_length = chars.as_str().len();
        let chars_at_construct_time = chars.clone();
        Lexer {
            current_error: Error::empty(),
            current_token: Token::empty(),
            string_interner: StringInterner::default(),
            chars,
            chars_at_construct_time,
            source_length,
        }
    }

    /// Find and parse the next token from the input data
    pub fn parse(&mut self) -> bool {
        self.reset_error();
        self.reset_token();

        match self.peek() {
            Some(FEATURE_ANNOTATION) => self.annotation(),
            Some(FEATURE_COMMENT) => self.comment(),
            Some(FEATURE_STRING | FEATURE_SHORT_STRING) => self.string_literal(),
            _ => {}
        }

        match self.current_token.kind {
            TokenKind::None => false,
            _ => true,
        }
    }
}