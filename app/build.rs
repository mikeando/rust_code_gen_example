// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");

    //TODO: Should this be a pow(1.0/3.0)?
    let cbrt_33_mul_3: f64 = 3.0 * 33.0f64.powf(0.5);

    let tribonacci_constant: f64 =
        1.0 + (19.0 - cbrt_33_mul_3).powf(1.0 / 3.0) + (19.0 + cbrt_33_mul_3).powf(1.0 / 3.0);

    let p = tribonacci_constant / 3.0;
    let s = 1.0
        / ((4.0 / 3.0) * tribonacci_constant - (1.0 / 9.0) * tribonacci_constant.powf(2.0) - 1.0);

    fs::write(
        &dest_path,
        format!(
            "\
        pub mod tribonacci {{\n\
            pub const P: f64 = {:.32};\n\
            pub const S: f64 = {:.32};\n\
        }}\n",
            p, s
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
