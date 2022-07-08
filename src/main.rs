mod win_support;

use indexmap::IndexMap;
use ansi_term::enable_ansi_support;
use terminal_size::{Width, Height, terminal_size};
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio};

fn main() {
    let _ = enable_ansi_support();
    let fg_yellow = "\x1b[93m";
    let fg_red = "\x1b[31m";

    if let Some((Width(w), Height(h))) = terminal_size() {
        print_with_color(fg_yellow, &format!("term size: {} cols wide, {} lines high", w, h), true);
    } else {
        print_with_color(fg_yellow, "failed to get terminal size", true);
    }

    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("managed shell".to_string(), "PowerShell".to_string());
    hashmap.insert("native shell".to_string(), "msh".to_string());
    print_with_color(fg_yellow, &format!("Use HashMap: {:?}", hashmap), true);

    let mut indexmap: IndexMap<String, String> = IndexMap::new();
    indexmap.insert("managed shell".to_string(), "PowerShell".to_string());
    indexmap.insert("native shell".to_string(), "msh".to_string());
    print_with_color(fg_yellow, &format!("Use IndexMap: {:?}", hashmap), true);

    let args: Vec<String> = env::args().collect();
    print_with_color(fg_yellow, &format!("{:?}", args), true);
    if args.len() > 1 {
        let mut process = Command::new(&args[1]);
        process.stdout(Stdio::piped());

        match process.spawn() {
            Ok(mut child) => {
                let mut buf_read = BufReader::new(child.stdout.take().unwrap());
                let mut line = String::new();

                loop {
                    let len = buf_read.read_line(&mut line).unwrap();
                    if len == 0 { break; } // EOF
                    print_with_color(fg_yellow, &line, false);
                    line.clear();
                }

                let _ = child.wait();
            }

            Err(err) => {
                print_with_color(fg_red, &format!("Failed to start process: {}", err), true);
            }
        }
    }
}

fn print_with_color(fg_color: &str, text: &str, new_line: bool) {
    if new_line {
        println!("{}{}\x1b[0m", fg_color, text);
    } else {
        print!("{}{}\x1b[0m", fg_color, text);
    }
}
