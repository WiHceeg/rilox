use std::io;
use thiserror;

use crate::object::Object;

#[derive(thiserror::Error, Debug)]
pub enum LoxErr {
    #[error("Usage: rilox [script]")]
    ScriptUsage,

    #[error("Io Error from: {0}")]
    Io(#[from] io::Error),

    #[error("Scan Error: [line {line}] {message}")]
    Scan{
        line: usize,
        message: String,
    },

    #[error("Parse Error: [line {line}] at {lexeme}. {message}")]
    Parse{
        line: usize,
        lexeme: String,
        message: String,
    },

    #[error("Runtime Error: [line {line}] {message}")]
    Runtime{
        line: usize,
        message: String,
    },

    // Java 原版用异常实现 Return
    #[error("RuntimeReturn: {ret_value}")]
    RuntimeReturn {
        ret_value: Object,
    },

    #[error("Resolve Error: [line {line}] {message}")]
    Resolve{
        line: usize,
        message: String,
    },

    #[error("Multiple errors occurred: {0:?}")]
    Many(Vec<LoxErr>),
}
