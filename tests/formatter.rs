#[cfg(test)]
mod test {
    use wabbit::parser::Parser;
    use wabbit::scanner::Scanner;

    #[test]
    fn format_programs() {
        let paths = std::fs::read_dir("./program_examples/").unwrap();

        for file in paths {
            let source = std::fs::read_to_string(file.unwrap().path()).unwrap();
            let mut scanner = Scanner::new(source.clone());
            scanner.scan().unwrap();
            let mut parser = Parser::from(&scanner);
            parser.parse().unwrap();

            let ast_from_og_source = parser.borrow_statements();
            let source_from_formatter = format!(
                "{}\n",
                ast_from_og_source
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            );

            let mut scanner2 = Scanner::new(source_from_formatter);
            scanner2.scan().unwrap();
            let mut parser2 = Parser::from(&scanner2);
            parser2.parse().unwrap();

            let ast_roundtrip = parser2.borrow_statements();
            assert_eq!(ast_from_og_source, ast_roundtrip);
        }
    }
}
