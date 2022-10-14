use num_modular::ModularInteger;
use num_modular::MontgomeryInt;
use num_modular::Vanilla;
use num_traits::ops::inv::Inv;

// Homomorphic encryption using elliptic curves
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct E2<const ORDER: u32, const P: u32>(Option<(MontgomeryInt<u32>, MontgomeryInt<u32>)>);

impl<const ORDER: u32, const P: u32> E2<ORDER, P> {
    pub fn residue(self) -> Option<(u32, u32)> {
        match self.0 {
            None => None,
            Some((x, y)) => Some((x.residue(), y.residue())),
        }
    }

    pub fn identity() -> Self {
        Self(None)
    }

    pub fn double(self) -> Self {
        let (x, y) = match self.0 {
            None => return self,
            Some((x, y)) => (x, y),
        };

        let s = x * x * 3 * (y * 2).inv();
        let xr = s * s - x * 2;
        let yr = s * (x - xr) - y;
        Self(Some((xr, yr)))
    }
}

impl<const ORDER: u32, const P: u32> From<(u32, u32)> for E2<ORDER, P> {
    fn from((x, y): (u32, u32)) -> Self {
        let x = MontgomeryInt::new(x, &P);
        let y = x.convert(y);
        Self(Some((x, y)))
    }
}

impl<const ORDER: u32, const P: u32> From<(i32, i32)> for E2<ORDER, P> {
    fn from((x, y): (i32, i32)) -> Self {
        use std::ops::Neg;
        let mut xp = MontgomeryInt::new(x.abs() as u32, &P);
        if x < 0 {
            xp = xp.neg();
        }
        let mut yp = xp.convert(y.abs() as u32);
        if y < 0 {
            yp = yp.neg();
        }
        Self(Some((xp, yp)))
    }
}

impl<const ORDER: u32, const P: u32> std::ops::Add<Self> for E2<ORDER, P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (xg, yg, x, y) = match (self.0, rhs.0) {
            (None, _) => return rhs,
            (_, None) => return self,
            (Some((xg, yg)), Some((x, y))) => (xg, yg, x, y),
        };

        // double
        if xg == x && yg == y {
            return self.double();
        }

        let s = (yg - y) * (xg - x).inv();
        let xr = s * s - (x + xg);
        let yr = s * (x - xr) - y;
        Self(Some((xr, yr)))
    }
}

impl<const ORDER: u32, const P: u32> std::ops::Mul<u32> for E2<ORDER, P> {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        if self.0.is_none() {
            return self;
        }

        let mut base = Self::identity();
        for bit in (0..32).filter_map(|e| ((rhs >> e) & 0b1 == 1).then(|| e)) {
            base = base + (0..bit).fold(self, |acc, _| acc.double());
        }
        return base;
    }
}

impl<const ORDER: u32, const P: u32> std::ops::Mul<i32> for E2<ORDER, P> {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        use std::ops::Neg;

        if self.0.is_none() {
            return self;
        }

        let nrhs = rhs.abs() as u32;
        if rhs < 0 {
            self * MontgomeryInt::new(nrhs, &ORDER).neg().residue()
        } else {
            self * nrhs
        }
    }
}
