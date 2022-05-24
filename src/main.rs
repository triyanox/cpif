mod get_date_created;
mod get_file_name;
mod get_file_size;
mod get_file_type;
mod get_last_modified;

use get_date_created::get_date_created;
use get_file_name::get_file_name;
use get_file_size::get_file_size;
use get_file_type::get_file_type;
use get_last_modified::get_last_modified;

use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct FileInfo {
    name: String,
    path: String,
    size: u64,
}

fn get_file_info(path: &str) -> FileInfo {
    let name = get_file_name(path);
    let size = get_file_size(path);
    FileInfo {
        name,
        path: path.to_string(),
        size: size.len() as u64,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file path provided");
        return;
    }
    let path = &args[1];
    if !std::fs::metadata(path).is_ok() {
        println!("File does not exist");
        return;
    }
    if args.len() == 3 && args[2] == "--copy" {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        clipboard::ClipboardProvider::set_contents(&mut ctx, contents).unwrap();
        println!("Copied to clipboard");
    } else if args.len() == 3 && args[2] == "--info" || args.len() == 2 {
        let info = get_file_info(path);
        println!("Name : {}", info.name);
        println!("Path : {}", info.path);
        println!("Size : {} KB", info.size);
        println!("Type : {}", get_file_type(path));
        println!("Created at : {}", get_date_created(path));
    } else if args.len() == 3 && args[2] == "--size" {
        let size = get_file_size(path);
        println!("Size : {} KB", size.len());
    } else if args.len() == 3 && args[2] == "--type" {
        println!("Type {}", get_file_type(path));
    } else if args.len() == 3 && args[2] == "--created" {
        println!("Created at : {}", get_date_created(path));
    } else if args.len() == 3 && args[2] == "--name" {
        println!("Path : {}", get_file_name(path));
    } else if args.len() == 3 && args[2] == "--modified" {
        println!("Modified at : {}", get_last_modified(path));
    } else if args.len() == 2 && args[1] == "--help" {
        println!("Usage: cpif <path> [--copy] [--info] [--size] [--type] [--created] [--name] [--modified]");
        println!("--copy : Copy file content to clipboard");
        println!("--info : Print file info");
        println!("--size : Print file size");
        println!("--type : Print file type");
        println!("--created : Print file created at");
        println!("--name : Print file name");
        println!("--modified : Print file modified at");
    } else {
        println!("Usage: cpif <path> [--copy] [--info] [--size] [--type] [--created] [--name] [--modified]");
    }
}
