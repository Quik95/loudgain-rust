use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::iter::Scan;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

use clap::Parser;
use lazy_static::lazy_static;

use crate::loudness_types::{Decibel, LoudnessUnitFullScale};

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

pub enum ScanMode {
    DontWriteTags,
    DeleteTags,
    WriteTags,
    WriteExtraTags,
    WriteExtraTagsLufs,
}

impl Display for ScanMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ScanMode::DontWriteTags => 's',
            ScanMode::DeleteTags => 'd',
            ScanMode::WriteTags => 'i',
            ScanMode::WriteExtraTags => 'e',
            ScanMode::WriteExtraTagsLufs => 'l',
        };
        write!(f, "{}", res)
    }
}

impl Debug for ScanMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let res = match self {
           ScanMode::DontWriteTags => 's',
           ScanMode::DeleteTags => 'd',
           ScanMode::WriteTags => 'i',
           ScanMode::WriteExtraTags => 'e',
           ScanMode::WriteExtraTagsLufs => 'l',
       };
        write!(f, "{}", res)
    }
}

impl FromStr for ScanMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(ScanMode::DontWriteTags),
            "d" => Ok(ScanMode::DeleteTags),
            "i" => Ok(ScanMode::WriteTags),
            "e" => Ok(ScanMode::WriteExtraTags),
            "l" => Ok(ScanMode::WriteExtraTagsLufs),
            _ => Err(format!("Cannot parse {} into a tag mode.", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author = "Sebastian Bartoszewicz")]
pub struct Args {
    pub files: Vec<String>,

    #[clap(short = 'q', long = "quiet")]
    pub quiet: bool,

    #[clap(short = 'k', long = "noclip")]
    pub no_clip: bool,

    #[clap(short = 'K', long = "maxtpl", default_value_t = LoudnessUnitFullScale::new(- 1.0))]
    pub maxtlp: LoudnessUnitFullScale,

    #[clap(short = 'd', long = "pregain", default_value_t = Decibel::new(0.0))]
    pub pregain: Decibel,

    #[clap(short = 'L', long = "lowercase")]
    pub lowercase_tags: bool,

    #[clap(short = 'r', long = "track")]
    pub track: bool,

    #[clap(short = 's', long = "tagmode", default_value_t = ScanMode::DontWriteTags)]
    pub scan_mode: ScanMode,
}

pub fn build_file_list(files: Vec<String>) -> Vec<String> {
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
        // "aiff",
        // "aif",
        // "alfc",
        // "ape",
        // "apl",
        // "bwf",
        "flac",
        "mp3",
        "mp4",
        "m4a",
        "m4b",
        "m4p",
        "m4r",
        // "mpc",
        "ogg",
        "vorbis",
        // "tta",
        // "wma",
        "wv",
    ]);

    paths.into_iter().filter(|path| {
        let extension = Path::new(&path).extension().expect("To be a file extension").to_str().expect("To be a string slice");
        return if !valid_extensions.contains(extension) {
            if !ARGS.quiet {
                println!("Ignoring the following file due to an unsupported extension: {}", path);
            }
            false
        } else {
            true
        };
    }).collect()
}

fn recursively_expand_directory(file: String) -> Vec<String> {
    let path = Path::new(&file);
    let mut res: Vec<String> = Vec::new();
    // TODO: Fix this to not ignore errors
    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
        res.push(entry.path().to_str().expect("To be a string slice").to_string());
    }
    res
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
