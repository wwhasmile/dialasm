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

fn main() -> Result<(), i32> {
    let Ok(contents) = fs::read_to_string("test.dlg") else {
        println!("Failed to open file 'test.dlg'");
        return Err(1);
    };
    let Some(dlg) = Dialogue::parse(&contents) else {
        println!("Failed to parse dialogue 'test.dlg'");
        return Err(1);
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
                let speaker_names: Vec<&String> = h.iter().map(|x| {
                    speakers[x]
                }).collect();
                println!("{:?}: {}", speaker_names, t);
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
                    if let Ok(idx) = buffer.trim_end().parse::<usize>() {
                        if idx < 1 || idx > choices.len() {
                            println!("Invalid choice index");
                            continue;
                        }
                        proceed = true;
                        let mut exit = false;
                        pointer = dlg.label(&choices[idx - 1].label).unwrap_or_else(|| {
                            println!("Label '{}' doesn't exist", choices[idx].label);
                            exit = true;
                            0
                        });
                        if exit {
                            return Err(2);
                        }
                    }
                }
            }
            dialasm::DialogueEntry::Jump(l) => {
                let mut exit = false;
                pointer = dlg.label(l).unwrap_or_else(|| {
                    println!("Label '{}' doesn't exist", l);
                    exit = true;
                    0
                });
                if exit {
                    return Err(2);
                }
            }
        }
    }

    Ok(())
}
```
The result of `Dialogue::parse(src)` is stored in the `Dialogue` struct. It doesn't contain any logic of executing dialogues on it's own, but it contains data relevant for implementing the actual engine for executing it. It mainly contains array of entries which represent each statement in the source text, with the exception of jump labels which are stored separately. Labels can be accessed via instance function `Dialogue::label(name)`, which returns index of the entry that you should jump to.