#[derive(Debug, PartialEq, Eq)]
pub enum FloatBits {
    Value(u128, i64),
    Nan(u64),
    Inf(u64),
}

pub trait Floaty {
    fn float_bits(&self) -> FloatBits;
    fn prec(&self) -> (u32, u32);
}

impl Floaty for f32 {
    fn float_bits(&self) -> FloatBits {
        let bits = self.to_bits();
        let sign = bits >> 31;
        let frac = (bits & ((1 << 23) - 1)) | (1 << 23) | (sign << 24);
        let exp = (bits >> 23) & 0xff;
        FloatBits::Value(frac as u128, exp as i64 - 127)
    }
    
    fn prec(&self) -> (u32, u32) {
        (25, 8)
    }
}

pub struct DisplayFloat<T: Floaty>(pub T);

impl<T: Floaty> std::fmt::Display for DisplayFloat<T> {
    fn fmt(&self, ff: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (nf, _ne) = self.0.prec();
        let (f, e) = match self.0.float_bits() {
            FloatBits::Value(f, e) => (f, e),
            _ => todo!(),
        };
        let sign = if f >> (nf - 1) == 0 {
            "+"
        } else {
            "-"
        };
        let f = f & ((1 << (nf - 1)) - 1);
        write!(ff, "{}{:0w$b}e{:+}", sign, f, e, w = nf as usize - 1)
    }
}

#[test]
fn test_float_rep_f32() {
    let try_real = |x: f32, fb: FloatBits, r: &str| {
        assert_eq!(x.float_bits(), fb);
        assert_eq!(&format!("{}", DisplayFloat(x)), r);
    };

    try_real(
        1.0,
        FloatBits::Value(1 << 23, 0),
        "+100000000000000000000000e+0",
    );
    try_real(
        -0.5,
        FloatBits::Value(0b11 << 23, -1),
        "-100000000000000000000000e-1",
    );
}
