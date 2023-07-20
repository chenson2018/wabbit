#[cfg(test)]
mod test {
    use dynfmt::{Format, SimpleCurlyFormat};
    use wabbit::error::Msg;
    use wabbit::parser::Parser;
    use wabbit::scanner::Scanner;
    use wabbit::typechecker::Typechecker;

    const RED_FRONT: &str = "\x1b[31m";
    const RED_BACK: &str = "\x1b[0m";

    // hacky, but don't feel like dealing with the owneship issue for just these tests
    fn red(s: &str) -> std::borrow::Cow<str> {
        SimpleCurlyFormat
            .format("{}{}{}", [RED_FRONT, s, RED_BACK])
            .expect("testing color error")
    }

    fn expect_err(source: &str, expected_error: Msg) {
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan().unwrap();
        let mut parser = Parser::from(&scanner);
        parser.parse().unwrap();
        let mut typechecker = Typechecker::from(&parser);
        if let Err(e) = typechecker.typecheck() {
            let colored_err = red(expected_error.msg());
            assert_eq!(e.label, colored_err)
        } else {
            panic!()
        }
    }

    fn expect_err_args(source: &str, expected_error: Msg, args: &[&str]) {
        let msg = SimpleCurlyFormat
            .format(expected_error.msg(), args)
            .expect("format error");

        let mut scanner = Scanner::new(source.to_string());
        scanner.scan().unwrap();
        let mut parser = Parser::from(&scanner);
        parser.parse().unwrap();
        let mut typechecker = Typechecker::from(&parser);
        if let Err(e) = typechecker.typecheck() {
            let colored_err = red(&msg);
            assert_eq!(e.label, colored_err)
        } else {
            panic!()
        }
    }

    #[test]
    fn alt_branch_1() {
        let source = "
            func u(x int) int {
                if x > 0 {
                    return 2;
                }
            }        
            ";
        expect_err(source, Msg::AltBranch)
    }

    #[test]
    fn alt_branch_2() {
        let source = "
            func u(x int) int {
                if x > 0 {
                    return 2;
                } else {
                    print 1;
                }
            }        
            ";
        expect_err(source, Msg::AltBranch)
    }

    #[test]
    fn dup_args() {
        let source = "
            func run(x int, x int) int {
                return x;
            }
            ";
        expect_err(source, Msg::DupArgs)
    }

    #[test]
    fn func_def_scope() {
        let source = "
            if true {
                func run() int {
                    return 0;
                }
            };";
        expect_err(source, Msg::FuncDefScope)
    }

    #[test]
    fn const_scope() {
        let source = "
            if true {
                const pi = 3.14;
            };";
        expect_err(source, Msg::ConstScope)
    }

    #[test]
    fn return_scope() {
        let source = "return 0;\n";
        expect_err(source, Msg::ReturnScope)
    }

    #[test]
    fn loop_req() {
        expect_err("break;\n", Msg::LoopReq);
        expect_err("continue;\n", Msg::LoopReq);
    }

    #[test]
    fn assign_retype() {
        let source = "
            var x int = 1;
            x = 1.0;
        ";
        expect_err_args(source, Msg::AssignRetype, &["x", "int", "float"])
    }

    #[test]
    fn var_undefined() {
        expect_err("x;\n", Msg::VarUndefined);
    }

    #[test]
    fn func_undefined() {
        expect_err("x();\n", Msg::FuncUndefined);
    }

    #[test]
    fn assign_undefined() {
        expect_err("x = 1;\n", Msg::AssignUndefined);
    }

    #[test]
    fn init_type() {
        expect_err("var x int = 1.0;\n", Msg::InitType);
    }

    #[test]
    fn type_convert() {
        expect_err("bool(1.0);\n", Msg::TypeConvert);
        expect_err("bool('c');\n", Msg::TypeConvert);
        expect_err("float('c');\n", Msg::TypeConvert);
        expect_err("float(true);\n", Msg::TypeConvert);
    }

    #[test]
    fn convert_airty() {
        expect_err("int(1.0, 2.0);\n", Msg::ConvertAirty);
    }

    #[test]
    fn func_airty() {
        let source = "
            func run() int {
                return 0;
            }

            run(1);
            ";
        expect_err_args(source, Msg::FuncAirty, &["run", "0", "1"])
    }

    #[test]
    fn param_type() {
        let source = "
            func self(x int) int {
                return x;
            }

            self(1.0);
            ";
        expect_err_args(source, Msg::ParamType, &["x", "int", "float"])
    }

    #[test]
    fn return_type() {
        let source = "
            func self(x int) int {
                return float(x);
            }
            ";
        expect_err_args(source, Msg::ReturnType, &["self", "int", "float"])
    }

    #[test]
    fn no_return() {
        let source = "
            func hi() int {
                print 'h';
                print 'i';
                print '\n';
            }
            ";
        expect_err(source, Msg::NoReturn)
    }

    #[test]
    fn type_eval() {
        let source = "print int;\n";
        expect_err(source, Msg::TypeEval)
    }

    #[test]
    fn access_uninit() {
        let source = "
            var x int;
            print x;
            ";
        expect_err_args(source, Msg::AccessUninit, &["x"])
    }

    #[test]
    fn expect_type() {
        let source = "
            if 1 {
                print 1;
            }
            ";
        expect_err_args(source, Msg::ExpectType, &["bool"])
    }

    #[test]
    fn type_match() {
        let source = "1 + 1.0;\n";
        expect_err(source, Msg::TypeMatch)
    }

    #[test]
    fn return_diverge() {
        let source = "
            func run() int {
                if true  {
                    return true;
                } else {
                    return 1;
                }
            }
            ";
        expect_err(source, Msg::ReturnDiverge)
    }

    #[test]
    fn redeclare_var() {
        let source = "
            var x int = 1;
            var x int = 2;
        ";
        expect_err_args(source, Msg::RedeclareVar, &["x"])
    }

    #[test]
    fn redeclare_const() {
        let source = "
            const x int = 1;
            const x int = 2;
        ";
        expect_err_args(source, Msg::RedeclareConst, &["x"])
    }

    #[test]
    fn redeclare_func() {
        let source = "
            func x() int { 
                return 0; 
            }
            
            func x() int { 
                return 0; 
            }
        ";
        expect_err_args(source, Msg::RedeclareFunc, &["x"])
    }

    #[test]
    fn typecheck_programs() {
        let paths = std::fs::read_dir("./program_examples/").unwrap();

        for file in paths {
            let source = std::fs::read_to_string(file.unwrap().path()).unwrap();
            println!("{source}");
            let mut scanner = Scanner::new(source);
            scanner.scan().unwrap();
            let mut parser = Parser::from(&scanner);
            parser.parse().unwrap();
            let mut typechecker = Typechecker::from(&parser);
            typechecker.typecheck().unwrap();
        }
    }
}
