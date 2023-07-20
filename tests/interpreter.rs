#[cfg(test)]
mod test {
    use wabbit::interpreter::Interpreter;
    use wabbit::parser::Parser;
    use wabbit::scanner::Scanner;
    use wabbit::WabbitType;

    fn expect_io(path: &str, expected: Vec<WabbitType>) {
        let source = std::fs::read_to_string(path).unwrap();
        let mut scanner = Scanner::new(&source);
        scanner.scan().unwrap();
        let mut parser = Parser::from(&scanner);
        parser.parse().unwrap();
        let mut interpreter = Interpreter::from(&parser);
        interpreter.interpret().unwrap();
        assert_eq!(interpreter.output, expected)
    }

    #[test]
    fn _00_intliteral() {
        let expected = vec![WabbitType::from(42)];
        expect_io("./program_examples/00_intliteral.wb", expected)
    }

    #[test]
    fn _01_intbinop() {
        let expected = vec![
            WabbitType::from(5),
            WabbitType::from(-1),
            WabbitType::from(6),
            WabbitType::from(2),
        ];
        expect_io("./program_examples/01_intbinop.wb", expected)
    }

    #[test]
    fn _02_intunaryop() {
        let expected = vec![WabbitType::from(-5), WabbitType::from(5)];
        expect_io("./program_examples/02_intunaryop.wb", expected)
    }

    #[test]
    fn _03_intvar() {
        let expected = vec![
            WabbitType::from(6),
            WabbitType::from(3),
            WabbitType::from(-1),
            WabbitType::from(12),
            WabbitType::from(3),
            WabbitType::from(1),
            WabbitType::from(-1),
            WabbitType::from(1),
            WabbitType::from(13),
        ];
        expect_io("./program_examples/03_intvar.wb", expected)
    }

    #[test]
    fn _04_floatliteral() {
        let expected = vec![WabbitType::from(4.2)];
        expect_io("./program_examples/04_floatliteral.wb", expected)
    }

    #[test]
    fn _05_floatbinop() {
        let expected = vec![
            WabbitType::from(5.0),
            WabbitType::from(-1.0),
            WabbitType::from(6.0),
            WabbitType::from(1.5),
        ];
        expect_io("./program_examples/05_floatbinop.wb", expected)
    }

    #[test]
    fn _06_floatunaryop() {
        let expected = vec![WabbitType::from(-5.0), WabbitType::from(5.0)];
        expect_io("./program_examples/06_floatunaryop.wb", expected)
    }

    #[test]
    fn _07_floatvar() {
        let expected = vec![
            WabbitType::from(6.0),
            WabbitType::from(3.0),
            WabbitType::from(-1.0),
            WabbitType::from(12.0),
            WabbitType::from(3.0),
            WabbitType::from(1.0),
            WabbitType::from(-1.0),
            WabbitType::from(13.0),
        ];
        expect_io("./program_examples/07_floatvar.wb", expected)
    }

    #[test]
    fn _08_intrel() {
        let expected = vec![
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
        ];
        expect_io("./program_examples/08_intrel.wb", expected)
    }

    #[test]
    fn _09_floatrel() {
        let expected = vec![
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
        ];
        expect_io("./program_examples/09_floatrel.wb", expected)
    }

    #[test]
    fn _10_bool() {
        let expected = vec![
            WabbitType::from(true),
            WabbitType::from(false),
            WabbitType::from(false),
            WabbitType::from(true),
            WabbitType::from(false),
            WabbitType::from(true),
            WabbitType::from(false),
        ];
        expect_io("./program_examples/10_bool.wb", expected)
    }

    #[test]
    fn _11_cond() {
        let expected = vec![WabbitType::from(3)];
        expect_io("./program_examples/11_cond.wb", expected)
    }

    #[test]
    fn _12_loop_test() {
        let expected = vec![
            WabbitType::from(1),
            WabbitType::from(2),
            WabbitType::from(6),
            WabbitType::from(24),
            WabbitType::from(120),
            WabbitType::from(720),
            WabbitType::from(5040),
            WabbitType::from(40320),
            WabbitType::from(362880),
            WabbitType::from(3628800),
        ];
        expect_io("./program_examples/12_loop.wb", expected)
    }

    #[test]
    fn _13_charliteral() {
        let expected = vec![
            WabbitType::from('h'),
            WabbitType::from('e'),
            WabbitType::from('l'),
            WabbitType::from('l'),
            WabbitType::from('o'),
            WabbitType::from('\n'),
            WabbitType::from('w'),
            WabbitType::from('o'),
            WabbitType::from('r'),
            WabbitType::from('l'),
            WabbitType::from('d'),
            WabbitType::from('\n'),
        ];
        expect_io("./program_examples/13_charliteral.wb", expected)
    }

    #[test]
    fn _14_charrel() {
        let expected = vec![
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
            WabbitType::from(true),
        ];
        expect_io("./program_examples/14_charrel.wb", expected)
    }

    #[test]
    fn _16_brk() {
        let expected = vec![
            WabbitType::from(1),
            WabbitType::from(2),
            WabbitType::from(3),
            WabbitType::from(4),
            WabbitType::from(6),
            WabbitType::from(7),
            WabbitType::from(8),
            WabbitType::from(9),
            WabbitType::from(10),
            WabbitType::from(11),
            WabbitType::from(-1),
        ];
        expect_io("./program_examples/16_brk.wb", expected)
    }

    #[test]
    fn _17_shortcircuit() {
        let expected = vec![WabbitType::from(true), WabbitType::from(false)];
        expect_io("./program_examples/17_shortcircuit.wb", expected)
    }

    #[test]
    fn _20_square() {
        let expected = (0..10).map(|i| WabbitType::from(i * i)).collect();
        expect_io("./program_examples/20_square.wb", expected)
    }

    #[test]
    #[ignore]
    fn _22_fib() {
        let expected = vec![
            WabbitType::from(1),
            WabbitType::from(1),
            WabbitType::from(2),
            WabbitType::from(3),
            WabbitType::from(5),
            WabbitType::from(8),
            WabbitType::from(13),
            WabbitType::from(21),
            WabbitType::from(34),
            WabbitType::from(55),
            WabbitType::from(89),
            WabbitType::from(144),
            WabbitType::from(233),
            WabbitType::from(377),
            WabbitType::from(610),
            WabbitType::from(987),
            WabbitType::from(1597),
            WabbitType::from(2584),
            WabbitType::from(4181),
            WabbitType::from(6765),
            WabbitType::from(10946),
            WabbitType::from(17711),
            WabbitType::from(28657),
            WabbitType::from(46368),
            WabbitType::from(75025),
            WabbitType::from(121393),
            WabbitType::from(196418),
            WabbitType::from(317811),
            WabbitType::from(514229),
            WabbitType::from(832040),
        ];
        expect_io("./program_examples/22_fib.wb", expected)
    }

    // Just running this to see that there aren't var name conflicts
    #[test]
    #[ignore]
    fn _23_mandel() {
        let source = std::fs::read_to_string("./program_examples/23_mandel.wb").unwrap();
        let mut scanner = Scanner::new(&source);
        scanner.scan().unwrap();
        let mut parser = Parser::from(&scanner);
        parser.parse().unwrap();
        let mut interpreter = Interpreter::from(&parser);
        interpreter.interpret().unwrap();
    }

    #[test]
    fn _24_conversions() {
        let expected = vec![
            WabbitType::from(43.234),
            WabbitType::from(43),
            WabbitType::from('*'),
            WabbitType::from(10),
        ];
        expect_io("./program_examples/24_conversions.wb", expected)
    }
}
