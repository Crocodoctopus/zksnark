// Homomorphic encryption using powers
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct E1<const P: u32>(u32);

impl<const P: u32> std::ops::Add<Self> for E1<P> {
    type Output = Self;
    fn add(self, opr: Self) -> Self::Output {
        Self(self.0 * opr.0 % 97)
    }
}

impl<const P: u32> std::ops::Mul<u32> for E1<P> {
    type Output = Self;
    fn mul(self, opr: u32) -> Self::Output {
        let b = self.0;
        let l = 8;
        let mut acc = 1u64;

        for _ in 0..opr / l {
            acc *= u64::pow(b as u64, l);
            acc %= P as u64;
        }

        acc *= u64::pow(b as u64, opr % l);
        acc %= P as u64;

        Self(acc as u32)
    }
}

impl<const P: u32> std::ops::Mul<i32> for E1<P> {
    type Output = Self;
    fn mul(self, opr: i32) -> Self::Output {
        let opr = if opr < 0 {
            (-opr) as u32 * 95
        } else {
            opr as u32
        }; // get a positive conjugate
        self * opr
    }
}

impl<const P: u32> From<u32> for E1<P> {
    fn from(base: u32) -> Self {
        Self(base)
    }
}
