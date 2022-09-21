// #![allow(unused)]
use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

extern crate confy;
extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

macro_rules! post_inc {
    ($i:ident) => {{
        let old = $i;
        $i += 1;
        old
    }};
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfyConfig {
    history_max_size: usize,
}
impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            history_max_size: 10,
        }
    }
}

fn head_border(len: usize, d: usize) -> usize {
    if len < d {
        0
    } else {
        len - d
    }
}
fn enqueue(path: &[u8], hist: &str, cfg: ConfyConfig) -> Result<(), BErr> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(hist)?;

    let mut contents = String::new();
    file.rewind()?;
    file.read_to_string(&mut contents)?;

    let path_str = std::str::from_utf8(path)?;
    let mut contents_list = contents.lines().collect::<Vec<_>>();
    contents_list.push(path_str);

    let mut cnt = 0;
    let contents_set = contents_list
        .into_iter()
        .map(|x| (x, post_inc!(cnt)))
        .collect::<HashMap<&str, usize>>();

    let mut contents_list = contents_set.into_iter().collect::<Vec<(&str, usize)>>();
    contents_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let contents_sliced = &contents_list[head_border(contents_list.len(), cfg.history_max_size)..];

    let contents_buf = contents_sliced
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<&str>>()
        .join("\n");

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hist)?;

    file.rewind()?;
    file.write_all(contents_buf.as_bytes())?; //.expect_err("Failed to write");
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

fn mark(_args: Cli, cfg: ConfyConfig) -> Result<(), BErr> {
    let path = env::current_dir()?;
    let hist = get_hist_path()?;
    if !std::path::Path::new(&hist).exists() {
        create_hist(&hist)?;
    }
    enqueue(path_buf_to_u8_slice_unix(&path), &hist, cfg)?;
    Ok(())
}

fn show(_args: Cli) -> Result<(), BErr> {
    let hist = get_hist_path()?;
    if std::path::Path::new(&hist).exists() {
        let contents = std::fs::read_to_string(hist)?;
        for line in contents.lines().rev() {
            println!("{}", line);
        }
    }
    Ok(())
}

fn create_hist(hist: &str) -> Result<(), BErr> {
    let path = std::path::Path::new(hist);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;
    std::fs::File::create(path)?;
    Ok(())
}

fn get_hist_path() -> Result<String, BErr> {
    let home = env::var("HOME")?;
    let hist = home + "/.cache/path_marker/hist.txt";
    Ok(hist)
}

fn conf(cfg: ConfyConfig) -> Result<(), BErr> {
    let file = confy::get_configuration_file_path("path-marker", None)?;
    println!("The configuration file path is: {:#?}", file);
    println!("The configuration is:");
    println!("{:#?}", cfg);
    Ok(())
}

fn main() -> Result<(), BErr> {
    let cfg: ConfyConfig = confy::load("path-marker", None)?;
    let args = Cli::parse();
    match &*args.pattern {
        "mark" => mark(args, cfg),
        "show" => show(args),
        "conf" => conf(cfg),
        _ => panic!("There was a problem reading arguments"),
    }
}
