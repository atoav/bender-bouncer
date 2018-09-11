#[macro_use] extern crate serde_derive;
extern crate docopt;
extern crate colored;
extern crate bender_bouncer;

use docopt::Docopt;
use colored::*;
use bender_bouncer::{check_blend, parse_scenes};


const USAGE: &'static str = "
bender-bouncer

The bouncer verifies wether one or more files are valid blend files and returns 
basic information like scene name, start and endframe.

Usage:
  bouncer <blendfiles>...
  bouncer (-h | --help)
  bouncer --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_blendfiles: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    
    for name in args.arg_blendfiles.iter(){
        // Check if we are dealing with a valid blend file
        let valid = match check_blend(&name[..]){
            Ok(()) => {
                let message = format!("{}", name.bold().on_green());
                println!("{}", message);
                true
            },
            Err(e) => {
                let message = format!("{} is not a blend file -> {}", name.bold().on_red(), e);
                println!("{}", message);
                false
            },
        };
        // If so, try to get Scene Name, Startframe and Endframe from it
        if valid {
            match parse_scenes(&name[..]){
                Ok(s) => {
                    for (name, frames) in s.iter(){
                        println!("Scene Name:   {}\nStartframe:   {}\nEndframe:     {}\n", name, frames.start, frames.end);
                    }
                },
                Err(e) => {
                    println!("Error: Couldn't parse file: {:?}", e);
                }
            };
            
        }

    }

}