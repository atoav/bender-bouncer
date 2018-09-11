extern crate bender_bouncer;




/// Test basic validation
mod validity{
    use bender_bouncer::check_blend;
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("default_2.79.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let valid = match check_blend(&path[..]){
            Ok(s) => {
                println!("{:?}", s);
                true
            },
            Err(e) =>{
                println!("{:?}", e);
                false
            }
        };
        assert!(valid);
    }

    #[test]
    fn catch_invalid() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("invalid.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let valid = match check_blend(&path[..]){
            Ok(s) => {
                println!("{:?}", s);
                true
            },
            Err(e) =>{
                println!("{:?}", e);
                false
            }
        };
        assert!(!valid);
    }
}

/// Test frame readouts
mod frames{
    use bender_bouncer::parse_scenes;
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("default_2.79.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let data = match parse_scenes(&path[..]){
            Ok(s) => Some(s),
            Err(e) => {
                println!("{}", e);
                None
            }
        }.expect("Unwrapping of parsed_scenes() failed");
        println!("{:?}", data);
        let start = data.get("Scene").expect("Failed to unwrap FrameRange.start").start;
        let end = data.get("Scene").expect("Failed to unwrap FrameRange.end").end;
        let count = data.get("Scene").expect("Failed to unwrap FrameRange.count()").count();
        assert_eq!(1, start);
        assert_eq!(250, end);
        assert_eq!(249, count);
    }

    #[test]
    fn big_frame_number() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("1048574frames_2.79.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let data = match parse_scenes(&path[..]){
            Ok(s) => Some(s),
            Err(e) => {
                println!("{}", e);
                None
            }
        }.expect("Unwrapping of parsed_scenes() failed");
        println!("{:?}", data);
        let start = data.get("Scene").expect("Failed to unwrap FrameRange.start").start;
        let end = data.get("Scene").expect("Failed to unwrap FrameRange.end").end;
        let count = data.get("Scene").expect("Failed to unwrap FrameRange.count()").count();
        assert_eq!(0, start);
        assert_eq!(1048574, end);
        assert_eq!(1048574, count);
    }

    #[test]
    fn frame_step() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("100_frames_step10.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let data = match parse_scenes(&path[..]){
            Ok(s) => Some(s),
            Err(e) => {
                println!("{}", e);
                None
            }
        }.expect("Unwrapping of parsed_scenes() failed");
        println!("{:?}", data);
        let start = data.get("Scene").expect("Failed to unwrap FrameRange.start").start;
        let end = data.get("Scene").expect("Failed to unwrap FrameRange.end").end;
        let count = data.get("Scene").expect("Failed to unwrap FrameRange.count()").count();
        assert_eq!(0, start);
        assert_eq!(100, end);
        assert_eq!(10, count);
    }
}