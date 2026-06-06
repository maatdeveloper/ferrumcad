/* standard library */

/* external crates */

/* ferrumcad crates */
pub trait Transform: Sized {
    fn translate(&self, dx: f64, dy: f64) -> Self;
    fn scale(&self, factor: f64) -> Self;
}
