use std::{collections::HashMap, ops::Index};

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dialasm.pest"]
pub struct DialasmParser;

#[derive(PartialEq, Eq, Debug)]
pub struct DialogueChoice {
    pub text: String,
    pub label: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum DialogueEntry {
    NameChange(String, String),
    Phrase(Vec<String>, String),
    Choice(Vec<DialogueChoice>),
    Jump(String),
}

#[derive(Debug)]
pub struct Dialogue {
    entries: Vec<DialogueEntry>,
    labels: HashMap<String, usize>,
}

impl Dialogue {
    pub fn example() -> Dialogue {
        let entries = vec![
            DialogueEntry::NameChange(String::from("m"), String::from("Maria")),
            DialogueEntry::NameChange(String::from("l"), String::from("Leon")),
            DialogueEntry::Phrase(
                vec![String::from("m")],
                String::from("Hello, my name is Maria!"),
            ),
            DialogueEntry::Phrase(
                vec![String::from("l")],
                String::from("Hello, my name is Leon."),
            ),
            DialogueEntry::Phrase(
                vec![String::from("m"), String::from("l")],
                String::from("And we can talk together as well!"),
            ),
            DialogueEntry::Phrase(
                vec![String::from("m")],
                String::from("Now, you pick where to go!"),
            ),
            DialogueEntry::Choice(vec![
                DialogueChoice {
                    text: String::from("I pick A"),
                    label: String::from("a"),
                },
                DialogueChoice {
                    text: String::from("I pick B"),
                    label: String::from("b"),
                },
            ]),
            DialogueEntry::Phrase(vec![String::from("m")], String::from("Excellent choice!")),
            DialogueEntry::Jump(String::from("last")),
            DialogueEntry::Phrase(
                vec![String::from("l")],
                String::from("Certainly better choice."),
            ),
            DialogueEntry::Phrase(
                vec![String::from("l"), String::from("m")],
                String::from("Now, last choice... well, you only have one."),
            ),
            DialogueEntry::Choice(vec![DialogueChoice {
                text: String::from("Byeee!"),
                label: String::from("end"),
            }]),
            DialogueEntry::Phrase(vec![String::from("m")], String::from("Goodbye!")),
        ];
        let labels = HashMap::from([
            (String::from("a"), 7),
            (String::from("b"), 9),
            (String::from("last"), 10),
            (String::from("end"), 12),
        ]);
        Dialogue { entries, labels }
    }

    pub fn parse(_src: &str) -> Dialogue {
        todo!();
    }

    pub fn get(&self, index: usize) -> Option<&DialogueEntry> {
        self.entries.get(index)
    }

    pub fn label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }

    pub fn entries(&self) -> &[DialogueEntry] {
        &self.entries
    }

    pub fn labels(&self) -> &HashMap<String, usize> {
        &self.labels
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn label_count(&self) -> usize {
        self.labels.len()
    }
}

impl Index<usize> for Dialogue {
    type Output = DialogueEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl PartialEq<Dialogue> for Dialogue {
    fn eq(&self, other: &Dialogue) -> bool {
        self.entries == other.entries && self.labels == other.labels
    }
}
