pub fn round_float(val: f64, decimals: Option<u32>) -> f64 {
    match decimals {
        Some(places) => {
            let factor = 10_f64.powi(places as i32);
            (val * factor).round() / factor
        }
        None => val,
    }
}
