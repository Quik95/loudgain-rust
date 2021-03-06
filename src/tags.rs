use std::fs;
use std::path::Path;

use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};
use tempfile::{Builder, NamedTempFile};

use crate::args::{ARGS, ScanMode};
use crate::replaygain_scanner::TrackGain;

const RG_TRACK_GAIN: &str = "REPLAYGAIN_TRACK_GAIN";
const RG_TRACK_PEAK: &str = "REPLAYGAIN_TRACK_PEAK";
const RG_ALBUM_GAIN: &str = "REPLAYGAIN_ALBUM_GAIN";
const RG_ALBUM_PEAK: &str = "REPLAYGAIN_ALBUM_PEAK";
const RG_TRACK_RANGE: &str = "REPLAYGAIN_TRACK_RANGE";
const RG_ALBUM_RANGE: &str = "REPLAYGAIN_ALBUM_RANGE";
const RG_REFERENCE_LOUDNESS: &str = "REPLAYGAIN_REFERENCE_LOUDNESS";

const RG_TRACK_GAIN_LOWERCASE: &str = "replaygain_track_gain";
const RG_TRACK_PEAK_LOWERCASE: &str = "replaygain_track_peak";
const RG_ALBUM_GAIN_LOWERCASE: &str = "replaygain_album_gain";
const RG_ALBUM_PEAK_LOWERCASE: &str = "replaygain_album_peak";
const RG_TRACK_RANGE_LOWERCASE: &str = "replaygain_track_range";
const RG_ALBUM_RANGE_LOWERCASE: &str = "replaygain_album_range";
const RG_REFERENCE_LOUDNESS_LOWERCASE: &str = "replaygain_reference_loudness";

const RG_TRACK_GAIN_OPUS: &str = "R128_TRACK_GAIN";
const RG_ALBUM_GAIN_OPUS: &str = "R128_ALBUM_GAIN";

pub fn save_tags(tags: &TrackGain) -> Result<(), std::io::Error> {
    match ARGS.scan_mode {
        ScanMode::DontWriteTags => return Ok(()),
        ScanMode::DeleteTags => return remove_rg_tags(&tags.filepath),
        _ => (),
    };

    let extension = get_file_extension(&tags.filepath);
    let formatted = format_tags(tags, extension);
    let new_file = ffmpeg_write_tags(&tags.filepath, formatted).expect("To be a copy of a song with the replaygain tags written to it.");
    swap_files(&tags.filepath, new_file.path().to_str().expect("To be a string slice")).unwrap();
    println!("Hello");

    if ARGS.strip_tags {
        strip_non_rg_metadata(&tags.filepath)?;
    }

    Ok(())
}

fn strip_non_rg_metadata(filepath: &str) -> Result<(), std::io::Error> {
    // abuse a little bit the fact that ffmpeg_write_tags takes a vector of strings to pass instead of
    // tags -map_metadata -1 which tells ffmpeg to remove metadata.
    // Be sure to call this function BEFORE writing scan results, as this will also remove them
    let new_file = ffmpeg_write_tags(filepath, vec!["-map_metadata".to_string(), "-1".to_string()]).expect("To work");
    swap_files(filepath, new_file.path().to_str().expect("To be a string slice."))
}

fn remove_rg_tags(filepath: &str) -> Result<(), std::io::Error> {
    let tags = match get_file_extension(filepath) {
        // for some reason currently doesn't work for opus. Maybe a ffmpeg bug?
        "ogg" => vec![
            "-metadata".to_string(), format!("{}=", RG_TRACK_GAIN_OPUS),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_GAIN_OPUS),
        ],
        _ => vec![
            "-metadata".to_string(), format!("{}=", RG_TRACK_GAIN),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_GAIN),
            "-metadata".to_string(), format!("{}=", RG_TRACK_GAIN_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_GAIN_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_TRACK_PEAK),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_PEAK),
            "-metadata".to_string(), format!("{}=", RG_TRACK_PEAK_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_PEAK_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_REFERENCE_LOUDNESS),
            "-metadata".to_string(), format!("{}=", RG_REFERENCE_LOUDNESS_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_TRACK_RANGE),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_RANGE),
            "-metadata".to_string(), format!("{}=", RG_TRACK_RANGE_LOWERCASE),
            "-metadata".to_string(), format!("{}=", RG_ALBUM_RANGE_LOWERCASE),
        ]
    };

    let new_file = ffmpeg_write_tags(filepath, tags).expect("To be a song with ReplayGain tags removed.");
    swap_files(filepath, new_file.path().to_str().expect("To be a string slice"))
}

fn swap_files(old: &str, new: &str) -> Result<(), std::io::Error> {
    // it's fine to copy, because the temporary file will be deleted when it goes out of scope
    // it might not be deleted if the program terminates abruptly, but it will be in a temp dir anyway
    let topdir = Path::new(old).parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Parent directory does not exists"))?;
    let name = Path::new(new).file_name().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "File does not have a name"))?;
    let new_path = topdir.join(name);
    fs::copy(new, &new_path)?;

    fs::rename(new_path, old)
}

fn get_file_extension(path: &str) -> &str {
    let p = Path::new(path);
    p.extension().expect("To be a file extension").to_str().expect("To be a string slice")
}

fn ffmpeg_write_tags(filepath: &str, tags: Vec<String>) -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let extension = get_file_extension(filepath);
    let temp_file = Builder::new().prefix("loudgain-").suffix(&format!(".{}", extension)).tempfile()?;

    let popen_args = [
        vec!["ffmpeg".to_string(),
             "-hide_banner".to_string(),
             "-i".to_string(),
             filepath.to_string(),
             "-map".to_string(),
             "0".to_string(),
             "-y".to_string(),
             "-codec".to_string(),
             "copy".to_string()], tags, vec![temp_file.path().to_str().expect("To be a string slice").to_string()]].concat();

    dbg!(popen_args.join(" "));

    let mut p = Popen::create(&popen_args, PopenConfig {
        stdin: Redirection::Pipe,
        stdout: Redirection::Pipe,
        stderr: Redirection::Pipe,
        detached: false,
        executable: None,
        env: None,
        cwd: None,
        setuid: None,
        setgid: None,
        setpgid: false,
        _use_default_to_construct: (),
    })?;
    let exit_code = p.wait()?;

    match exit_code {
        ExitStatus::Exited(_) => Ok(temp_file),
        _ => Err("Incorrect exit status".into()),
    }
}

fn format_tags(tags: &TrackGain, extension: &str) -> Vec<String> {
    let lufs = matches!(ARGS.scan_mode, ScanMode::WriteExtraTagsLufs);

    let mut res = match extension {
        "ogg" => vec![
            // as to replicate the loudgain behavior we don't write track peak tags.
            // also extra tags are not allowed in Opus
            "-metadata".to_string(), format!("{}={}", RG_TRACK_GAIN_OPUS, tags.gain.to_q78num()),
        ],
        _ => vec![
            "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags { RG_TRACK_GAIN_LOWERCASE } else { RG_TRACK_GAIN }, if !lufs { tags.gain.to_string() } else { tags.gain.as_LU().to_string() }),
            "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags { RG_TRACK_PEAK_LOWERCASE } else { RG_TRACK_PEAK }, tags.true_peak),
        ],
    };

    if extension != "ogg" && matches!(ARGS.scan_mode, ScanMode::WriteExtraTags) || lufs {
        res.append(&mut vec![
            "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags { RG_TRACK_RANGE_LOWERCASE } else { RG_TRACK_RANGE }, if !lufs { tags.range.to_string() } else { tags.range.as_LU().to_string() }),
            // Reference loudness is in LUFS already
            "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags { RG_REFERENCE_LOUDNESS_LOWERCASE } else { RG_REFERENCE_LOUDNESS }, tags.reference_loudness),
        ]);
    }

    res
}