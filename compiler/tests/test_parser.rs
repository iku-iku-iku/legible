#[cfg(test)]
mod test_parser {

    use compiler::lexer;
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub legible);

    #[cfg(test)]
    mod test_title {
        use super::legible;
        use super::lexer;

        #[test]
        fn test_title_level() {
            let input = "### ";
            let input = lexer::Lexer::new(input);
            let res = legible::TitleParser::new().parse(input);
            assert_eq!(3, *res.unwrap().level());
        }

        fn title_content(input: &str, content: &str) {
            let input = lexer::Lexer::new(input);
            let res = legible::TitleParser::new().parse(input);
            assert_eq!(res.unwrap().content(), content);
        }

        #[test]
        fn test_title_content() {
            title_content("### hello\n", "hello");
            title_content("### hello", "hello");
            title_content("### hello world", "hello world");
            title_content("### `hello world`", "`hello world`");
            title_content("### #hello#world", "#hello#world");
        }
    }

    #[cfg(test)]
    mod test_ordered_list {
        use super::legible;
        use super::lexer;

        #[test]
        fn test_ordered_list_element() {
            let input = "1. `a`";

            let input = lexer::Lexer::new(input);
            let res = legible::OrderedListElementParser::new().parse(input);
            let s = res.unwrap();

            assert_eq!(s, "a");
        }

        #[test]
        fn test_arg() {
            let input = "1. `a`";

            let input = lexer::Lexer::new(input);
            let res = legible::InputArgParser::new().parse(input);
            let s = res.unwrap();

            assert_eq!(format!("{:?}", s), "`a`");
        }
    }

    #[cfg(test)]
    mod test_block {
        use super::legible;
        use super::lexer;

        #[test]
        fn test_block_title() {
            let input = "### hello";
            let l = lexer::Lexer::new(input);
            let res = legible::BlockParser::new().parse(l);
            assert_eq!(input, format!("{:?}", res.unwrap().title()));
        }
    }

    mod test_template {
        use super::legible;
        use super::lexer;

        #[test]
        fn test_template0() {
            let input = "hello";
            let l = lexer::Lexer::new(input);
            let res = legible::TemplateParser::new().parse(l);
            let res = res.unwrap();
            assert_eq!(res.0, vec!["hello"]);
            assert!(res.1.is_empty());
        }

        #[test]
        fn test_template1() {
            let input = "hello `num` world";
            let l = lexer::Lexer::new(input);
            let res = legible::TemplateParser::new().parse(l);
            let res = res.unwrap();
            assert_eq!(res.0, vec!["hello", "world"]);
            assert_eq!(res.1, vec![("num".to_string(), 1)]);
        }

        #[test]
        fn test_template2() {
            let input = "`num` world";
            let l = lexer::Lexer::new(input);
            let res = legible::TemplateParser::new().parse(l);
            let res = res.unwrap();
            assert_eq!(res.0, vec!["world"]);
            assert_eq!(res.1, vec![("num".to_string(), 0)]);
        }
    }
}

//         #[test]
//         fn test_args() {
//             let input =
//             r#"1. `a` is first operand
// 2. `b` is second operand
// "#;
//             let input = lexer::Lexer::new(input);
//             let res = legible::InputArgsParser::new().parse(input);
//             let s = res.unwrap();

//             assert_eq!(format!("{:?}", s), "[`a`, `b`]");
//         }

//         #[test]
//         fn test_block_input() {
//             let input =
//             r#"### add
// 1. `a`
// 2. `b`"#;
//             let input_var =
//             r#"### add
// 1. `a` is first operand
// 2. `b` is second operand"#;
//             let l = lexer::Lexer::new(input);
//             let res = legible::BlockParser::new().parse(l);
//             assert_eq!(input, format!("{:?}", res.unwrap()));

//             let l = lexer::Lexer::new(input_var);
//             let res = legible::BlockParser::new().parse(l);
//             assert_eq!(input, format!("{:?}", res.unwrap()));
//         }

//         #[test]
//         fn test_expression() {
//             // TODO: implement this
//             let input = "1+1";
//             let input = lexer::Lexer::new(input);
//             let res = legible::ExprParser::new().parse(input);
//             println!("{:?}", res);
//         }

//         #[test]
//         fn test_block_output() {
//             let input =
//             r#"### add
// - a: 1 + 1
// + b: 1 + (1 * 2)"#;
//         }
