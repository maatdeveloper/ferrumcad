pub const EPS: f64 = 1e-9;

pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

pub fn approx_zero(a: f64) -> bool {
    a.abs() < EPS
}
