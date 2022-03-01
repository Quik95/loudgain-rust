use std::fs;
use std::path::Path;

use crate::args::ARGS;

use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};
use tempfile::{Builder, NamedTempFile};

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

pub fn save_tags(tags: &TrackGain) -> Result<(), std::io::Error> {
    let formatted = format_tags(tags);
    let new_file = ffmpeg_write_tags(tags.filepath, formatted).expect("To be a copy of a song with the replagain tags written to it.");
    swap_files(tags.filepath, new_file.path().to_str().expect("To be a string slice"))?;
    Ok(())
}

fn swap_files(old: &str, new: &str) -> Result<u64, std::io::Error> {
    //TODO: make it safer by renaming after the copying has finished instead of overwriting

    // it's fine to copy, because the temporary file will be deleted when it goes out of scope
    // it might not be deleted the program terminates abruptly, but it will be in a temp dir anyway
    fs::copy(new, old)
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

fn format_tags(tags: &TrackGain) -> Vec<String> {
    let formatted = vec![
        "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags {RG_TRACK_GAIN_LOWERCASE} else {RG_TRACK_GAIN}, tags.gain),
        "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags {RG_TRACK_PEAK_LOWERCASE} else {RG_TRACK_PEAK}, tags.true_peak),
        "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags {RG_TRACK_RANGE_LOWERCASE} else {RG_TRACK_RANGE}, tags.range),
        "-metadata".to_string(), format!("{}={}", if ARGS.lowercase_tags {RG_REFERENCE_LOUDNESS_LOWERCASE} else {RG_REFERENCE_LOUDNESS}, tags.reference_loudness),
    ];

    formatted
}