use core::str;
use malloced::Malloced;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct CompileError {
    pub(crate) buffer: Malloced<[u8]>,
}

impl CompileError {
    pub fn message(&self) -> &str {
        match str::from_utf8(&self.buffer[2..]) {
            Ok(s) => s,
            Err(e) => panic!("Compile error not valid utf8: {e}"),
        }
    }
}

impl Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "luau compile error: {}", self.message())
    }
}

impl Error for CompileError {}
