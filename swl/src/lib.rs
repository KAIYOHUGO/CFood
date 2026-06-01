#[unsafe(no_mangle)]
extern "C" fn power_both_side(a: f64, b: f64) -> f64 {
    a.powf(b) + b.powf(a)
}
