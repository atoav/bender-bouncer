#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate colored;

use docopt::Docopt;
use colored::*;

const USAGE: &'static str = "
Bouncer.

Usage:
  bouncer <blendfiles>... [--json] [--dry-run]
  bouncer (-h | --help)
  bouncer --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --json        Output the status in json
  --dry-run     Just pretend you are doing something
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_json: bool,
    flag_dry_run: bool,
    arg_blendfiles: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    if !args.flag_json{
        for name in args.arg_blendfiles.iter(){
            println!("{}", name.bold().on_bright_blue());
        }
    }
}