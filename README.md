# DialASM - a primitive dialogue writing language
And it was made for Rust.
## Script example
```
/* Multiline comments are allowed,
as you can see here. */
@m = "Maria"; /* This is a name assignment. */
@l = "Leon"; /* Another name assignment */

: "This is a phrase told by... well, nobody.";
@m: "Hello, my name is Maria!";
@l: "Hello, my name is Leon.";
(@m & @l): "And we can talk together as well!";
@m: "Now, you pick where to go!";

? ("I pick A": a | "I pick B": b); /* Choices are important to make! */

a: /* Labels can be used for choices... */
@m: "Excellent choice!";
jump last; /* ...or for jumps. */

b:
@l: "Certainly better choice.";

last:
(@l & @m): "Now, last choice... well, you only have one.";
? "Byeee!": end;

end:
@m: "Goodbye!";

```

## Example of using the API:
```rust
use std::{
    collections::HashMap,
    fs::{self},
    io::{self, Write},
};

use dialasm::Dialogue;

fn main() {
    let Ok(contents) = fs::read_to_string("test.dlg") else {
        println!("Failed to open file 'test.dlg'");
        return;
    };
    let Some(dlg) = Dialogue::parse(&contents) else {
        println!("Failed to parse dialogue 'test.dlg'");
        return;
    };

    let mut pointer: usize = 0;
    let mut speakers = HashMap::new();
    while pointer < dlg.len() {
        match &dlg[pointer] {
            dialasm::DialogueEntry::NameChange(h, n) => {
                speakers.insert(h, n);
                pointer += 1;
            }
            dialasm::DialogueEntry::Phrase(h, t) => {
                let speaker_names = h.iter().map(|x| speakers[x].clone()).collect::<Vec<String>>().join(" & ");
                print!("{}: {}", speaker_names, t);
                io::stdout().flush().expect("Failed to flush stdout");
                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read line from stdin");
                pointer += 1;
            }
            dialasm::DialogueEntry::Choice(choices) => {
                let mut proceed = false;
                while !proceed {
                    for i in 1..=choices.len() {
                        println!("{}: {}", i, choices[i - 1].text);
                    }
                    io::stdout().flush().expect("Failed to flush stdout");
                    let mut buffer = String::new();
                    io::stdin()
                        .read_line(&mut buffer)
                        .expect("Failed to read line from stdin");
                    if let Result::Ok(idx) = buffer.trim_end().parse::<usize>() {
                        if idx < 1 || idx > choices.len() {
                            println!("Invalid choice index");
                            continue;
                        }
                        proceed = true;
                        pointer = dlg.label(&choices[idx - 1].label).unwrap();
                    }
                }
            }
            dialasm::DialogueEntry::Jump(l) => {
                pointer = dlg.label(l).unwrap();
            }
        }
    }
}
```
`Dialogue::parse(src)` walks over every statement in the source text. There are two types of statements: dialogue statements which are responsible for the flow of the dialogue (all of them end with semicolon), and jump labels, either used by `jump` instructions, or by choices. What is parsed is then converted into `Dialogue` struct. It doesn't contain any logic of executing dialogues on it's own, but it contains data relevant for implementing the actual engine for executing it. It mainly contains array of entries which represent each statement in the source text, with the exception of jump labels which are stored separately and store indices for their respective entries. Jump labels can be accessed via instance function `Dialogue::label(name)`.
# The language:
## Flow statements
You have four main flow statements.
### Name assignment
Your characters need names. In order to assign them, you use handles. Like in social media like Twitter, or Discord, these are ids of character.

`@m = "Maria";`

This will assign a name "Maria" to speaker `@m`. Note that you cannot use character whose name was not set, as such character doesn't exist.
### Phrase
The actual content of the dialogue. You would expect a speaker being specified, but you actually can do anonymous phrases as well:

`: "This is an anonymous phrase.";`

To assign speaker to a phrase, you write it like this:

`@m: "Hello! My name is Maria!";`

We can make multiple speakers tell the same phrase too!

`(@m & @l): "We are number one!";`

### Choices
What kind of dialogue is without choices? ...kinetic one. But `DialASM` supports choices in this format:

`? ("This is a choice a": a | "This is a choice b": b);`

It starts with `?` character, and then is followed by the actual choices. Single choice is defined by it's text and jump label.
Single choice can also be made:

`? "Bye": bye;`

### Jumps
Very useful thing that allows up to jump to any spot of the dialogue.

`jump label;`

This should make dialogue move to the point marked by this specific label.

## Label
As you've already seen, labels are used for choices and `jump` statements. The syntax to mark label is this:

`a:`

This will define label `a` to point at the instruction right next to it. Multiple labels can be put in a row, but they will point at the same instruction.
Important: labels should be unique per file. Also, any label referred in choice or `jump` statement should exist in the actual source, otherwise parsing will fail.

## Comments
The least important thing, but very useful for documenting chapters. Only C-style comments are supported.

`/* This is a comment. */`

## Grammar
```
WHITESPACE = _{ " " | "\r" | "\n" | "\t" }

identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
string_content = { (!"\"" ~ (!"\\" ~ ANY) | "\\" ~ ANY)* }
string_literal = ${ "\"" ~ string_content ~ "\"" }

handle = ${ "@" ~ identifier }
handle_group = { "(" ~ handle ~ ("&" ~ handle)* ~ ")" }
choice = { string_literal ~ ":" ~ identifier }
choice_group = { "(" ~ choice ~ ("|" ~ choice)* ~ ")" }

label = ${ identifier ~ ":" }
name_statement = { handle ~ "=" ~ string_literal }
phrase_statement = { (handle | handle_group)? ~ ":" ~ string_literal }
choice_statement = { "?" ~ (choice | choice_group) }
jump_statement = { "jump" ~ identifier }

dialogue_statement = { (name_statement | phrase_statement | choice_statement | jump_statement) ~ ";" }

statement = { dialogue_statement | label }
comment = _{ ("/*" ~ (!"*/" ~ ANY)* ~ "*/") }

program = { SOI ~ (statement | comment)* ~ EOI }
```
