// #![allow(unused)]
use clap::Parser;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::os::unix::ffi::OsStrExt;

fn add2file(path: &[u8], hist: &str) -> Result<(), BErr> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(hist)?;
    file.write_all(path)?; //.expect_err("Failed to write");
    file.write_all(b"\n")?; //.expect_err("Failed to write");
    Ok(())
}


#[derive(Parser)]
struct Cli {
    pattern: String,
    // #[clap(parse(from_os_str))]
    // path: std::path::PathBuf,
}

type BErr = Box<dyn std::error::Error>;

pub fn path_buf_to_u8_slice_unix(input: &PathBuf) -> &[u8] {
    input.as_os_str().as_bytes()
}

fn mark(_args: Cli) -> Result<(), BErr> {
    let path = env::current_dir()?;
    let hist =  get_hist_path()?;
    if ! std::path::Path::new(&hist).exists() {
        create_hist(&hist)?;
    }
    add2file(path_buf_to_u8_slice_unix(&path), &hist)?;
    Ok(())
}

fn show(_args: Cli) -> Result<(), BErr> {
    let hist =  get_hist_path()?;
    if std::path::Path::new(&hist).exists() {
        let contents = std::fs::read_to_string(hist)?;
        for line in contents.lines() {
            println!("{}", line);
        }
    }
    Ok(())
}

fn create_hist(hist: &str) -> Result<(), BErr> {
    let path = std::path::Path::new(hist);
    let prefix = path.parent().unwrap();
    // println!("{:?}",prefix);
    std::fs::create_dir_all(prefix)?;
    std::fs::File::create(path)?;
    Ok(())
}

fn get_hist_path() -> Result<String, BErr> {
    let home = env::var("HOME")?;
    let hist = home + "/.cache/path_marker/hist.txt";
    Ok(hist)
}

fn main() -> Result<(), BErr> {
    let args = Cli::parse();
    match &*args.pattern {
        "mark" => mark(args),
        "show" => show(args),
        _ => panic!("There was a problem reading arguments")
    }
}
