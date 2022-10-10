mod e32;

type Enc = e32::E32<5, 97>;

fn main() {
    // Verifier's knowledge
    let x = 999;
    let shift = 16;
    let t = x * x - 3 * x + 2; // The target polynomial solved for x
    let s3 = Enc::from(x * x * x);
    let s2 = Enc::from(x * x);
    let s1 = Enc::from(x);
    let s0 = Enc::from(1);
    let s3_shift = s3 * shift;
    let s2_shift = s2 * shift;
    let s1_shift = s1 * shift;
    let s0_shift = s0 * shift;

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
    let eh = s1 * h_c0 + Enc::from(h_c1);
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
