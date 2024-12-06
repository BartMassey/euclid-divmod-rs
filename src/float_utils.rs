#[derive(Debug, PartialEq, Eq)]
pub enum FloatBits {
    Value(u128, i64),
    Denorm(u128),
    Nan(u128),
    Inf(u128),
}

pub trait Floaty {
    fn float_bits(&self) -> FloatBits;
    fn prec(&self) -> (u32, u32);
}

impl Floaty for f32 {
    fn float_bits(&self) -> FloatBits {
        let bits = self.to_bits();
        let sign = bits >> 31;
        let exp = (bits >> 23) & 0xff;
        let f = bits & ((1 << 23) - 1);
        match exp {
            0 => {
                // Denormalized number, including 0.
                let f = f | (sign << 24);
                FloatBits::Denorm(f as u128)
            }
            e if e == (1 << self.prec().1) - 1 => {
                // Not really a number.
                match f {
                    0 => FloatBits::Inf(sign as u128),
                    f => FloatBits::Nan((f | (sign << 24)) as u128),
                }
            }
            e => {
                // "Normal" number.
                let f = f | (1 << 23) | (sign << 24);
                FloatBits::Value(f as u128, e as i64 - 127)
            }
        }
    }
    
    fn prec(&self) -> (u32, u32) {
        (25, 8)
    }
}

pub struct DisplayFloat<T: Floaty>(pub T);

impl<T: Floaty> std::fmt::Display for DisplayFloat<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (nf, _ne) = self.0.prec();
        let sign = |s| {
            if s & 1 == 0 {
                "+"
            } else {
                "-"
            }
        };
        match self.0.float_bits() {
            FloatBits::Value(f, e) => {
                let s = sign(f >> (nf - 1));
                let f = f & ((1 << (nf - 2)) - 1);
                write!(fmt, "{}1.{:0w$b}b{:+}", s, f, e, w = nf as usize - 2)
            }
            FloatBits::Denorm(f) => {
                let s = sign(f >> (nf - 1));
                let f = f & ((1 << (nf - 2)) - 1);
                write!(fmt, "{}0.{:0w$b}b-127", s, f, w = nf as usize - 2)
            }
            FloatBits::Inf(s)  => {
                let s = sign(s);
                write!(fmt, "{}∞", s)
            }
            FloatBits::Nan(_)  => {
                write!(fmt, "NaN")
            }
        }
    }
}

#[test]
fn test_float_rep_f32() {
    let try_real = |x: f32, fb: FloatBits, r: &str| {
        assert_eq!(x.float_bits(), fb, "{}", x);
        assert_eq!(&format!("{}", DisplayFloat(x)), r, "{}", x);
    };

    try_real(
        1.0,
        FloatBits::Value(1 << 23, 0),
        "+1.00000000000000000000000b+0",
    );
    try_real(
        -0.5,
        FloatBits::Value(0b11 << 23, -1),
        "-1.00000000000000000000000b-1",
    );
    try_real(
        -0.0,
        FloatBits::Denorm(1 << 24),
        "-0.00000000000000000000000b-127",
    );
    try_real(
        -1.0 / 0.0,
        FloatBits::Inf(1),
        "-∞",
    );

    let x = -0.0 / 0.0;
    assert!(matches!(x.float_bits(), FloatBits::Nan(_)));
    assert_eq!("NaN", &format!("{}", DisplayFloat(x)));
}
