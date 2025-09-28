use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct MetricValue {
    unit: MetricUnit,
    absolute: f32,
    prefix: MetricPrefix,
}
impl MetricValue {
    fn convert_to(self, new_metric_prefix: MetricPrefix) -> Self {
        let conversion_scale: f32 = self.prefix.to(new_metric_prefix.clone());
        Self {
            unit: self.unit,
            absolute: (self.absolute * conversion_scale),
            prefix: new_metric_prefix,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
enum MetricUnit {
    Meter,
    Seconds,
    Coulomb,
}
#[derive(Serialize, Deserialize, Clone)]
enum MetricPrefix {
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
        f32::powf(10.0, (to_mpref as usize - from_mpref as usize) as f32)
    }
}
