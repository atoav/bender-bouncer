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
                let success = format!(" ✔ {} ", name).bold().on_green();
                let message = format!("{}", success);
                println!("{}", message);
                true
            },
            Err(_e) => {
                let error = format!(" ✖ {} ", name).bold().on_red();
                let message = format!("{} seems not to be a blend file", error);
                println!("{}", message);
                false
            },
        };
        // If so, try to get Scene Name, Startframe and Endframe from it
        if valid {
            match parse_scenes(&name[..]){
                Ok(s) => {
                    for (scenename, data) in s.iter(){
                        let versionlabel = format!("{}      ", "Version:".green());
                        let scenelabel = format!("{}   ", "Scene Name:".green());
                        let framelabel = format!("{}  ", "Frame Range:".green());
                        println!("{}{}\n{}{}\n{}{}-{} ({} in total)\n", 
                            versionlabel, data.version,
                            scenelabel, scenename, 
                            framelabel, data.frames.start, data.frames.end, data.frames.count());
                    }
                },
                Err(e) => {
                    println!("Error: Couldn't parse file: {:?}", e);
                }
            };
            
        }

    }

}