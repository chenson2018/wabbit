use std::path::PathBuf;
use std::process::Command;

use clap::Parser as CliParser;
use main_error::MainError;

use wabbit::error::WabbitErrorReporter;
use wabbit::interpreter::Interpreter;
use wabbit::llvm::CodegenLLVM;
use wabbit::parser::Parser;
use wabbit::scanner::Scanner;
use wabbit::typechecker::Typechecker;

#[derive(CliParser, Debug)]
struct Cli {
    /// Option to print Tokens
    #[arg(short, long)]
    tokens: bool,

    /// Option to print AST
    #[arg(short, long)]
    ast: bool,

    /// Option to print formatted code
    #[arg(short, long)]
    format: bool,

    /// Option to skip typechecking
    #[arg(long)]
    skip_typecheck: bool,

    #[arg(long)]
    llvm_print: bool,

    #[arg(long)]
    llvm_exec: bool,

    /// Option to use interpreter
    #[arg(short, long)]
    interpret: bool,

    /// Path to Wabbit Program
    path: PathBuf,
}

// Potential improvements:
//  Scanner: handler errors for floats like 10.a with better error messages
//  Parser: handle multiple errors

fn main() -> Result<(), MainError> {
    let args = Cli::parse();
    let source = std::fs::read_to_string(&args.path)?;
    let mut scanner = Scanner::new(source.clone());

    if let Err(errs) = scanner.scan() {
        let error_report = WabbitErrorReporter::new(errs, args.path, source, "Scanner");
        return Err(error_report.into());
    }

    if args.tokens {
        println!("Tokens: \n\n{:#?}\n", scanner.borrow_tokens());
    }

    let mut parser = Parser::from(&scanner);

    if let Err(err) = parser.parse() {
        let error_report = WabbitErrorReporter::new(vec![err], args.path, source, "Parser");
        return Err(error_report.into());
    }

    if args.ast {
        println!("Statements: \n\n{:#?}", parser.borrow_statements());
    }

    if args.format {
        for stmt in parser.borrow_statements() {
            println!("{}", stmt);
        }
    }

    if !args.skip_typecheck {
        let mut typechecker = Typechecker::from(&parser);

        if let Err(err) = typechecker.typecheck() {
            let error_report =
                WabbitErrorReporter::new(vec![err], args.path, source, "Typechecking");
            return Err(error_report.into());
        }
    }

    if args.llvm_print || args.llvm_exec {
        let mut codegen = CodegenLLVM::from(&parser);
        let llvm = codegen.llvm_codegen();

        if args.llvm_print {
            println!("{}\n", llvm);
        }
        if args.llvm_exec {
            std::fs::write("./out.ll", llvm)?;
            let clang = Command::new("clang").args(["./out.ll"]).status()?;

            if clang.success() {
                let output = Command::new("./a.out").output()?;
                print!("{}", std::str::from_utf8(&output.stdout)?);
                std::fs::remove_file("./a.out")?;
                std::fs::remove_file("./out.ll")?;
            } else {
                std::fs::remove_file("./out.ll")?;
            }
        };
    }

    if args.interpret {
        let mut interpreter = Interpreter::from(&parser);
        println!("\nInterpreter output:\n");

        if let Err(err) = interpreter.interpret() {
            let error_report =
                WabbitErrorReporter::new(vec![err], args.path, source, "Interpreter");
            return Err(error_report.into());
        }
    }

    Ok(())
}
