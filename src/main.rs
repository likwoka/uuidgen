//! uuidgen -- A toy program that generates v4 UUIDs to stdout.
//!
//! Usage:
//! uuidgen           # generate 20 UUIDs (the default)
//! uuidgen -n 5      # generate 5 UUIDs
//! uuidgen -i        # interactive mode, prompt user for how many to generate.
//!         
use std;
use std::{env, io, error::Error};
use uuid::Uuid;

const HELP_MSG: &str = r"uuidgen -- A toy program that generates v4 UUIDs to stdout.

Usage:
uuidgen           # generate 20 UUIDs (the default)
uuidgen -h        # print this help message
uuidgen -n 5      # generate 5 UUIDs
uuidgen -i        # interactive mode, prompt user for how many to generate";

/// Hard code to generate 20 UUIDs by default.
const NUM_TO_GENERATE: u8 = 20;
 
/// Return a list of X UUIDs where X is the count.  
fn gen_uuids(count: u8) -> Vec<Uuid> {
    let mut vals: Vec<Uuid> = Vec::new();
    for _n in 0..count {
        vals.push(Uuid::new_v4());
    }
    vals
}

enum OutputMode {
    Help,
    Default,
    NumSpecified(u8),
    Interactive
}

enum MyError {
    WrongSyntax,
    NumberNotInteger
}

fn parse_output_mode(args: Vec<String>) -> Result<OutputMode, MyError> {
    if args.len() == 1 {
        Ok(OutputMode::Default)
    } else if args.len() == 2 && args[1] == "-h" {
        Ok(OutputMode::Help)
    } else if args.len() == 2 && args[1] == "-i" {
        Ok(OutputMode::Interactive)
    } else if args.len() == 3 && args[1] == "-n" {
        match args[2].parse::<u8>() {
            Ok(cnt) => Ok(OutputMode::NumSpecified(cnt)),
            Err(_error) => Err(MyError::NumberNotInteger)
        }
    } else {
        Err(MyError::WrongSyntax)
    }
}

fn interactive_ask() -> Result<u8, String>  {
    let mut cnt_str = String::new();
    println!("How many UUIDs go do you want to generate?");
    match io::stdin().read_line(&mut cnt_str) {
        Ok(_size) => {
            match cnt_str.trim().parse::<u8>() {
                Ok(cnt) => Ok(cnt),
                Err(_error) => {
                    Err(_error.to_string())
                }
            }
        },
        Err(_error) => Err(_error.to_string())
    }
}

fn print_uuids(uuids: Vec<Uuid>) {
    for x in &uuids {
        println!("{}", x);
    } 
}

fn main() {
    // TODO4: add unit tests
    let args: Vec<String> = env::args().collect();
    let mode = parse_output_mode(args);

    match mode {
        Ok(OutputMode::Help) => {
            println!("{}", HELP_MSG);
        },
        Err(MyError::NumberNotInteger) => {
            println!("Input must be an integer!");
        },
        Err(MyError::WrongSyntax) => {
            println!("Wrong syntax!");
        },
        Ok(OutputMode::Default) => {
            let uuids = gen_uuids(NUM_TO_GENERATE);
            print_uuids(uuids);
        },
        Ok(OutputMode::NumSpecified(cnt)) => {
            let uuids = gen_uuids(cnt);
            print_uuids(uuids);
        },
        Ok(OutputMode::Interactive) => {
            match interactive_ask() {
                Err(msg) => println!("{}", msg),
                Ok(cnt) => {
                    let uuids = gen_uuids(cnt);
                    print_uuids(uuids);
                }
            }
        }        
    };
}
