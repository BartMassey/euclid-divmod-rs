pub mod float_utils;

use num::{
    rational::BigRational as Q,
    traits::identities::zero,
    ToPrimitive,
};
use thiserror::Error;

fn divmod_euclid_deff(n: f32, d: f32) -> (f32, f32) {
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

fn divmod_euclid_rational(n0: f32, d0: f32, exact: bool) -> (f32, f32) {
    assert!(d0 != 0.0);
    let n = Q::from_float(n0).unwrap();
    let d = Q::from_float(d0).unwrap();
    let q = &n / &d;
    // XXX This is almost certainly always the case, but worth
    // checking since currently undocumented.
    assert!(q.denom() >= &zero());
    let r = n - &q * d;
    let to_f32: Box<dyn Fn(&Q) -> f32> = if exact {
        let to_f32 = |x: &Q| {
            let frac = Q::from_integer(u32::pow(2, 24).into());
            let rounded = (x * &frac).floor() / &frac;
            rounded.to_f32().unwrap()
        };
        Box::new(to_f32)
    } else {
        // This may not produce the exact answer, since the
        // rounding of `to_f32()` is unknown but probably
        // round-to-nearest-even. Still, good enough in most
        // situations.
        Box::new(|x| x.to_f32().unwrap())
    };
    (to_f32(&q), to_f32(&r))
}

fn divmod_euclid_exactish(n0: f32, d0: f32) -> (f32, f32) {
    divmod_euclid_rational(n0, d0, false)
}

fn divmod_euclid_exact(n0: f32, d0: f32) -> (f32, f32) {
    divmod_euclid_rational(n0, d0, true)
}

pub type D = fn(f32, f32) -> (f32, f32);

#[derive(Debug, Error)]
pub enum DivModError {
    #[error("unknown divmod $0")]
    Unknown(String),
}

pub fn get_op(name: &str) -> Result<D, DivModError> {
    match name {
        "deff" => Ok(divmod_euclid_deff),
        "std" => Ok(divmod_euclid_std),
        "exactish" => Ok(divmod_euclid_exactish),
        "exact" => Ok(divmod_euclid_exact),
        n => Err(DivModError::Unknown(n.into())),
    }
}
