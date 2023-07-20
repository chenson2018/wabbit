use std::path::PathBuf;
use std::process::Command;

use clap::Parser as CliParser;
use main_error::MainError;

use wabbit::{CodegenLLVM, Interpreter, Parser, Scanner, Typechecker, WabbitErrorReporter};

/// command-line options

#[derive(CliParser, Debug)]
pub struct Cli {
    /// option to print tokens
    #[arg(short, long)]
    tokens: bool,

    /// option to print AST
    #[arg(short, long)]
    ast: bool,

    /// option to print minimized code
    #[arg(short, long)]
    minimize: bool,

    /// option to skip typechecking
    #[arg(long)]
    skip_typecheck: bool,

    /// option to print LLVM IR
    #[arg(long)]
    llvm_print: bool,

    /// option to compile and execute LLVM IR (using clang)
    #[arg(long)]
    llvm_exec: bool,

    /// option to use interpreter
    #[arg(short, long)]
    interpret: bool,

    /// path to Wabbit program
    path: PathBuf,
}

// Potential improvements:
//  Scanner: handler errors for floats like 10.a with better error messages
//  Parser: handle multiple errors

fn main() -> Result<(), MainError> {
    let args = Cli::parse();
    let source = std::fs::read_to_string(&args.path)?;
    let mut scanner = Scanner::new(&source);

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

    if args.minimize {
        for stmt in parser.borrow_statements() {
            println!("{stmt}");
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
            println!("{llvm}\n");
        }
        if args.llvm_exec {
            std::fs::write("./out.ll", llvm)?;
            let clang = Command::new("clang").args(["./out.ll"]).status()?;

            if clang.success() {
                let output = Command::new("./a.out").output()?;
                print!("{}", std::str::from_utf8(&output.stdout)?);
                std::fs::remove_file("./a.out")?;
            }
            std::fs::remove_file("./out.ll")?;
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
