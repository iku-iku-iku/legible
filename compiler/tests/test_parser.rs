#[cfg(test)]
mod test_parser {

    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub legible);

    #[cfg(test)]
    mod test_title {
        use super::legible;

        #[test]
        fn test_title_level() {
            let input = "###";
            let res = legible::TitleParser::new().parse(input);
            assert_eq!(3, *res.unwrap().level());
        }

        fn title_content(input: &str, content: &str) {
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

    // mod test_comment {
    //     use super::legible;

    //     #[test]
    //     fn test_comment() {
    //         let input = "> hello world";
    //         let res = legible::CommentParser::new().parse(input);
    //         println!("{:?}", res);
    //     }
    // }

    #[cfg(test)]
    mod test_block {
        use super::legible;

        #[test]
        fn test_block_title() {
            let input = "### hello";
            let res = legible::BlockParser::new().parse(input);
            assert_eq!(input, format!("{:?}", res.unwrap().title()));
        }

        #[test]
        fn test_orderedlist() {
            let input = "03.`xx`";

            let res = legible::OrderedListElementParser::new().parse(input);
            let s = res.unwrap();

            println!("{:?}", s);
        }

        #[test]
        fn test_block_input() {
            let input = 
            r#"### add
1. `a`
2. `b`
            "#;
            // let res = legible::BlockParser::new().parse(input);
            println!("{}", input);
        }
    }
}
