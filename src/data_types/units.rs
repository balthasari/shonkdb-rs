use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LengthUnit {
    mm,
    cm,
    m,
    km,
    inch,
    foot,
    mile,
}
#[derive(Serialize, Deserialize)]
pub enum TimeUnit {
    ms,
    s,
    min,
    h,
    d,
}

#[derive(Serialize, Deserialize)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}
