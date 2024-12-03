use thiserror::Error;

fn divmod_euclid_naive(n: f32, d: f32) -> (f32, f32) {
    let q = if d > 0.0 {
        // Division should round toward negative infinity to
        // avoid double-rounding.
        (n / d).floor()
    } else if d < 0.0 {
        // Division should round toward positive infinity to
        // avoid double-rounding.
        (n / d).ceil()
    } else {
        // Division by zero will return -inf, inf or
        // NaN. Might as well return a zero remainder in any
        // case: it's probably the most sensible answer.
        return (n / d, 0.0);
    };
    // FMA should round toward zero to avoid being out-of-range.
    let r = -q.mul_add(d, -n);
    (q, r)
}

fn divmod_euclid_std(n: f32, d: f32) -> (f32, f32) {
    (n.div_euclid(d), n.rem_euclid(d))
}

pub type D = fn(f32, f32) -> (f32, f32);

#[derive(Debug, Error)]
pub enum DivModError {
    #[error("unknown divmod $0")]
    Unknown(String),
}


pub fn get_op(name: &str) -> Result<D, DivModError> {
    match name {
        "naive" => Ok(divmod_euclid_naive),
        "std" => Ok(divmod_euclid_std),
        n => Err(DivModError::Unknown(n.into())),
    }
}
