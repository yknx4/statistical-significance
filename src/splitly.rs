use wasm_bindgen::prelude::*;
fn cdf_norm(x: f64) -> f64 {
  let a = 0.0498673470;
  let b = 0.0211410061;
  let c = 0.0032776263;
  let d = 0.0000380036;
  let e = 0.0000488906;
  let f = 0.0000053830;
  let x_abs = x.abs();
  let mut t = 1.0 + x_abs * (a + x_abs * (b + x_abs * (c + x_abs * (d + x_abs * (e + x_abs * f)))));
  t *= t;
  t *= t;
  t *= t;
  t *= t;
  t = 1.0 / (t + t);
  if x >= 0.0 {
    t = 1.0 - t;
  }
  return t
}

#[wasm_bindgen]
pub fn splitly(original_tests: f64, original_successes: f64, variant_tests: f64, variant_successes: f64) -> f64 {
  let original_mean = original_successes / original_tests;
  let variant_mean = variant_successes / variant_tests;
  let p_value: f64;
  if original_mean == variant_mean {
    p_value = 0.5;
  } else {
    let std_error = ((original_mean * (1.0 - original_mean) / original_tests) + (variant_mean * (1.0 - variant_mean) / variant_tests)).sqrt();
    let z_value = (variant_mean - original_mean) / std_error;
    p_value = cdf_norm(z_value);
  }
  return p_value
}
