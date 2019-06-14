// Copyright 2018 fishers_exact Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Fisher's exact test.
//!
//! Implements a 2×2 Fishers exact test. Use this to test the independence of two
//! categorical variables when the sample sizes are small.
//!
//! For an approachable explanation of Fisher's exact test, see
//! [Fisher's exact test of independence](http://www.biostathandbook.com/fishers.html) by
//! John H. McDonald in the [Handbook of Biological Statistics](http://www.biostathandbook.com/).
//!
//! The test is computed using code ported from Øyvind Langsrud's JavaScript
//! implementation at [http://www.langsrud.com/fisher.htm](http://www.langsrud.com/fisher.htm),
//! used with permission.
use wasm_bindgen::prelude::*;

fn lngamm(z: i32) -> f64
// Reference: "Lanczos, C. 'A precision approximation
// of the gamma function', J. SIAM Numer. Anal., B, 1, 86-96, 1964."
// Translation of  Alan Miller's FORTRAN-implementation
// See http://lib.stat.cmu.edu/apstat/245
{
    let z = z as f64;
    let mut x = 0.0;
    x += 0.1659470187408462e-06 / (z + 7.0);
    x += 0.9934937113930748e-05 / (z + 6.0);
    x -= 0.1385710331296526 / (z + 5.0);
    x += 12.50734324009056 / (z + 4.0);
    x -= 176.6150291498386 / (z + 3.0);
    x += 771.3234287757674 / (z + 2.0);
    x -= 1259.139216722289 / (z + 1.0);
    x += 676.5203681218835 / (z);
    x += 0.9999999999995183;
    x.ln() - 5.58106146679532777 - z + (z - 0.5) * (z + 6.5).ln()
}

fn lnfact(n: i32) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    lngamm(n + 1)
}

fn lnbico(n: i32, k: i32) -> f64 {
    lnfact(n) - lnfact(k) - lnfact(n - k)
}

fn hyper_323(n11: i32, n1_: i32, n_1: i32, n: i32) -> f64 {
    (lnbico(n1_, n11) + lnbico(n - n1_, n_1 - n11) - lnbico(n, n_1)).exp()
}

fn hyper(s: &mut HyperState, n11: i32) -> f64 {
    hyper0(s, n11, 0, 0, 0)
}

struct HyperState {
    n11: i32,
    n1_: i32,
    n_1: i32,
    n: i32,
    prob: f64,
    valid: bool,
}

impl HyperState {
    fn new() -> HyperState {
        HyperState {
            n11: 0,
            n1_: 0,
            n_1: 0,
            n: 0,
            prob: 0.0,
            valid: false,
        }
    }
}

fn hyper0(s: &mut HyperState, n11i: i32, n1_i: i32, n_1i: i32, ni: i32) -> f64 {
    if s.valid && (n1_i | n_1i | ni) == 0 {
        if !(n11i % 10 == 0) {
            if n11i == s.n11 + 1 {
                s.prob *= ((s.n1_ - s.n11) as f64 / n11i as f64)
                    * ((s.n_1 - s.n11) as f64 / (n11i + s.n - s.n1_ - s.n_1) as f64);
                s.n11 = n11i;
                return s.prob;
            }
            if n11i == s.n11 - 1 {
                s.prob *= ((s.n11 as f64) / (s.n1_ - n11i) as f64)
                    * ((s.n11 + s.n - s.n1_ - s.n_1) as f64 / (s.n_1 - n11i) as f64);
                s.n11 = n11i;
                return s.prob;
            }
        }
        s.n11 = n11i;
    } else {
        s.n11 = n11i;
        s.n1_ = n1_i;
        s.n_1 = n_1i;
        s.n = ni;
        s.valid = true
    }
    s.prob = hyper_323(s.n11, s.n1_, s.n_1, s.n);
    return s.prob;
}

// Returns prob,sleft,sright,sless,slarg
fn exact(n11: i32, n1_: i32, n_1: i32, n: i32) -> (f64, f64, f64, f64, f64) {
    let mut sleft: f64;
    let mut sright: f64;
    let sless: f64;
    let slarg: f64;
    let mut p: f64;
    let mut i;
    let mut j;
    let prob: f64;
    let mut max = n1_;
    if n_1 < max {
        max = n_1;
    }
    let mut min = n1_ + n_1 - n;
    if min < 0 {
        min = 0;
    }
    if min == max {
        return (1.0, 1.0, 1.0, 1.0, 1.0);
    }
    let mut s = HyperState::new();
    prob = hyper0(&mut s, n11, n1_, n_1, n);
    sleft = 0.0;
    p = hyper(&mut s, min);
    i = min + 1;
    while p <= 0.99999999 * prob {
        sleft += p;
        p = hyper(&mut s, i);
        i += 1;
    }
    i -= 1;
    if p <= 1.00000001 * prob {
        sleft += p;
    } else {
        i += 1;
    }
    sright = 0.0;
    p = hyper(&mut s, max);
    j = max - 1;
    while p <= 0.99999999 * prob {
        sright += p;
        p = hyper(&mut s, j);
        j -= 1;
    }
    j += 1;
    if p <= 1.00000001 * prob {
        sright += p;
    } else {
        j += 1;
    }
    if (i - n11).abs() < (j - n11).abs() {
        sless = sleft;
        slarg = 1.0 - sleft + prob;
    } else {
        sless = 1.0 - sright + prob;
        slarg = sright;
    }
    return (prob, sleft, sright, sless, slarg);
}

/// `FishersExactPvalues` holds the pvalues calculated by the `fishers_exact` function.
#[derive(Clone, Copy, Debug)]
pub struct FishersExactPvalues {
    /// pvalue for the two-tailed test. Use this when there is no prior alternative.
    pub two_tail_pvalue: f64,
    /// pvalue for the "left" or "lesser" tail. Use this when the alternative to
    /// independence is that there is negative association between the variables.
    /// That is, the observations tend to lie in lower left and upper right.
    pub less_pvalue: f64,
    /// Use this when the alternative to independence is that there is positive
    /// association between the variables. That is, the observations tend to lie
    /// in upper left and lower right.
    pub greater_pvalue: f64,
}

fn exact22(n11: i32, n12: i32, n21: i32, n22: i32) -> FishersExactPvalues {
    let (left, right, mut twotail);

    let n1_ = n11 + n12;
    let n_1 = n11 + n21;
    let n = n11 + n12 + n21 + n22;
    let (_, sleft, sright, sless, slarg) = exact(n11, n1_, n_1, n);
    left = sless;
    right = slarg;
    twotail = sleft + sright;
    if twotail > 1.0 {
        twotail = 1.0;
    }

    return FishersExactPvalues {
        two_tail_pvalue: twotail,
        less_pvalue: left,
        greater_pvalue: right,
    };
}

/// Computes the Fisher's exact pvales to determine if there are nonrandom associations between two
/// categorical variables, in a two by two contingency table.
///
/// The test is computed using code ported from Øyvind Langsrud's JavaScript
/// implementation at [http://www.langsrud.com/fisher.htm](http://www.langsrud.com/fisher.htm).
///
/// Use this when sample sizes are small. For large samples, other statistical tests of independence
/// are more appropriate.
///
/// # Examples
/// ```
/// use fishers_exact::fishers_exact;
///
/// let p = fishers_exact(&[1,9,11,3]).unwrap();
///
/// assert!((p.less_pvalue - 0.001346).abs() < 0.0001);
/// assert!((p.greater_pvalue - 0.9999663).abs() < 0.0001);
/// assert!((p.two_tail_pvalue - 0.0027594).abs() < 0.0001);
/// ```
///
#[wasm_bindgen]
// pub fn fishers_exact(table: &[u32; 4]) -> Result<FishersExactPvalues,TooLargeValueError> {
pub fn fishers_exact_two_tail(original_tests: f64, original_successes: f64, variant_tests: f64, variant_successes: f64) -> f64 {
    let result = exact22(original_successes as i32, variant_successes as i32, (original_tests - original_successes) as i32, (variant_tests - variant_successes) as i32);
    return result.two_tail_pvalue
}

#[wasm_bindgen]
pub fn fishers_exact_less_value(original_tests: f64, original_successes: f64, variant_tests: f64, variant_successes: f64) -> f64 {
    let result = exact22(original_successes as i32, variant_successes as i32, (original_tests - original_successes) as i32, (variant_tests - variant_successes) as i32);
    return result.less_pvalue
}

#[wasm_bindgen]
pub fn fishers_exact_greater_value(original_tests: f64, original_successes: f64, variant_tests: f64, variant_successes: f64) -> f64 {
    let result = exact22(original_successes as i32, variant_successes as i32, (original_tests - original_successes) as i32, (variant_tests - variant_successes) as i32);
    return result.greater_pvalue
}
