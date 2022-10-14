mod e1;
mod e2;

fn main() {
    // generator
    //let g = e2::E2::<93, 97>::from((3u32, 15u32));
    let g = e1::E1::<97>::from(5);

    // Verifier's knowledge
    let x = 13;
    let shift = 16;
    let t = x * x - 3 * x + 2; // The target polynomial solved for x
    let s3 = g * (x * x * x);
    let s2 = g * (x * x);
    let s1 = g * (x);
    let s0 = g * (1);
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
    let eh = s1 * h_c0 + g * h_c1;
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
