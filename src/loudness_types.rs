use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::ParseFloatError;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Decibel(f64);

#[derive(Copy, Clone)]
pub struct LoudnessUnit(f64);

#[derive(Copy, Clone)]
pub struct LoudnessUnitFullScale(f64);

#[derive(Copy, Clone)]
pub struct LinearLoudness(f64);

impl Decibel {
    pub fn new(val: f64) -> Self {
        Decibel(val)
    }
    #[allow(non_snake_case)] pub fn as_LUFS(&self) -> LoudnessUnitFullScale { LoudnessUnitFullScale::new(self.0) }
    #[allow(non_snake_case)] pub fn as_LU(&self) -> LoudnessUnit { LoudnessUnit::new(self.0) }
    pub fn as_linear(&self) -> LinearLoudness { LinearLoudness::new((self.0 / 20.0).powi(10)) }
    pub fn to_q78num(&self) -> i32 { (self.0 * 256.0).round() as i32 }
}

impl Add for Decibel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}

impl Sub for Decibel {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0) }
}

impl FromStr for Decibel {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with(" dB") {
            Ok(Decibel::new(s[0..s.len() - 3].parse::<f64>()?))
        } else {
            Ok(Decibel::new(s.parse::<f64>()?))
        }
    }
}

impl Debug for Decibel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Decibel {
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
    #[allow(non_snake_case)] pub fn as_dB(&self) -> Decibel { Decibel::new(self.0) }
    pub fn as_linear(&self) -> LinearLoudness { self.as_dB().as_linear() }
}

impl FromStr for LoudnessUnitFullScale {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with(" LUFS") {
            Ok(LoudnessUnitFullScale::new(s[0..s.len() - 5].parse::<f64>()?))
        } else {
            Ok(LoudnessUnitFullScale::new(s.parse::<f64>()?))
        }
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

impl Sub for LoudnessUnitFullScale {
    type Output = LoudnessUnitFullScale;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.0 - rhs.0) }
}

impl Add for LoudnessUnitFullScale {
    type Output = LoudnessUnitFullScale;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}

impl LinearLoudness {
    pub fn new(val: f64) -> Self {
        Self(val)
    }
    #[allow(non_snake_case)] pub fn as_dB(&self) -> Decibel { Decibel::new(20.0 * self.0.log10()) }
}

impl Add for LinearLoudness {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.0 + rhs.0) }
}

impl Mul for LinearLoudness {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output { Self::new(self.0 * rhs.0) }
}

impl PartialOrd for LinearLoudness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.0 > other.0 { Ordering::Greater } else if self.0 < other.0 { Ordering::Less } else { Ordering::Equal })
    }
}

impl PartialEq<Self> for LinearLoudness {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl Div for LinearLoudness {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output { Self::new(self.0 / rhs.0) }
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

