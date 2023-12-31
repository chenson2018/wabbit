use wasm_bindgen::prelude::*;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::typechecker::Typechecker;

/// entry point for WebAssembly interpreter
///
/// to view in your browser, run `wasm-pack build --target web` and then open [index.html](https://github.com/chenson2018/wabbit/blob/main/index.html)
#[wasm_bindgen(start)]
pub fn wasm_entry() -> Result<(), JsValue> {
    Ok(())
}

/// a Wabbit interpreter, exported to WebAssembly
///
/// note that this is the entire Rust implementation of the scanner, parser and interpreter, not a
/// use of WebAssembly as a compilation target
#[wasm_bindgen]
pub fn wasm_interp(source: &str) -> String {
    let mut scanner = Scanner::new(source);

    // doing simplified error reporting here...
    // this also has the terminal color codes in the error strings

    if let Err(errs) = scanner.scan() {
        return errs[0].label.clone();
    }

    let mut parser = Parser::from(&scanner);

    if let Err(e) = parser.parse() {
        return e.label;
    }

    let mut typechecker = Typechecker::from(&parser);

    if let Err(e) = typechecker.typecheck() {
        return e.label;
    }

    let mut interpreter = Interpreter::from(&parser);

    if let Err(e) = interpreter.interpret() {
        return e.label;
    }

    interpreter
        .output
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

// See https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
// for how to log to console
