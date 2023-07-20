//! This is my implementation of Wabbit, written as part of David Beazley's excellent class on compilers.
//!
//! This crate provides a Wabbit interpreter, a typechecker, a compiler that uses LLVM IR,
//! and an export of the interpreter to WebAssembly.
//!
//! The below re-exports are roughly in the order that source code is processed (branching
//! depending on if you choose to interpret or compile).

#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(let_chains)]

pub mod analyzer;
pub mod error;
pub mod formatter;
pub mod interpreter;
pub mod llvm;
pub mod parser;
pub mod scanner;
pub mod typechecker;

mod ast;
mod environment;
mod operators;
mod tokens;
mod types;
mod wasm_interpreter;

// re-exporting for nice docs
pub use crate::scanner::Scanner;

pub use crate::parser::Parser;

pub use crate::analyzer::Analyzer;

pub use crate::typechecker::Typechecker;

pub use crate::interpreter::Interpreter;

pub use crate::llvm::CodegenLLVM;

pub use crate::error::*;
pub use crate::types::{Type, WabbitType};

#[macro_use]
extern crate lazy_static;

// after running wasm-pack build --target web, this generates the wasm page
#[allow(unused_imports)]
pub use crate::wasm_interpreter::*;
