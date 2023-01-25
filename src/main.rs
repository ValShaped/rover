#![allow(unused_imports)]

use overmount::btrfs::format;
use std::{
    env::args,
    path::{Path, PathBuf},
    process::Output,
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();

    if let [file, label] = &args[1..=2] {
        let file = PathBuf::from(&file);
        let Output {
            status: _status,
            stdout: out,
            stderr: err,
        } = format(&file, &label)?;
        dbg!(
            String::from_utf8(out).unwrap(),
            String::from_utf8(err).unwrap(),
        );
    };

    //let mut path = PathBuf::from(args())
    Ok(())
}
