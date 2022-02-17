use std::fmt;
use std::fmt::{Debug, Formatter};

use ebur128::EbuR128;
use ebur128::Error;

use crate::decode_audio::DecodedFile;
use crate::loudness_types::{Decibel, LinearLoudness, LoudnessUnitFullScale};

#[derive(Debug)]
pub struct ScanResult {
    pub true_peak: LinearLoudness,
    pub loudness_range: Decibel,
    pub integrated_loudness: LoudnessUnitFullScale,
}

impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "True peak: {}\nLoudness Range: {}\nIntegrated Loudness: {}", self.true_peak, self.loudness_range, self.integrated_loudness)
    }
}

impl ScanResult {
    pub fn new(integrated_loudness: f64, loudness_range: f64, true_peak: f64) -> Self {
        ScanResult {
            true_peak: LinearLoudness::new(true_peak),
            loudness_range: Decibel::new(loudness_range),
            integrated_loudness: LoudnessUnitFullScale::new(integrated_loudness)
        }
    }
}

pub fn scan_file(file: DecodedFile) -> Result<ScanResult, Error> {
    let mode = get_mode();

    let mut instance = EbuR128::new(file.channels, file.rate, mode)?;
    instance.add_frames_i16(file.pcm.as_slice())?;

    Ok(ScanResult::new(
        instance.loudness_global()?,
        instance.loudness_range()?,
        instance.true_peak(0)?,
    ))
}

fn get_mode() -> ebur128::Mode {
    let mut mode  = ebur128::Mode::I;
    mode.insert(ebur128::Mode::TRUE_PEAK);
    mode.insert(ebur128::Mode::LRA);

    mode
}

