use crate::args::ARGS;
use crate::loudness_types::{Decibel, LinearLoudness, LoudnessUnitFullScale};

pub fn calculate_gain(int_loudness: LoudnessUnitFullScale, true_peak: LinearLoudness) -> Decibel {
    let reference_loudness = LoudnessUnitFullScale::new(-18.0);
    let pregain = Decibel::new(0.0);

    let gain = (reference_loudness - int_loudness).as_dB() + pregain;

    if ARGS.no_clip { avoid_clipping(gain, true_peak) } else { gain }
}

fn avoid_clipping(gain: Decibel, true_peak: LinearLoudness) -> Decibel {
    let peak_after_gain = gain.as_linear() * true_peak;
    let mtpl = LoudnessUnitFullScale::new(-1.0);

    if peak_after_gain > mtpl.as_linear() {
        gain - (peak_after_gain / mtpl.as_linear()).as_dB()
    } else {peak_after_gain.as_dB()}
}