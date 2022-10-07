#[derive(Debug, Copy, Clone, PartialEq)]
struct Mod5_97(u32);

fn modpow(b: u32, e: u32, p: u32) -> u32 {
    let l = 8;
    let mut acc = 1u64;
    
    for _ in 0..e/l {
        acc *= u64::pow(b as u64, l);
        acc %= p as u64;
    }
    
    acc *= u64::pow(b as u64, e % l);
    acc %= p as u64;
    
    return acc as _;
}

impl std::ops::Mul<u32> for Mod5_97 {
    type Output = Self;
    fn mul(self, opr: u32) -> Self::Output {
        Self(modpow(self.0, opr, 97))
    }
}

impl std::ops::Mul<i32> for Mod5_97 {
    type Output = Self;
    fn mul(self, opr: i32) -> Self::Output {
        let opr = if opr < 0 { (-opr) as u32 * 95 } else { opr as u32 }; // get a positive conjugate
        self * opr
    }
}

impl std::ops::Add<Self> for Mod5_97 {
    type Output = Self;
    fn add(self, opr: Self) -> Self::Output {
        Self(self.0 * opr.0 % 97)
    }
}

impl From<i32> for Mod5_97 {
    fn from(x: i32) -> Self {
        Self(5) * x
    }
}

fn main() {
    // Verifier's knowledge
    let x = 999;
    let shift = 16;
    let t = x * x - 3 * x + 2; // The target polynomial solved for x
    let s3 = Mod5_97::from(x * x * x);
    let s2 = Mod5_97::from(x * x);
    let s1 = Mod5_97::from(x);
    let s0 = Mod5_97::from(1);
    let s3_shift = Mod5_97::from(x * x * x) * shift;
    let s2_shift = Mod5_97::from(x * x) * shift;
    let s1_shift = Mod5_97::from(x) * shift;
    let s0_shift = Mod5_97::from(1) * shift;

    // To prover:
    // p(x) = x^3 - 6x^2 + 11x - 6
    let p_c0 = 1;
    let p_c1 = -6;
    let p_c2 = 11;
    let p_c3 = -6;
    // h(x) = x - 3
    let h_c0 = 1;
    let h_c1 = -3;
    let s3 = s3; // \
    let s2 = s2; // | Prover is given these...
    let s1 = s1; // /
    let s3_shift = s3_shift; // \
    let s2_shift = s2_shift; // | ...These too
    let s1_shift = s1_shift; // /
    
    // Solve encrypted p, h, and p_shift with s and s_shift factors
    let ep = s3 * p_c0 + s2 * p_c1 + s1 * p_c2 + s0 * p_c3;
    let eh = s1 * h_c0 + Mod5_97::from(h_c1);
    let ep_shift = s3_shift * p_c0 + s2_shift * p_c1 + s1_shift * p_c2 + s0_shift * p_c3;

    // Apply shift so encrypted values sent to verifier cant be guess
    let delta = 39; // Some random delta 
    let ep = ep * delta;
    let eh = eh * delta;
    let ep_shift = ep_shift * delta;

    // Back to verified:
    let correct_roots = eh * t == ep;
    let correct_form = ep * shift == ep_shift;
    assert!(correct_roots);
    assert!(correct_form);
    println!("Pass");
}