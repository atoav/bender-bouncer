#[macro_use] extern crate serde_derive;
extern crate docopt;
extern crate colored;
extern crate bender_bouncer;

use docopt::Docopt;
use colored::*;
use bender_bouncer::{check_blend};


const USAGE: &str = "
bender_bouncer

The bouncer verifies wether one or more files are valid blend files. If so, bouncer
returns the version of Blender used to create the file.

Usage:
  bouncer [--json|--only-valid|--only-invalid] [--no-colors] [--basic] <blendfiles>...
  bouncer (-h | --help)
  bouncer --version

Options:
  -h --help         Show this screen.
  --version         Show version.
  --json            Json output
  --only-valid      Output only valid blendfiles
  --only-invalid    Output only invalid blendfiles
  --no-colors       Nomen est omen
  --basic           Output validity on one line
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_blendfiles: Vec<String>,
    flag_only_valid: bool,
    flag_only_invalid: bool,
    flag_no_colors: bool,
    flag_basic: bool,
    flag_json: bool,
    arg_blendfile: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());

    // Without --json flags print colourful results for multiple files
    if !args.flag_json{
        for path in args.arg_blendfiles.iter(){
            match check_blend(&path[..]){
                Ok(version) => {
                    if !args.flag_only_invalid{
                        // Success: print path
                        let mut fname = format!(" ✔ {} ", path).bold().on_green();
                        if args.flag_no_colors { fname = fname.clear(); }
                        let message = format!("{}", fname);
                        println!("{}", message);
                        if !args.flag_basic{
                            // Success: print version
                            let mut versionlabel = format!("{}      ", "Version:").green();
                            if args.flag_no_colors { versionlabel = versionlabel.clear(); };
                            println!("    {}{}\n", versionlabel, version,);
                        }
                    }
                },
                Err(_) => {
                    if !args.flag_json && !args.flag_only_valid {
                        // Error: print path
                        let mut fname = format!(" ✖ {} ", path).bold().on_red();
                        if args.flag_no_colors{ fname = fname.clear() ;};
                        println!("{}", fname);
                        if !args.flag_basic{
                            // Error: print message
                            let mut errorlabel = "ERROR:".to_string().red();
                            if args.flag_no_colors{ errorlabel = errorlabel.clear() ;};
                            println!("    {}        this seems not to be a valid blend file\n", errorlabel);
                        }
                    }
                }

            }
        }
    }

    // With --json flag just return basic json with validity and version
    if args.flag_json{
        match check_blend(&args.arg_blendfile[..]) {
            Ok(version) => {
                println!("{{version:{}}}", version);
            },
            Err(_) => {
                println!("{{version:invalid}}");
            }
        }

    }

}