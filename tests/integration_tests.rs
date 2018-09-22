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

    #[test]
    fn version() {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("tests");
        buf.push("resources");
        buf.push("default_2.79.blend");
        let path = buf.clone().into_os_string().into_string().expect("Unwrapping pathbuf basic failed");
        let version = match check_blend(&path[..]){
            Ok(v) => v,
            Err(e) => e.to_string()
        };
        assert_eq!(version, "2.79");
    }
}
