

pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

pub mod ls {
    use lazy_static::lazy_static; // 1.4.0

    lazy_static! {
        //TODO: Should this be a pow(1.0/3.0)?
        pub static ref cbrt_33_mul_3: f64 = 3.0 * 33.0f64.powf(0.5);
        
        pub static ref tribonacci_constant: f64 = 1.0
        + (19.0 - *cbrt_33_mul_3).powf(1.0 / 3.0)
        + (19.0 + *cbrt_33_mul_3).powf(1.0 / 3.0);
    }

    pub fn tribonacci(n: f64) -> f64 {
        return (
            (*tribonacci_constant / 3.0).powf(n)
            / (
                (4.0 / 3.0)
                * *tribonacci_constant
                - (1.0 / 9.0)
                * tribonacci_constant.powf(2.0) - 1.0
            )
        ).round();
    }

}

pub mod hc {
    pub fn tribonacci(n: f64) -> f64 {
        let p = super::constants::tribonacci::P;
        let s = super::constants::tribonacci::S;
        return (s * p.powf(n)).round();
    }
}

pub mod mc {
    use trib_macros::{ts,tp};

    pub fn tribonacci(n: f64) -> f64 {
        return (ts!() * tp!().powf(n)).round();
    }
}


#[cfg(test)]
mod tests {
    use crate::ls;

    #[test]
    fn it_works() {

        println!("{}", *ls::tribonacci_constant / 3.0);
        println!("{}", 1.0 / (
            (4.0 / 3.0)
            * *ls::tribonacci_constant
            - (1.0 / 9.0)
            * ls::tribonacci_constant.powf(2.0) - 1.0
        ));
    }
}
