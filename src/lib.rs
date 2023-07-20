//! This is my implementation of Wabbit, written as part of David Beazley's excellent class on compilers.
//!
//! This crate provides a Wabbit interpreter, a typechecker, a compiler that uses LLVM IR,
//! and an export of the interpreter to `WebAssembly`.
//!
//! The below re-exports are roughly in the order that source code is processed (branching
//! depending on if you choose to interpret or compile).
//!
//! See [`wabbit_rs`](../wabbit_rs/index.html) for command-line usage

#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(let_chains)]

// roughly grouped by category
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::if_not_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::float_cmp)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

/// typecheck or interpret Wabbit AST
pub mod analyzer;
/// error reporting
pub mod error;
/// a code minimizer
pub mod formatter;
/// interpret Wabbit AST
pub mod interpreter;
/// generate LLVM IR
pub mod llvm;
/// parse Wabbit tokens
pub mod parser;
/// scan Wabbit source code
pub mod scanner;
/// interpret Wabbit AST
pub mod typechecker;

/// types for Wabbit AST
mod ast;
/// manage variable scope
mod environment;
/// Wabbit primitive operators
mod operators;
/// Wabbit tokens
mod tokens;
/// Wabbit types and values
mod types;
/// a `WebAssembly` interpreter
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

#[allow(unused_imports)]
pub use crate::wasm_interpreter::*;
