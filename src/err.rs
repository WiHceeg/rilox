use std::io;

use thiserror;


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

    #[error("Parse Error: [line {line}] {message}")]
    Parse{
        line: usize,
        message: String,
    },

    #[error("Runtime Error: [line {line}] {message}")]
    Runtime{
        line: usize,
        message: String,
    },

    #[error("Resolve Error: [line {line}] {message}")]
    Resolve{
        line: usize,
        message: String,
    },

    #[error("Multiple errors occurred: {0:?}")]
    Many(Vec<LoxErr>),
}
