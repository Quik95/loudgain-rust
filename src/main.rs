use std::collections::HashSet;
use std::fs;
use std::path::{Path};
use std::process::exit;

use clap::Parser;

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let songs = build_file_list(args.files);
    dbg!(songs);
}

#[derive(Parser, Debug)]
#[clap(author = "Sebastian Bartoszewicz")]
struct Args {
    files: Vec<String>,
}

fn build_file_list(files: Vec<String>) -> Vec<String> {
    check_for_invalid_paths(&files);

    let expanded_directories = get_files_from_folders_recursively(files);
    let absolute_paths = make_paths_absolute(expanded_directories);

    check_for_invalid_extension(absolute_paths)
}

fn get_files_from_folders_recursively(files: Vec<String>) -> Vec<String> {
    files.into_iter().flat_map(recursively_expand_directory).collect()
}

fn make_paths_absolute(files: Vec<String>) -> Vec<String> {
    files.into_iter().map(|file| fs::canonicalize(file).expect("To be an absolute path.").to_str().expect("To be a string slice.").to_string()).collect()
}

fn check_for_invalid_extension(paths: Vec<String>) -> Vec<String> {
    let valid_extensions = HashSet::from([
        "aiff",
        "aif",
        "alfc",
        "ape",
        "apl",
        "bwf",
        "flac",
        "mp3",
        "mp4",
        "m4a",
        "m4b",
        "m4p",
        "m4r",
        "mpc",
        "ogg",
        "tta",
        "wma",
        "wv",
    ]);

    paths.into_iter().filter(|path| {
        let extension = Path::new(&path).extension().expect("To be a file extension").to_str().expect("To be a string slice");
            if !valid_extensions.contains(extension) {
                println!("Ignoring the following file due to an unsupported extension: {}", path);
               false
            } else {
               true
            }
    }).collect()
}

fn recursively_expand_directory(file: String) -> Vec<String> {
    let path = Path::new(&file);
    if path.is_dir() {
        path.read_dir()
            .expect("To return directory contents")
            .map(|entry| {
                let path = entry.expect("To be a directory entry").path();
                return if path.is_dir() {
                    recursively_expand_directory(path.to_str().expect("To be &str").to_string())
                } else {
                    vec![path.to_str().expect("To be path").to_string()]
                };
            })
            .flatten()
            .collect()
    } else {
        vec![file.to_string()]
    }
}

fn check_for_invalid_paths(files: &[String]) {
    let invalid_files: Vec<_> = files
        .iter()
        .filter(|file| !Path::new(file).exists())
        .collect();
    if !invalid_files.is_empty() {
        invalid_files
            .iter()
            .for_each(|file| println!("File not found: {}", file));
        exit(1);
    }
}
