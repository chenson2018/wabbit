#[cfg(test)]
mod test {
    use dynfmt::{Format, SimpleCurlyFormat};
    use wabbit::error::Msg;
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
        if let Err(errs) = scanner.scan() {
            let colored_err = red(expected_error.msg());
            assert_eq!(errs[0].label, colored_err)
        } else {
            panic!()
        }
    }

    fn expect_err_args(source: &str, expected_error: Msg, args: &[&str]) {
        let msg = SimpleCurlyFormat
            .format(expected_error.msg(), args)
            .expect("format error");

        let mut scanner = Scanner::new(source.to_string());
        if let Err(errs) = scanner.scan() {
            match errs.as_slice() {
                [err] => {
                    let colored_err = red(&msg);
                    assert_eq!(err.label, colored_err)
                }
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }

    #[test]
    fn invalid_number() {
        expect_err_args("10.a;\n", Msg::InvalidNumber, &["."])
    }

    #[test]
    fn invalid_char() {
        expect_err("'abc';\n", Msg::InvalidChar)
    }

    #[test]
    fn double_token() {
        expect_err_args("true | false;\n", Msg::DoubleToken, &["|", "|"])
    }

    #[test]
    fn unexpected_char() {
        expect_err_args("@;\n", Msg::UnexpectedChar, &["@"])
    }

    // Just checking that it doesn't throw an error, not if it is correct
    #[test]
    fn scan_programs() {
        let paths = std::fs::read_dir("./program_examples/").unwrap();

        for file in paths {
            let source = std::fs::read_to_string(file.unwrap().path()).unwrap();
            let mut scanner = Scanner::new(source);
            scanner.scan().unwrap()
        }
    }
}
