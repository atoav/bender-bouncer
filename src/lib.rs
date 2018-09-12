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
//! bender_bouncer = { git = "https://github.com/atoav/bender-bouncer.git" }
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
//! 3. run `cargo build --release`
//! 4. copy the compiled binary `./target/release/bender-bouncer-cli` wherever you like
//!

extern crate flate2;
#[macro_use] extern crate structure;

use std::collections::HashMap;
use std::io::prelude::*;
use flate2::read::GzDecoder;

use std::str;
use std::fs::File;
use std::io::SeekFrom;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;


/// This function returns Ok(()) if the read file is a valid blend file and an
/// Error if something is wonky
pub fn check_blend<S>(path: S) -> GenResult<()> where S: Into<String>{
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

/// A Data Struct that holds a blend files data
#[derive(Debug)]
pub struct Data{
    pub frames: FrameRange,
    pub version: String
}

/// A FrameRange Struct that holds a start and a endframe
#[derive(Debug)]
pub struct FrameRange{
    pub start: i32,
    pub end: i32
}

impl FrameRange{
    /// Return the number of frames
    pub fn count(&self) -> i32{
        self.end - self.start
    }
}

/// A HashMap-based Type to hold a Scene with Scene Name and Data
pub type Scenes = HashMap<String, Data>;



/// Find the scenes within the blend file specified. This assumes the file is a
/// valid blend file and will return a Error if this is not the case. If you are
/// not sure, check for validity with the `bender_bouncer::check_blend()` function
pub fn parse_scenes<S>(path: S) -> GenResult<Scenes> where S: Into<String>{
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
    let blender_head_size = if buf[7] == 0x2d { 24 } else { 20 };
    let is_big_endian = buf[8] == 0x56;

    // Go to 9th byte and read a buffer of 3 bytes for version
    f.seek(SeekFrom::Start(9))?;
    let mut buf = vec![0u8; 3];
    f.read_exact(&mut buf)?;
    let mut version = str::from_utf8(&buf)?.to_string();
    version.insert_str(1, ".");


    // Go to 12th byte and read a buffer of 4 bytes
    f.seek(SeekFrom::Start(12))?;
    let mut utf8buf = vec![0u8; 4];
    f.read_exact(&mut utf8buf)?;
    let mut scenes = Scenes::new();

    // Loop through the scenes
    while str::from_utf8(&utf8buf)? == "REND"{

        // 4 Bytes "size" integer describing the the
        // Total length of data after the file-block-header
        let mut buf = vec![0u8; 4];
        f.read_exact(&mut buf)?;
        let _file_block_header_size = match is_big_endian { 
            true => {
                let s = structure!(">i");
                s.unpack(buf)?
            },
            false => {
                let s = structure!("<i");
                s.unpack(buf)?
            }
        };

        // 4 or 8 Bytes pointer describing the old memory adress
        // "where the structure was located when written to disk"
        let mut buf = vec![0u8; match blender_head_size {
            20 => 4,
            _ => 8
        }];
        f.read_exact(&mut buf)?; // just skip it

        // 4 byte integer describing the SDNA index
        let mut buf = vec![0u8; 4];
        f.read_exact(&mut buf)?;
        let _sdna_index = match is_big_endian { 
            true => {
                let s = structure!(">i");
                s.unpack(buf)?.0
            },
            false => {
                let s = structure!("<i");
                s.unpack(buf)?.0
            }
        };

        // 4 byte integer describing the Number of structures located 
        // in this file-block
        let mut buf = vec![0u8; 4];
        f.read_exact(&mut buf)?;
        let _count = match is_big_endian { 
            true => {
                let s = structure!(">i");
                s.unpack(buf)?.0
            },
            false => {
                let s = structure!("<i");
                s.unpack(buf)?.0
            }
        };

        // Now find the rest
        buf = vec![0u8; 8];
        f.read_exact(&mut buf)?;
        // Find Start and endframes
        let (start, end) = match is_big_endian { 
            true => {
                let s = structure!(">2i");
                s.unpack(buf)?
            },
            false => {
                let s = structure!("<2i");
                s.unpack(buf)?
            }
        };

        // Find Scenename
        buf = vec![0u8; 64];
        f.read_exact(&mut buf)?;
        // Try to create valid utf8 from it and trim all null chars
        let mut scenename = str::from_utf8(&buf)?;
        scenename = scenename.trim_matches(char::from(0));
        // Create a Data struct and insert into scenes
        scenes.insert(
            scenename.to_string(), 
            Data{
                frames: FrameRange{start, end},
                version: version.clone()
            });

        // Read for the next round
        utf8buf = vec![0u8; 4];
        f.read_exact(&mut utf8buf)?;
    }
    Ok(scenes)
}