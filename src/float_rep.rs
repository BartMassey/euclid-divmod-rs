#[derive(Debug)]
pub enum FloatBits {
    Value(u128, i64),
    Nan(u64),
    Inf(u64),
}

pub trait FloatRep {
    fn float_bits(&self) -> FloatBits;
    fn prec(&self) -> (u32, u32);
}

impl FloatRep for f32 {
    fn float_bits(&self) -> FloatBits {
        let bits = self.to_bits();
        let sign = bits >> 31;
        let frac = (bits & ((1 << 23) - 1)) | (1 << 23) | (sign << 24);
        let exp = (bits >> 23) & 0xff;
        FloatBits::Value(frac as u128, exp as i64 - 127)
    }
    
    fn prec(&self) -> (u32, u32) {
        (24, 8)
    }
}

#[test]
fn test_float_rep_f32() {
    let try_real = |x: f32| {
        match x.float_bits() {
            FloatBits::Value(f, e) => (f, e),
            v => panic!("float {} not real: {:?}", x, v),
        }
    };

    let r = try_real(1.0);
    assert_eq!(r, (1 << 23, 0));
}
