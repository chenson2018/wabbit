#[cfg(test)]
mod test {
    use dynfmt::{Format, SimpleCurlyFormat};
    use wabbit::error::Msg;
    use wabbit::parser::Parser;
    use wabbit::scanner::Scanner;

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
        if let Err(e) = parser.parse() {
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
        if let Err(e) = parser.parse() {
            let colored_err = red(&msg);
            assert_eq!(e.label, colored_err)
        } else {
            panic!()
        }
    }

    #[test]
    fn vardef_empty() {
        expect_err("var x;\n", Msg::VarDefEmpty);
    }

    #[test]
    fn expect_expr() {
        expect_err("print;\n", Msg::ExpectExpr);
    }

    #[test]
    fn expect_typename() {
        expect_err(
            "
                   func run() {
                     return 0;
                   }
                   ",
            Msg::ExpectTypeName,
        );
    }

    #[test]
    fn expect_varname() {
        expect_err("var int x = 1;\n", Msg::ExpectVarName);
    }

    // This one is very general...
    #[test]
    fn parser_expect() {
        expect_err_args("print 1\n", Msg::ParserExpect, &[";"])
    }

    // Just checking that it doesn't throw an error, not if it is correct
    #[test]
    fn parse_programs() {
        let paths = std::fs::read_dir("./program_examples/").unwrap();

        for file in paths {
            let source = std::fs::read_to_string(file.unwrap().path()).unwrap();
            let mut scanner = Scanner::new(source);
            scanner.scan().unwrap();
            let mut parser = Parser::from(&scanner);
            parser.parse().unwrap();
        }
    }
}
