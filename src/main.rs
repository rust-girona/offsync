use std::process;
use std::path::{Path, PathBuf};
use std::io;

mod img_st;
use img_st::*;

fn main() {
    let mut dir = String::new();
    let mut img_map : ImgMap = ImgMap::new();

    println!("Introduce directory for images:");
    let result = io::stdin().read_line(&mut dir);
    load_dir(&mut img_map, PathBuf::from(dir.trim())).unwrap();
    img_map.print();
}
