extern crate dialasm;

#[cfg(test)]
mod rules {
    use dialasm::{DialasmParser, Rule};
    use pest::Parser;

    use super::*;

    mod identifier {
        use super::*;

        #[test]
        fn lowercase_letter_identifier_valid() {
            const INPUT: &str = "a";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn uppercase_letter_identifier_valid() {
            const INPUT: &str = "A";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn underscore_identifier_valid() {
            const INPUT: &str = "_";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn digit_identifier_invalid() {
            const INPUT: &str = "2";
            DialasmParser::parse(Rule::identifier, INPUT)
                .expect_err(&format!("Identifier can't be valid if it's just a digit"));
        }

        #[test]
        fn all_lowercase_identifier_valid() {
            const INPUT: &str = "abcdefg";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn all_uppercase_identifier_valid() {
            const INPUT: &str = "ABCDEFG";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn all_underscore_identifier_valid() {
            const INPUT: &str = "_______";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn all_digits_identifier_invalid() {
            const INPUT: &str = "1265322";
            DialasmParser::parse(Rule::identifier, INPUT).expect_err(&format!(
                "Identifier can't be valid if it's a set of digits"
            ));
        }

        #[test]
        fn mixed_case_identifier_valid() {
            const INPUT: &str = "aCggW";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn mixed_case_and_underscore_identifier_valid() {
            const INPUT: &str = "_aC__ggW";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn mixed_identifier_valid() {
            const INPUT: &str = "_aC_55_gg2W";
            let pairs = DialasmParser::parse(Rule::identifier, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Inputs are not equal: expected {}, got {}",
                INPUT, output
            );
        }

        #[test]
        fn mixed_first_digit_identifier_invalid() {
            const INPUT: &str = "12aGG3asdg3";
            DialasmParser::parse(Rule::identifier, INPUT).expect_err(&format!(
                "Identifier can't be valid if it starts with digits"
            ));
        }
    }

    mod string_literal {
        use super::*;

        #[test]
        fn simple_string_literal_valid() {
            const INPUT: &str = "\"Hello, world!\"";
            let pairs = DialasmParser::parse(Rule::string_literal, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let string = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                string,
                &INPUT[1..INPUT.len() - 1],
                "Strings are not equal: expected {}, got {}",
                &INPUT[1..INPUT.len() - 1],
                string
            );
        }

        #[test]
        fn newline_string_literal_valid() {
            const INPUT: &str = "\"Hello,\\nworld!\"";
            let pairs = DialasmParser::parse(Rule::string_literal, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let string = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                string,
                &INPUT[1..INPUT.len() - 1],
                "Strings are not equal: expected {}, got {}",
                &INPUT[1..INPUT.len() - 1],
                string
            );
        }

        #[test]
        fn string_literal_with_quotes_valid() {
            const INPUT: &str = "\"Hello,\\\"world!\"";
            let pairs = DialasmParser::parse(Rule::string_literal, INPUT)
                .expect(&format!("Expected to parse valid identifier {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let string = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                string,
                &INPUT[1..INPUT.len() - 1],
                "Strings are not equal: expected {}, got {}",
                &INPUT[1..INPUT.len() - 1],
                string
            );
        }
    }

    mod handle {
        use super::*;

        #[test]
        fn empty_handle_invalid() {
            const INPUT: &str = "@";
            DialasmParser::parse(Rule::handle, INPUT).expect_err(&format!("Handle can't be empty"));
        }

        #[test]
        fn simple_handle_valid() {
            const INPUT: &str = "@_a";
            let pairs = DialasmParser::parse(Rule::handle, INPUT)
                .expect(&format!("Expected to parse valid handle {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let handle = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                handle,
                &INPUT[1..],
                "Handles are not equal: expected {}, got {}",
                &INPUT[1..],
                handle
            );
        }
    }

    mod handle_group {
        use super::*;

        #[test]
        fn single_handle_in_group_valid() {
            const INPUT: &str = "(@_a)";
            let pairs = DialasmParser::parse(Rule::handle_group, INPUT)
                .expect(&format!("Expected to parse valid handle group {}", INPUT));
            let group: Vec<&str> = pairs
                .peek()
                .unwrap()
                .into_inner()
                .map(|p| p.into_inner().peek().unwrap().as_str())
                .collect();
            assert_eq!(group, &["_a"])
        }

        #[test]
        fn multiple_handles_in_group_valid() {
            const INPUT: &str = "(@_a & @i & @bbb)";
            let pairs = DialasmParser::parse(Rule::handle_group, INPUT)
                .expect(&format!("Expected to parse valid handle group {}", INPUT));
            let group: Vec<&str> = pairs
                .peek()
                .unwrap()
                .into_inner()
                .map(|p| p.into_inner().peek().unwrap().as_str())
                .collect();
            assert_eq!(group, &["_a", "i", "bbb"])
        }

        #[test]
        fn trailing_ampersand_in_group_invalid() {
            const INPUT: &str = "(@_a & @i &)";
            DialasmParser::parse(Rule::handle_group, INPUT)
                .expect_err(&format!("Trailing '&' is not allowed in handle groups"));
        }
    }

    mod choice {
        use super::*;

        #[test]
        fn simple_choice_valid() {
            const INPUT: &str = "\"Hello!\": start";
            let pairs = DialasmParser::parse(Rule::choice, INPUT)
                .expect(&format!("Expected to parse valid choice {}", INPUT));
            let mut inner_pairs = pairs.peek().unwrap().into_inner();
            assert_eq!(
                inner_pairs.len(),
                2,
                "Choice might include only two inputs: choice text and jump label."
            );
            let choice = (
                inner_pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .expect("Should be string literal")
                    .as_str(),
                inner_pairs.next().expect("Should be identifier").as_str(),
            );
            assert_eq!(choice, ("Hello!", "start"));
        }
    }

    mod choice_group {
        use super::*;

        #[test]
        fn single_choice_in_group_valid() {
            const INPUT: &str = "(\"Hello!\": start)";
            let pairs = DialasmParser::parse(Rule::choice_group, INPUT)
                .expect(&format!("Expected to parse valid choice group {}", INPUT));
            let group: Vec<(&str, &str)> = pairs
                .peek()
                .unwrap()
                .into_inner()
                .map(|p| {
                    let mut inner_pairs = p.into_inner();
                    assert_eq!(
                        inner_pairs.len(),
                        2,
                        "Choice might include only two inputs: choice text and jump label."
                    );
                    (
                        inner_pairs
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .expect("Should be string literal")
                            .as_str(),
                        inner_pairs.next().expect("Should be identifier").as_str(),
                    )
                })
                .collect();
            assert_eq!(group, &[("Hello!", "start")]);
        }

        #[test]
        fn multiple_choices_in_group_valid() {
            const INPUT: &str = "(\"Hello!\": start | \"Goodbye\": end)";
            let pairs = DialasmParser::parse(Rule::choice_group, INPUT)
                .expect(&format!("Expected to parse valid choice group {}", INPUT));
            let group: Vec<(&str, &str)> = pairs
                .peek()
                .unwrap()
                .into_inner()
                .map(|p| {
                    let mut inner_pairs = p.into_inner();
                    assert_eq!(
                        inner_pairs.len(),
                        2,
                        "Choice might include only two inputs: choice text and jump label."
                    );
                    (
                        inner_pairs
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .expect("Should be string literal")
                            .as_str(),
                        inner_pairs.next().expect("Should be identifier").as_str(),
                    )
                })
                .collect();
            assert_eq!(group, &[("Hello!", "start"), ("Goodbye", "end")]);
        }

        #[test]
        fn trailing_pipe_in_group_invalid() {
            const INPUT: &str = "(\"Hello!\": start | \"Goodbye\": end |)";
            DialasmParser::parse(Rule::choice_group, INPUT)
                .expect_err(&format!("Trailing '|' is not allowed in choice groups"));
        }
    }

    mod label {
        use super::*;

        #[test]
        fn empty_label_invalid() {
            const INPUT: &str = ":";
            DialasmParser::parse(Rule::label, INPUT).expect_err(&format!("Label can't be empty"));
        }

        #[test]
        fn simple_label_valid() {
            const INPUT: &str = "start:";
            let pairs = DialasmParser::parse(Rule::label, INPUT)
                .expect(&format!("Expected to parse valid label {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let label = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                label,
                &INPUT[..INPUT.len() - 1],
                "Labels are not equal: expected {}, got {}",
                &INPUT[..INPUT.len() - 1],
                label
            );
        }
    }

    mod name_statement {
        use super::*;

        #[test]
        fn simple_name_statement_valid() {
            const INPUT: &str = "@m = \"Hello kitty\"";
            let pairs = DialasmParser::parse(Rule::name_statement, INPUT)
                .expect(&format!("Expected to parse valid label {}", INPUT));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rules = (
                inner.next().expect("Expected valid handle").as_rule(),
                inner
                    .next()
                    .expect("Expected valid string literal")
                    .as_rule(),
            );
            assert_eq!(rules, (Rule::handle, Rule::string_literal));
        }
    }

    mod phrase_statement {
        use super::*;

        #[test]
        fn empty_speaker_phrase_statement_valid() {
            const INPUT: &str = ": \"Hello kitty!\"";
            let pairs = DialasmParser::parse(Rule::phrase_statement, INPUT).expect(&format!(
                "Expected to parse valid phrase statement {}",
                INPUT
            ));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rule = inner
                .next()
                .expect("Expected valid string literal")
                .as_rule();
            assert_eq!(rule, Rule::string_literal);
        }

        #[test]
        fn empty_handle_speaker_phrase_statement_invalid() {
            const INPUT: &str = "@: \"Hello kitty!\"";
            DialasmParser::parse(Rule::phrase_statement, INPUT).expect_err(&format!(
                "Empty handle can't be a valid speaker for the phrase"
            ));
        }

        #[test]
        fn single_speaker_phrase_statement_valid() {
            const INPUT: &str = "@m: \"Hello kitty!\"";
            let pairs = DialasmParser::parse(Rule::phrase_statement, INPUT).expect(&format!(
                "Expected to parse valid phrase statement {}",
                INPUT
            ));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rules = (
                inner.next().expect("Expected valid handle").as_rule(),
                inner
                    .next()
                    .expect("Expected valid string literal")
                    .as_rule(),
            );
            assert_eq!(rules, (Rule::handle, Rule::string_literal));
        }

        #[test]
        fn multiple_speakers_phrase_statement_valid() {
            const INPUT: &str = "(@m & @d): \"Hello kitty!\"";
            let pairs = DialasmParser::parse(Rule::phrase_statement, INPUT).expect(&format!(
                "Expected to parse valid phrase statement {}",
                INPUT
            ));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rules = (
                inner.next().expect("Expected valid handle group").as_rule(),
                inner
                    .next()
                    .expect("Expected valid string literal")
                    .as_rule(),
            );
            assert_eq!(rules, (Rule::handle_group, Rule::string_literal));
        }
    }

    mod choice_statement {
        use super::*;

        #[test]
        fn single_option_choice_statement_valid() {
            const INPUT: &str = "? \"Hello kitty!\" : start";
            let pairs = DialasmParser::parse(Rule::choice_statement, INPUT).expect(&format!(
                "Expected to parse valid choice statement {}",
                INPUT
            ));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rule = inner.next().expect("Expected valid choice").as_rule();
            assert_eq!(rule, Rule::choice);
        }

        #[test]
        fn multiple_options_choice_statement_valid() {
            const INPUT: &str = "? (\"Hello kitty!\" : start | \"Hello britty!\" : start)";
            let pairs = DialasmParser::parse(Rule::choice_statement, INPUT).expect(&format!(
                "Expected to parse valid choice statement {}",
                INPUT
            ));
            let mut inner = pairs.peek().unwrap().into_inner();
            let rule = inner.next().expect("Expected valid choice group").as_rule();
            assert_eq!(rule, Rule::choice_group);
        }
    }

    mod jump_statement {
        use super::*;

        #[test]
        fn simple_jump_statement_valid() {
            const INPUT: &str = "jump start";
            let pairs = DialasmParser::parse(Rule::jump_statement, INPUT)
                .expect(&format!("Expected to parse valid jump statement {}", INPUT));
            let output = pairs.peek().unwrap().as_str();
            assert_eq!(
                output, INPUT,
                "Outputs are not equal: expected {}, got {}",
                INPUT, output
            );
            let label = pairs.peek().unwrap().into_inner().peek().unwrap().as_str();
            assert_eq!(
                label,
                &INPUT[5..],
                "Handles are not equal: expected {}, got {}",
                &INPUT[5..],
                label
            );
        }
    }

    mod dialogue_statement {
        use super::*;

        #[test]
        fn name_is_valid_dialogue_statement() {
            const INPUT: &str = "@m = \"Max\";";
            DialasmParser::parse(Rule::dialogue_statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }

        #[test]
        fn phrase_is_valid_dialogue_statement() {
            const INPUT: &str = "(@m & @d): \"Max!\";";
            DialasmParser::parse(Rule::dialogue_statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }

        #[test]
        fn choice_is_valid_dialogue_statement() {
            const INPUT: &str = "? \"End\" : end;";
            DialasmParser::parse(Rule::dialogue_statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }

        #[test]
        fn jump_is_valid_dialogue_statement() {
            const INPUT: &str = "jump middle;";
            DialasmParser::parse(Rule::dialogue_statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }
    }

    mod statement {
        use super::*;

        #[test]
        fn dialogue_statement_is_a_valid_statement() {
            const INPUT: &str = "jump middle;";
            DialasmParser::parse(Rule::statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }

        #[test]
        fn dlabel_is_a_valid_statement() {
            const INPUT: &str = "middle:";
            DialasmParser::parse(Rule::statement, INPUT).expect(&format!(
                "Expected to parse valid dialogue statement {}",
                INPUT
            ));
        }
    }

    mod comment {
        use super::*;

        #[test]
        fn simple_multi_line_comment_valid() {
            const INPUT: &str = "/* Hello?\n I love you. */";
            DialasmParser::parse(Rule::comment, INPUT)
                .expect(&format!("Expected valid simple comment {}", INPUT));
        }
    }

    mod program {
        use super::*;

        #[test]
        fn empty_program_valid() {
            const INPUT: &str = "";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }

        #[test]
        fn single_comment_program_valid() {
            const INPUT: &str = "/* Hello */";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }

        #[test]
        fn two_comments_in_row_program_valid() {
            const INPUT: &str = "/* Hello */ /* Lolololo */";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }

        #[test]
        fn single_statement_program_valid() {
            const INPUT: &str = "hi:";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }

        #[test]
        fn single_statement_and_comment_program_valid() {
            const INPUT: &str = "hi: /* World */";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }

        #[test]
        fn multiple_statements_and_comment_program_valid() {
            const INPUT: &str = "hi:\n@m = \"Maria\"; /* World */";
            DialasmParser::parse(Rule::program, INPUT)
                .expect(&format!("Expected valid program {}", INPUT));
        }
    }
}
