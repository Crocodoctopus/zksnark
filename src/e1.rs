use num_modular::MontgomeryInt;
use num_modular::ModularInteger;
use num_traits::Pow;
use num_traits::Inv;

// Homomorphic encryption using powers
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct E1<const P: u32>(MontgomeryInt<u32>);

impl<const P: u32> E1<P> {
    pub fn residue(&self) -> u32 {
        self.0.residue()
    }
}

impl<const P: u32> std::ops::Add<Self> for E1<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<const P: u32> std::ops::Mul<u32> for E1<P> {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0.pow(rhs))
    }
}

impl<const P: u32> std::ops::Mul<i32> for E1<P> {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        if rhs < 0 {
            let rhs = rhs.abs() as u32;
            Self(self.0.pow(rhs).inv())
        } else {
            let rhs = rhs as u32;
            Self(self.0.pow(rhs))
        }
    }
}

impl<const P: u32> From<u32> for E1<P> {
    fn from(base: u32) -> Self {
        Self(MontgomeryInt::new(base, &P))
    }
}