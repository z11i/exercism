// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASELINE_PRODUCED: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (BASELINE_PRODUCED * speed as u32) as f64 * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    f64::floor(production_rate_per_hour(speed) / 60 as f64) as u32
}

fn success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77,
    }
}
