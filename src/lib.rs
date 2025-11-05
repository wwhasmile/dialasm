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
    pub fn parse(src: &str) -> Dialogue {
        todo!();
    }

    pub fn get(&self, index: usize) -> Option<&DialogueEntry> {
        self.entries.get(index)
    }

    pub fn label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).map(|s| *s)
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
