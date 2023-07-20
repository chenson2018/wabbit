#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(let_chains)]

pub mod error;
pub mod formatter;
pub mod interpreter;
pub mod llvm;
pub mod parser;
pub mod scanner;
pub mod typechecker;

mod analyzer;
mod ast;
mod environment;
mod operators;
mod tokens;
mod types;
mod wasm_interpreter;
pub use crate::types::WabbitType;

#[macro_use]
extern crate lazy_static;

// after running wasm-pack build --target web, this generates the wasm page
#[allow(unused_imports)]
use crate::wasm_interpreter::wasm_entry;
