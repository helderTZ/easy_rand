use std::os::raw::c_uint;
use std::os::raw::c_int;

extern "C" {
    fn srand(seed: c_uint);
    fn rand() -> c_int;
}

pub fn c_srand(seed: u32) {
    unsafe { srand(seed as c_uint); };
}

pub fn c_rand() -> i32 {
    unsafe { rand() }
}

pub fn c_rand_range(min: i32, max: i32) -> i32 {
    unsafe { rand() % (max - min) + min }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        c_srand(0);
        let _ = c_rand();
    }

    #[test]
    fn c_rand_range_unsigned() {
        c_srand(0);
        for _ in 0..1000 {
            let result = c_rand_range(1,10);
            assert!(result >= 1, "result wasn't greater than 1, was {}", result);
            assert!(result < 10, "result wasn't lesser than 10, was {}", result);
        }
    }

    #[test]
    fn c_rand_range_signed() {
        c_srand(0);
        for _ in 0..1000 {
            let result = c_rand_range(-10,10);
            assert!(result >= -10, "result wasn't greater than -10, was {}", result);
            assert!(result < 10, "result wasn't lesser than 10, was {}", result);
        }
    }
}
