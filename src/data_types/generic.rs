use super::units::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Length(f32, LengthUnit);

#[derive(Serialize, Deserialize)]
pub struct Date {
    year: isize,
    month: Month,
    day: isize,
}
