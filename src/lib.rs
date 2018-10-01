//! The bouncer has two responsibilities:  
//! - checking if a file is a valid .blend file
//! - collecting basic data about the framerange
//! It is a Rust implementation of *blender_render_info.py* that comes bundled with python
//! 
//! bender-bouncer only reads the binary data of the .blend file and therefore
//! doesn't need a blender executable. This is also much faster than opening the
//! .blend with Blender
//!
//! ## bender bouncer consists of two parts  
//! - a **rust library** exposing the checking functionality so it can be used elsewhere
//! - a small CLI tool called **bender-bouncer-cli**, that allows for manual or scripted checking
//!
//! ## Library Usage
//! It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:  
//! ```ignore
//! [dependencies]
//! bender_bouncer = { git = "ssh://git@code.hfbk.net:4242/bendercode/bender-bouncer.git" }
//! ```
//! To update run
//! ```ignore
//! cargo clean
//! cargo update
//! ```
//! 
//! ## Documentation
//! To view the documentation on the different Functions execute  
//! ```ignore
//! cargo doc --no-deps --open
//! ```
//!
//! ## Installation (CLI tool)
//! 1. Make sure you have rust and cargo installed (easiest with [rustup](http://rustup.rs))
//! 2. Clone the repo via `git clone` and go into the repo with `cd bender_bouncer`
//! 3. run `sh install.sh`
//!


extern crate flate2;

use std::io::prelude::*;
use flate2::read::GzDecoder;
use std::str;
use std::fs::File;
use std::io::SeekFrom;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;




/// Takes a path and returns Ok<Version> if the blendfile is valid. If either
/// of the functions validate_basic or get_version fails, return a Error
pub fn check_blend<S>(path: S) -> GenResult<Version> where S: Into<String> {
    let path = path.into();
    validate_basic(&path[..])?;
    let version = get_version(&path[..])?;
    Ok(version)
}


/// This function returns Ok(()) if the read file is a valid blend file and an
/// Error if something is wonky
pub fn validate_basic<S>(path: S) -> GenResult<()> where S: Into<String>{
    let path = path.into();
    let mut f = File::open(&path[..])?;

    // read up to 7 bytes from the file header
    let mut head = vec![0u8; 7];
    f.read_exact(&mut head)?;

    // Check for gzip magic on first two bytes and unpack if needed
    if &head[0..2] == &[0x1f, 0x8b]{
        let mut gz = GzDecoder::new(&head[..]);
        // empty the buffer and overwrite it with the decompressed version
        let mut head = vec![0u8; 7];
        gz.read_exact(&mut head)?;
    }

    // Try to create a valid utf-8 string from head
    let header_string = str::from_utf8(&head)?;
    // If it asserts to assert_equal the file in question is a valid Blend file
    assert_eq!("BLENDER", header_string);
    Ok(())
}

type Version = String;


/// Find the scenes within the blend file specified. This assumes the file is a
/// valid blend file and will return a Error if this is not the case. If you are
/// not sure, check for validity with the `bender_bouncer::check_blend()` function
pub fn get_version<S>(path: S) -> GenResult<Version> where S: Into<String>{
    let path = path.into();
    let mut f = File::open(&path[..])?;

    // read up to 32 bytes from the file header
    let mut buf = vec![0u8; 32];
    f.read_exact(&mut buf)?;

    // Check for gzip magic on first two bytes and unpack if needed
    if &buf[0..2] == &[0x1f, 0x8b]{
        let mut gz = GzDecoder::new(&buf[..]);
        // empty the buffer and overwrite it with the decompressed version
        let mut buf = vec![0u8; 32];
        gz.read_exact(&mut buf)?;
    }

    // 24 for 64 Bit, 20 for 32 Bit
    let _blender_head_size = if buf[7] == 0x2d { 24 } else { 20 };
    let _is_big_endian = buf[8] == 0x56;

    // Go to 9th byte and read a buffer of 3 bytes for version
    f.seek(SeekFrom::Start(9))?;
    let mut buf = vec![0u8; 3];
    f.read_exact(&mut buf)?;
    let mut version = str::from_utf8(&buf)?.to_string();
    version.insert_str(1, ".");

    Ok(version.clone())

}
