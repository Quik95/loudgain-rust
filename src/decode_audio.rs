use std::{fmt, fs};
use std::error::Error;
use std::fs::File;
use std::path::Path;

use rodio::decoder::Decoder;
use rodio::Source;

pub struct DecodedFile {
    pub pcm: Vec<i16>,
    pub channels: u32,
    pub rate: u32,
}

impl DecodedFile {
    pub fn new(pcm: Vec<i16>, channels: u32, rate: u32) -> Self {
        Self { pcm, channels, rate }
    }
}

impl fmt::Debug for DecodedFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pcm.len() = {}, channels: {}, sample_rate: {}", self.pcm.len(), self.channels, self.rate)
    }
}

pub fn decode_file(file_path: &str) -> Result<DecodedFile, Box<dyn Error>> {
    let audio_file = read_audio_file(file_path)?;
    let decoder = Decoder::new(audio_file)?;
    let channels = decoder.channels();
    assert_ne!(channels, 0);

    let rate = decoder.sample_rate();
    Ok(DecodedFile::new(decoder.collect(), channels as u32, rate))
}

fn read_audio_file(path: &str) -> std::io::Result<fs::File> {
    let path = Path::new(path);
    File::open(path)
}