extern crate dialasm;

#[cfg(test)]
mod rules {
    use std::collections::HashMap;

    use super::*;
    use dialasm::{Dialogue, DialogueChoice, DialogueEntry};

    #[test]
    fn empty_dialogue() {
        const INPUT: &str = "";
        let dlg = Dialogue::parse(INPUT);
        assert_eq!(dlg.len(), 0);
        assert_eq!(dlg.label_count(), 0);
    }

    #[test]
    fn single_line_dialogue() {
        const INPUT: &str = ": \"Hello.\";";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let labels_to_check = HashMap::new();
        assert_eq!(
            entries,
            &[DialogueEntry::Phrase(vec![], String::from("Hello!"))]
        );
        assert_eq!(labels, &labels_to_check);
    }

    #[test]
    fn single_label_dialogue() {
        const INPUT: &str = "damn:";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let mut labels_to_check = HashMap::new();
        labels_to_check.insert(String::from("damn"), 0);
        assert_eq!(entries, &[]);
        assert_eq!(labels, &labels_to_check);
    }

    #[test]
    fn multiple_label_dialogue() {
        const INPUT: &str = "damn:\ntest:";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let mut labels_to_check = HashMap::new();
        labels_to_check.insert(String::from("damn"), 0);
        labels_to_check.insert(String::from("test"), 0);
        assert_eq!(entries, &[]);
        assert_eq!(labels, &labels_to_check);
    }

    #[test]
    fn no_labels_dialogue() {
        const INPUT: &str = "@m = \"Maria\";
        @m: \"Hello!\"
        : \"Hi!\"";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let labels_to_check = HashMap::new();
        assert_eq!(
            entries,
            &[
                DialogueEntry::NameChange(String::from("m"), String::from("Maria")),
                DialogueEntry::Phrase(Vec::from(&[String::from("m")]), String::from("Hello!")),
                DialogueEntry::Phrase(vec![], String::from("Hi!"))
            ]
        );
        assert_eq!(labels, &labels_to_check);
    }

    #[test]
    fn jump_dialogue() {
        const INPUT: &str = "jump fall;";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let labels_to_check = HashMap::new();
        assert_eq!(entries, &[DialogueEntry::Jump(String::from("fall"))]);
        assert_eq!(labels, &labels_to_check);
    }

    #[test]
    fn choice_dialogue() {
        const INPUT: &str = "? (\"A!\": a | \"B!\": b | \"C!\": c);";
        let dlg = Dialogue::parse(INPUT);
        let entries = dlg.entries();
        let labels = dlg.labels();
        let labels_to_check = HashMap::new();
        assert_eq!(
            entries,
            &[DialogueEntry::Choice(vec![
                DialogueChoice {
                    text: String::from("A!"),
                    label: String::from("a")
                },
                DialogueChoice {
                    text: String::from("B!"),
                    label: String::from("b")
                },
                DialogueChoice {
                    text: String::from("C!"),
                    label: String::from("c")
                }
            ])]
        );
        assert_eq!(labels, &labels_to_check);
    }
}
