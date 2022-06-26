mod minesweeper;

use std::{
    io::{self, Write},
    string::String
};

use minesweeper::*;
use clap::Parser;

#[derive(Parser,Default)]
#[clap(author = "George Grainger <george@ggrainger.uk>", version, about="Simple cli minesweeper")]
struct Arguments {
    mines: usize,
    #[clap(default_value="10")]
    width: usize,
    #[clap(default_value="10")]
    height: usize
}

fn main() {
    let args = Arguments::parse();

    let mut ms = Minesweeper::new(args.width, args.height, args.mines);
    
    while ms.game_state == GameState::InProgress {
        print!("{ms}");
        print!("> ");
        io::stdout().flush();

        // Read input
        let mut buf: String = String::from("");
        io::stdin().read_line(&mut buf);

        let raw_input = buf.as_str().replace("\n", "");
        let input: Vec<&str> = raw_input.split(" ").collect();

        // This seems messy to me, is there a better way of doing this?
        match [input.get(0), input.get(1), input.get(2)] {
            // Statement to catch 3 argument input
            [Some(val0), Some(val1), Some(val2)] => {
                let cmd = *val0;
                let x = val1.parse::<usize>().unwrap();
                let y = val2.parse::<usize>().unwrap();

                let pos = (x, y);

                if cmd == "f" || cmd == "flag" {
                    ms.flag(pos);
                } else if cmd == "o" || cmd == "open" {
                    ms.open(pos);
                }
            }

            // Statement to catch single argument input
            [Some(val), _, _] => {
                let cmd = *val;

                if cmd == "help" {
                    // TODO
                } else {
                    println!("Unknown command '{cmd}'");
                    println!("Type 'help' for more information");
                }
            }

            // Wildcard
            _ => {}
        }
    }

    // Show final state
    print!("{ms}");
}
