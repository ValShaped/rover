#![allow(unused_imports)]

use overmount::btrfs::format::{ChecksumAlgorithm, Formatter};
use overmount::Result as OvResult;
use std::{
    env::args,
    path::{Path, PathBuf},
    process::Output,
};

fn main() -> OvResult<()> {
    // let args: Vec<String> = args().collect();
    // if args.len() < 2 {
    //     return Ok(());
    // }
    // let file = PathBuf::from(&args[1]);
    // let rootdir = PathBuf::from("./testdir");
    //create_btrfs_image(file, rootdir, "label")?;
    dump_args()?;

    //let mut path = PathBuf::from(args())
    Ok(())
}

#[allow(dead_code)]
fn create_btrfs_image(path: PathBuf, rootdir: PathBuf, label: &str) -> OvResult<()> {
    let formatter = Formatter::options()
        .rootdir(rootdir)?
        .label(label)?
        .shrink()?
        .mixed()?
        .force()?
        .finalize();
    let Output {
        status: _status,
        stdout: out,
        stderr: err,
    } = formatter.format(&path)?;
    println!(
        "> STDOUT:\n{}\n> STDERR:\n{}",
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    );
    Ok(())
}

#[allow(dead_code)]
fn dump_args() -> OvResult<()> {
    use overmount::btrfs::format::*;
    let output = Formatter::options()
        .byte_count(536_870_912_u64)?
        .checksum(ChecksumAlgorithm::Crc32c)?
        .data(DataProfile::Dup)?
        .features(["mixed-bg"])?
        .force()? // true if called
        .label("label-label")?
        .metadata(DataProfile::Dup)?
        .mixed()? // true if called
        .no_discard()? // true if called
        .nodesize(4096_usize)?
        .rootdir(PathBuf::from("./overmount/src"))?
        .runtime_features(["quota"])?
        .sectorsize(4096_usize)?
        .shrink()? // true if called
        .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")?
        .dump_args()
        .finalize()
        .format(&PathBuf::from("test.btrfs"))?;

    println!(
        "> STDOUT:\n{}\n> STDERR:\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );
    Ok(())
}

#[allow(dead_code)]
fn try_example_code() -> OvResult<()> {
    use overmount::btrfs::format::*;
    let formatter = Formatter::options()
        .byte_count(536_870_912_u64)?
        .checksum(ChecksumAlgorithm::Crc32c)?
        .data(DataProfile::Dup)?
        .features(["mixed-bg"])?
        .force()? // true if called
        .label("label")?
        .metadata(DataProfile::Dup)?
        .mixed()? // true if called
        .no_discard()? // true if called
        .nodesize(4096_usize)?
        .rootdir(PathBuf::from("./testdir"))?
        .runtime_features(["quota"])?
        .sectorsize(4096_usize)?
        .shrink()? // true if called
        .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")?
        .finalize();
    let Output {
        status: _status,
        stdout: out,
        stderr: err,
    } = formatter.format(&PathBuf::from("./test.btrfs"))?;
    println!(
        "> STDOUT:\n{}\n> STDERR:\n{}",
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    );
    Ok(())
}
