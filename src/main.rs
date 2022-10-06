#[derive(Debug, Copy, Clone, PartialEq)]
struct Mod5_97(u32);

fn modpow(b: u32, e: u32, p: u32) -> u32 {
    let l = 3;
    let mut acc = 1;
    
    for _ in 0..e/l {
        acc *= u32::pow(b, l);
        acc %= p;
    }
    
    acc *= u32::pow(b, e % l);
    acc %= p;
    
    return acc;
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
    let x = 999;
    
    
    let ep0 = Mod5_97::from(x * x * x) * 1;
    let ep1 = Mod5_97::from(x * x) * -6;
    let ep2 = Mod5_97::from(x) * 11;
    let ep3 = Mod5_97::from(-6);
    let ep = ep0 + ep1 + ep2 + ep3;
    
    let eh0 = Mod5_97::from(x) * 1;
    let eh1 = Mod5_97::from(-3);
    let eh = eh0 + eh1;
    
    let t = x * x - 3 * x + 2;
    
    println!("{:?} {:?}", ep, eh * t); 
}