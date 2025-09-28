use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MetricValue {
    unit: MetricUnit,
    absolute: f32,
    prefix: MetricPrefix,
}
impl MetricValue {
    pub fn new(numerical_absolute_value: f32, prefix: MetricPrefix, unit: MetricUnit) -> Self {
        Self {
            absolute: numerical_absolute_value,
            unit: unit,
            prefix: prefix,
        }
    }
    pub fn convert_to(self, new_metric_prefix: MetricPrefix) -> Self {
        let conversion_scale: f32 = self.prefix.to(new_metric_prefix.clone());
        Self {
            unit: self.unit,
            absolute: (self.absolute * conversion_scale),
            prefix: new_metric_prefix,
        }
    }
}

impl ToString for MetricValue {
    fn to_string(&self) -> String {
        format!(
            "{} {}{}",
            self.absolute.to_string(),
            self.prefix.to_string(),
            self.unit.to_string()
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MetricUnit {
    Meter,
    Seconds,
    Coulomb,
}

impl ToString for MetricUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Meter => "m",
            Self::Coulomb => "C",
            Self::Seconds => "s",
            _ => "unknown",
        }
        .to_owned()
    }
}
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MetricPrefix {
    quecto = -30,
    ronto = -27,
    yocto = -24,
    zepto = -21,
    atto = -18,
    femto = -15,
    pico = -12,
    nano = -9,
    micro = -6,
    milli = -3,
    centi = -2,
    deci = -1,
    si = 0,
    deca = 1,
    hecto = 2,
    kilo = 3,
    mega = 6,
    giga = 9,
    tera = 12,
    peta = 15,
    exa = 18,
    zetta = 21,
    yotta = 24,
    rotta = 27,
    quetta = 30,
}

impl MetricPrefix {
    fn to(self, to: MetricPrefix) -> f32 {
        Self::conversion(self, to)
    }

    fn conversion(from_mpref: MetricPrefix, to_mpref: MetricPrefix) -> f32 {
        f32::powf(10.0, (from_mpref as isize - to_mpref as isize) as f32)
    }
}

impl ToString for MetricPrefix {
    fn to_string(&self) -> String {
        match self {
            Self::quecto => "q",
            Self::ronto => "r",
            Self::yocto => "y",
            Self::zepto => "z",
            Self::atto => "a",
            Self::femto => "f",
            Self::pico => "p",
            Self::nano => "n",
            Self::micro => "μ",
            Self::milli => "m",
            Self::centi => "c",
            Self::deci => "d",
            Self::si => "",
            Self::deca => "da",
            Self::hecto => "h",
            Self::kilo => "k",
            Self::mega => "M",
            Self::giga => "G",
            Self::tera => "T",
            Self::peta => "P",
            Self::exa => "E",
            Self::zetta => "Z",
            Self::yotta => "Y",
            Self::rotta => "R",
            Self::quetta => "Q",
            _ => "unkown",
        }
        .to_owned()
    }
}
