pub mod float_utils;
use float_utils::{Floaty, FloatBits};

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
    let q = if d < zero() {
        q.ceil()
    } else {
        q.floor()
    };
    // XXX This is almost certainly always the case, but worth
    // checking since currently undocumented.
    assert!(q.denom() >= &zero());
    let r = n - &q * &d;
    let to_f32: Box<dyn Fn(&Q) -> f32> = if exact {
        let to_f32 = |x: &Q| {
            let frac = Q::from_integer((1 << 24).into());
            let rounded = if d < zero() {
                (x * &frac).ceil() / &frac
            } else {
                (x * &frac).floor() / &frac
            };
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

fn divmod_euclid_prop(n0: f32, d0: f32) -> (f32, f32) {
    let n = n0.float_bits();
    let d = d0.float_bits();

    match (n, d) {
        (FloatBits::Value(nf, mut ne), FloatBits::Value(df, mut de)) => {
            ne += 1;
            de += 1;
            eprintln!("n={nf} {ne} d={df} {de}");
            // Fix sign bits and adjust for division.
            // XXX Sign bit fix is currently wrong; need to
            // switch between floor() and ceil().
            // XXX Adjustment is also currently wrong; need
            // to deal with very negative exponents, so shift
            // farther left everywhere.
            let sign = (nf ^ df) >> 24;
            let nf = (nf & !(1 << 24)) << (24 + ne);
            let df = (df & !(1 << 24)) << (24 + de);

            // Calculate quotient.
            let mut qf = nf / df;
            eprintln!("qf={qf}");

            // Align and adjust exponent.
            let qz = qf.leading_zeros() as i64;
            let qs = -127 + qz + 24;
            qf <<= qs;
            eprintln!("qb={}", (qf >> 24) & 1);
            let qe = 127 - qz;
            eprintln!("q={qs} {:06x} {qe}", qf);

            // Hit the floor.
            qf &= !((1 << (24 - qe)) - 1);
            eprintln!("qff={}", qf >> (24 - qe));

            // Calculate remainder.
            let mut rf = (nf - (qf >> (24 - qe)) * df) >> 24;
            eprintln!("rf={rf}");

            // Align and adjust exponent.
            let rz = rf.leading_zeros() as i64;
            rf <<= -127 + rz + 24;
            eprintln!("rb={}", (rf >> 24) & 1);
            let re = 127 - rz - 24;
            eprintln!("r={rz} {rf:03x} {re}");

            // Return floats.
            let make_float = |x, f, e, s| {
                eprintln!(
                    "mf{} {} {:06x} {} {}",
                    x,
                    f as f32 * f32::powf(2.0, (e - 24) as f32),
                    f, 
                    e,
                    s,
                );
                let bits = ((s as u32) << 31)
                    | (((e + 127) as u32 & 0xff) << 23)
                    | ((f >> 1) as u32 & ((1 << 23) - 1));
                eprintln!("bits={:032b}", bits);
                f32::from_bits(bits)
            };
            (make_float("q", qf, qe, sign), make_float("r", rf, re, 0))
        }
        _ => todo!(),
    }
}

pub type D = fn(f32, f32) -> (f32, f32);

#[derive(Debug, Error)]
pub enum DivModError {
    #[error("unknown divmod {0}")]
    Unknown(String),
}

pub fn get_op(name: &str) -> Result<D, DivModError> {
    match name {
        "deff" => Ok(divmod_euclid_deff),
        "std" => Ok(divmod_euclid_std),
        "exactish" => Ok(divmod_euclid_exactish),
        "exact" => Ok(divmod_euclid_exact),
        "prop" => Ok(divmod_euclid_prop),
        n => Err(DivModError::Unknown(n.into())),
    }
}
