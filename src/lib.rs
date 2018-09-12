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

    // Check if header is over
    assert!(str::from_utf8(&utf8buf)? == "REND");

    // 4 Bytes "size" integer describing the the
    // Total length of data after the file-block-header
    let buf = nom(&mut f, 4)?;
    let _file_block_header_size = unpack_i32(buf, is_big_endian);

    // 4 or 8 Bytes pointer describing the old memory adress
    // "where the structure was located when written to disk"
    read_old_mem(&mut f, blender_head_size)?;

    // 4 byte integer describing the SDNA index
    let buf = nom(&mut f, 4)?;
    let _sdna_index = unpack_i32(buf, is_big_endian);

    // 4 byte integer describing the Number of structures located 
    // in this file-block
    let buf = nom(&mut f, 4)?;
    let _count = unpack_i32(buf, is_big_endian);

    // Now find the rest
    let buf = nom(&mut f, 4)?;
    let start = unpack_i32(buf, is_big_endian)?;
    let mut buf = nom(&mut f, 4)?;
    let end = unpack_i32(buf, is_big_endian)?;

    // Find Scenename
    buf = vec![0u8; 64];
    f.read_exact(&mut buf)?;
    // Try to create valid utf8 from it and trim all null chars
    let mut scenename = str::from_utf8(&buf)?;
    scenename = scenename.trim_matches(char::from(0));

    // Read for the next round
    utf8buf = vec![0u8; 4];
    f.read_exact(&mut utf8buf)?;

    loop_blocks(f, is_big_endian, blender_head_size)?;

    // Create a Data struct and insert into scenes
    scenes.insert(
        scenename.to_string(), 
        Data{
            frames: FrameRange{start, end},
            version: version.clone()
        });
    Ok(scenes)

}



fn nom(f: &mut File, bytes: usize) -> GenResult<Vec<u8>> {
    let mut buf = vec![0u8; bytes];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn nom_till(f: &mut File, bytes: usize, exitstring: &str) -> GenResult<Vec<u8>> {
    let mut vector: Vec<u8> = Vec::new();
    while let Ok(buf) = nom(f, bytes){
        let found = match str::from_utf8(&buf){
            Ok(s) => {
                // manage if in dnablock
                if s == exitstring { true } else { false }
            },
            Err(_e) => ( false )
        };
        if found { 
            break 
        } else { 
            for b in buf {
                vector.push(b); 
            }
        }
    }
    Ok(vector)
}

fn nom_after(f: &mut File, bytes: usize, exitstring: &str) -> GenResult<Vec<u8>> {
    while let Ok(buf) = nom(f, bytes){
        let found = match str::from_utf8(&buf){
            Ok(s) => {
                // manage if in dnablock
                if s == exitstring { true } else { false }
            },
            Err(_e) => ( false )
        };
        if found { break }
    }
    nom(f, bytes)
}

fn unpack_i32(buf: Vec<u8>, is_big_endian: bool) -> GenResult<i32> {
    let x = match is_big_endian { 
        true => {
            let s = structure!(">i");
            s.unpack(buf)?.0
        },
        false => {
            let s = structure!("<i");
            s.unpack(buf)?.0
        }
    };
    Ok(x)
}

fn read_old_mem(f: &mut File, blender_head_size: usize ) -> GenResult<()> {
    let mut buf = vec![0u8; match blender_head_size {
        20 => 4,
        _ => 8
    }];
    f.read_exact(&mut buf)?; // just skip it
    Ok(())
}

fn loop_blocks(mut f: File, is_big_endian: bool, blender_head_size: usize) -> GenResult<()>{
    // nom till code "DNA1" and read 4 bytes after
    let buf = nom_after(&mut f, 4, "DNA1")?;
    let _blocksize = unpack_i32(buf, is_big_endian)?;
    // Skip old pointer
    read_old_mem(&mut f, blender_head_size)?;
    // SDNA Index
    let buf = nom(&mut f, 4)?;
    let _sdna_index = unpack_i32(buf, is_big_endian)?;
    // Count
    let buf = nom(&mut f, 4)?;
    let _count = unpack_i32(buf, is_big_endian)?;

    nom_till(&mut f, 4, "NAME")?;
    let buf = nom(&mut f, 4)?;
    let _name_count = unpack_i32(buf, is_big_endian)?;
    // let strrr = str::from_utf8(&buf)?;
    // println!("There are {} names", name_count);

    // Read names
    let _buf = nom_till(&mut f, 4, "TYPE")?;
    // let strrr = str::from_utf8(&_buf)?;
    // for (i, s) in strrr.split("\0").enumerate() {
    //     println!("{:<8} {}", i, s);
    // }
    let _ = nom(&mut f, 4)?;

    let buf = nom(&mut f, 4)?;
    let _type_count = unpack_i32(buf, is_big_endian)?;
    // let strrr = str::from_utf8(&buf)?;
    // println!("There are {} names", name_count);

    // Read names
    let buf = nom_till(&mut f, 4, "TLEN")?;
    let strrr = str::from_utf8(&buf)?;
    for (i, s) in strrr.split("\0").enumerate() {
        if i == 145 || i == 2700{
            println!("{:<8} {}", i, s);
        }
    }


    





    // while let Ok(buf) = nom(&mut f, 4){
    //     match str::from_utf8(&buf){
    //         Ok(s) => {
    //             // manage if in dnablock
    //             if !in_dna_block && s == "DNA1" { in_dna_block = true; }
    //             if s == "ENDB"{ in_dna_block = false;}
    //         },
    //         Err(_e) => ()
    //     }
    // }
    Ok(())
}