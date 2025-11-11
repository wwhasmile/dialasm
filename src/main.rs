use anyhow::Result;
use anyhow::*;
use std::{
    collections::HashMap,
    env,
    fs::{self},
    io::{self, Write},
};

use dialasm::Dialogue;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "dialasm {}\nBy {}\n\nUsage:\ndialasm (<path to your script> or \"example\" for example dialogue)",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
        return Ok(());
    }

    let path = args.get(1).unwrap();
    let dlg: Dialogue = if path == "example" {
        Dialogue::example()
    } else {
        let contents = fs::read_to_string(path)?;
        match Dialogue::parse(&contents) {
            Result::Ok(parsed) => parsed,
            Err(e) => return Err(anyhow!(e)),
        }
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
    Ok(())
}
