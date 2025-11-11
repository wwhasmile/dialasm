use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use pest::{Parser, error::Error, iterators::Pair};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "dialasm.pest"]
pub struct DialasmParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid program: {0}")]
    InvalidProgram(Error<Rule>),
    #[error("Undefined speaker '{0}'")]
    UndefinedSpeaker(String),
    #[error("Undefined label '{0}'")]
    UndefinedLabel(String),
    #[error("Duplicate label '{0}'")]
    DuplicateLabel(String),
}

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
    /// Example dialogue. Repeats the one from README.md
    pub fn example() -> Dialogue {
        let entries = vec![
            DialogueEntry::NameChange(String::from("m"), String::from("Maria")),
            DialogueEntry::NameChange(String::from("l"), String::from("Leon")),
            DialogueEntry::Phrase(
                vec![],
                String::from("This is a phrase told by... well, nobody."),
            ),
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
            (String::from("a"), 8),
            (String::from("b"), 10),
            (String::from("last"), 11),
            (String::from("end"), 13),
        ]);
        Dialogue { entries, labels }
    }

    /// Parses source. If there is an error then it fails.
    pub fn parse(src: &str) -> Result<Dialogue, ParseError> {
        let program = DialasmParser::parse(Rule::program, src).map_err(|e| ParseError::InvalidProgram(e))?;
        let mut program_inner = program.peek().unwrap().into_inner();
        let mut labels = HashMap::new();
        let mut entries = Vec::new();
        let mut unknown_labels: HashSet<String> = HashSet::new();
        let mut known_speakers: HashSet<String> = HashSet::new();
        program_inner.try_fold(0, |idx, p| {
            if p.as_rule() == Rule::EOI {
                return Ok(idx);
            }
            let statement = p.into_inner().peek().unwrap();
            if statement.as_rule() == Rule::label {
                let n = statement.into_inner().peek().unwrap().as_str();
                if labels.insert(n.to_string(), idx).is_some() {
                    return Err(ParseError::DuplicateLabel(n.to_string()));
                };
                unknown_labels.remove(n);
                return Ok(idx);
            };
            let statement = statement.into_inner().peek().unwrap();
            match statement.as_rule() {
                Rule::name_statement => {
                    let result = Self::parse_name_statement(statement);
                    if let DialogueEntry::NameChange(n, _) = &result {
                        known_speakers.insert(n.clone());
                    };
                    entries.push(result);
                }
                Rule::phrase_statement => {
                    let result = Self::parse_phrase_statement(statement);
                    if let DialogueEntry::Phrase(n, _) = &result {
                        n.iter().try_for_each(|n| {
                            if !known_speakers.contains(n) {
                                return Err(ParseError::UndefinedSpeaker(n.clone()));
                            }
                            Ok(())
                        })?;
                    };
                    entries.push(result);
                }
                Rule::choice_statement => {
                    let result = Self::parse_choice_statement(statement);
                    if let DialogueEntry::Choice(n) = &result {
                        n.iter().for_each(|n| {
                            if !labels.contains_key(&n.label) {
                                unknown_labels.insert(n.label.clone());
                            }
                        });
                    }
                    entries.push(result);
                }
                Rule::jump_statement => {
                    let result = Self::parse_jump_statement(statement);
                    if let DialogueEntry::Jump(n) = &result
                        && !labels.contains_key(n)
                    {
                        unknown_labels.insert(n.clone());
                    }
                    entries.push(result);
                }
                _ => (),
            };
            Ok(idx + 1)
        })?;
        if !unknown_labels.is_empty() {
            return Err(ParseError::UndefinedLabel(
                unknown_labels.iter().next().unwrap().clone(),
            ));
        }
        Ok(Dialogue { entries, labels })
    }

    fn parse_name_statement(pair: Pair<'_, Rule>) -> DialogueEntry {
        let mut inner = pair.into_inner();
        DialogueEntry::NameChange(
            inner
                .next()
                .unwrap()
                .into_inner()
                .peek()
                .unwrap()
                .as_str()
                .to_string(),
            inner
                .next()
                .unwrap()
                .into_inner()
                .peek()
                .unwrap()
                .as_str()
                .to_string(),
        )
    }

    fn parse_phrase_statement(pair: Pair<'_, Rule>) -> DialogueEntry {
        let mut inner = pair.into_inner();
        let mut first = inner.next().unwrap();
        let speakers = if first.as_rule() == Rule::handle_group {
            let result = first
                .into_inner()
                .map(|n| n.into_inner().peek().unwrap().as_str().to_string())
                .collect();
            first = inner.next().unwrap();
            result
        } else if first.as_rule() == Rule::handle {
            let result = vec![first.into_inner().peek().unwrap().as_str().to_string()];
            first = inner.next().unwrap();
            result
        } else {
            Vec::new()
        };
        DialogueEntry::Phrase(
            speakers,
            first.into_inner().peek().unwrap().as_str().to_string(),
        )
    }

    fn parse_choice_statement(pair: Pair<'_, Rule>) -> DialogueEntry {
        let inner = pair.into_inner().peek().unwrap();
        let result = if inner.as_rule() == Rule::choice_group {
            inner
                .into_inner()
                .map(|p| {
                    let mut choice_pairs = p.into_inner();
                    DialogueChoice {
                        text: choice_pairs
                            .next()
                            .unwrap()
                            .into_inner()
                            .peek()
                            .unwrap()
                            .as_str()
                            .to_string(),
                        label: choice_pairs.next().unwrap().as_str().to_string(),
                    }
                })
                .collect()
        } else {
            let mut choice_pairs = inner.into_inner();
            vec![DialogueChoice {
                text: choice_pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .peek()
                    .unwrap()
                    .as_str()
                    .to_string(),
                label: choice_pairs.next().unwrap().as_str().to_string(),
            }]
        };
        DialogueEntry::Choice(result)
    }

    fn parse_jump_statement(pair: Pair<'_, Rule>) -> DialogueEntry {
        DialogueEntry::Jump(pair.into_inner().peek().unwrap().as_str().to_string())
    }

    /// Safely get dialogue entry (instruction).
    pub fn get(&self, index: usize) -> Option<&DialogueEntry> {
        self.entries.get(index)
    }

    /// Safely get instruction pointer from label name.
    pub fn label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }

    /// Returns immutable list of entries.
    pub fn entries(&self) -> &[DialogueEntry] {
        &self.entries
    }

    /// Returns immutable map of labels to their pointers.
    pub fn labels(&self) -> &HashMap<String, usize> {
        &self.labels
    }

    // Returns count of entries (instruction).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    // If there are any entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    // Count of labels.
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
