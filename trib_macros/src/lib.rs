extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn tp(_item: TokenStream) -> TokenStream {
    let cbrt_33_mul_3: f64 = 3.0 * 33.0f64.powf(0.5);

    let tribonacci_constant: f64 =
        1.0 + (19.0 - cbrt_33_mul_3).powf(1.0 / 3.0) + (19.0 + cbrt_33_mul_3).powf(1.0 / 3.0);

    let p = tribonacci_constant / 3.0;
    format!("{}f64", p).parse().unwrap()
}

#[proc_macro]
pub fn ts(_item: TokenStream) -> TokenStream {
    let cbrt_33_mul_3: f64 = 3.0 * 33.0f64.powf(0.5);

    let tribonacci_constant: f64 =
        1.0 + (19.0 - cbrt_33_mul_3).powf(1.0 / 3.0) + (19.0 + cbrt_33_mul_3).powf(1.0 / 3.0);

    let s = 1.0
        / ((4.0 / 3.0) * tribonacci_constant - (1.0 / 9.0) * tribonacci_constant.powf(2.0) - 1.0);
    format!("{}f64", s).parse().unwrap()
}
