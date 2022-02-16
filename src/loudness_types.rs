use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Decibel(f64);
pub struct LoudnessUnit(f64);
pub struct LoudnessUnitFullScale(f64);
pub struct LinearLoudness(f64);

impl Decibel {
    pub fn new(val: f64) -> Self {
        Decibel(val)
    }
}

impl Debug for Decibel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Decibel{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} dB", self.0)
    }
}

impl LoudnessUnit {
    pub fn new(val: f64) -> Self {
        Self(val)
    }
}

impl Debug for LoudnessUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for LoudnessUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} LU", self.0)
    }
}

impl LoudnessUnitFullScale {
    pub fn new(val: f64) -> Self {
        Self(val)
    }
}

impl Debug for LoudnessUnitFullScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for LoudnessUnitFullScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} LUFS", self.0)
    }
}

impl LinearLoudness {
    pub fn new(val: f64) -> Self {
        Self(val)
    }
}

impl Debug for LinearLoudness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for LinearLoudness {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.7}", self.0)
    }
}

