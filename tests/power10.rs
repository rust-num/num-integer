extern crate num_integer;
extern crate num_traits;

use num_integer::Power10;
use num_traits::ToPrimitive;

fn log10_compare_with_f64<T: Power10 + ToPrimitive>(x: T) {
    let expected = x.to_f64().unwrap().log10().floor() as u32;
    assert_eq!(x.floor_log10(), expected); //, "{}", x.to_f64().unwrap()
}

macro_rules! unsigned_power10 {
    ($T:ty, $I:ident) => {
        mod $I {
            use super::log10_compare_with_f64;
            use num_integer::checked_next_power_of_ten;
            use num_integer::floor_log10;
            use num_integer::is_power_of_ten;
            use num_integer::wrapping_next_power_of_ten;
            use num_integer::Power10;

            #[test]
            fn test_is_pow_10() {
                assert_eq!((0 as $T).is_power_of_ten(), false);
                assert_eq!(<$T>::max_value().is_power_of_ten(), false);
                let mut x: $T = 1;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(is_power_of_ten(x), true);
                    assert_eq!((x - 1).is_power_of_ten(), false);
                    assert_eq!((x + 1).is_power_of_ten(), false);
                    x *= 10;
                }
                assert_eq!(x.is_power_of_ten(), true);
                assert_eq!((x - 1).is_power_of_ten(), false);
                assert_eq!((x + 1).is_power_of_ten(), false);
            }

            #[test]
            fn test_floor_log10() {
                assert_eq!((0 as $T).floor_log10(), 0);
                log10_compare_with_f64(<$T>::max_value());
                log10_compare_with_f64(<$T>::max_value() - 1);
                log10_compare_with_f64(<$T>::max_value() - 2);
                let mut x: $T = 1;
                let mut count: u32 = 0;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(floor_log10(x), count);
                    if x > 1 {
                        assert_eq!((x - 1).floor_log10(), count - 1);
                    }
                    assert_eq!((x + 1).floor_log10(), count);
                    x *= 10;
                    count += 1;
                }
                assert_eq!(x.floor_log10(), count);
                assert_eq!((x - 1).floor_log10(), count - 1);
                assert_eq!((x + 1).floor_log10(), count);

                //powers of 2
                x = 1;
                while x != 0 {
                    log10_compare_with_f64(x);
                    log10_compare_with_f64(x + 1);
                    log10_compare_with_f64(x - 1);
                    x <<= 1;
                }
            }

            #[test]
            fn test_wrap_next_power_10() {
                assert_eq!((0 as $T).wrapping_next_power_of_ten(), 1);
                assert_eq!(<$T>::max_value().wrapping_next_power_of_ten(), 0);
                assert_eq!(
                    (<$T>::max_value() - 1).wrapping_next_power_of_ten(),
                    0,
                    "max - 1"
                );
                assert_eq!(
                    (<$T>::max_value() - 2).wrapping_next_power_of_ten(),
                    0,
                    "max - 2"
                );
                let mut x: $T = 1;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(wrapping_next_power_of_ten(x), x);
                    if x > 1 {
                        assert_eq!((x - 1).wrapping_next_power_of_ten(), x);
                    }
                    assert_eq!((x + 1).wrapping_next_power_of_ten(), x * 10);
                    x *= 10;
                }
                assert_eq!(x.wrapping_next_power_of_ten(), x);
                assert_eq!((x - 1).wrapping_next_power_of_ten(), x);
                assert_eq!((x + 1).wrapping_next_power_of_ten(), 0);
        
                //powers of 2
                x = 4;
                let mut pow10 = 10;
                while x != 0 {
                    assert_eq!(x.wrapping_next_power_of_ten(), pow10);
                    assert_eq!((x + 1).wrapping_next_power_of_ten(), pow10);
                    assert_eq!((x - 1).wrapping_next_power_of_ten(), pow10);
                    x <<= 1;
                    if x > pow10 {
                        if pow10 > end {
                            pow10 = 0;
                        } else {
                            pow10 *= 10;
                        }
                    }
                }
            }
        
            #[test]
            #[should_panic]
            fn test_next_power_10_panic1() {
                assert_eq!(<$T>::max_value().next_power_of_ten(), 0);
            }
        
            #[test]
            #[should_panic]
            fn test_next_power_10_panic2() {
                assert_eq!((<$T>::max_value() - 1).next_power_of_ten(), 0);
            }
        
            #[test]
            fn test_checked_next_power_10() {
                assert_eq!((0 as $T).checked_next_power_of_ten(), Some(1));
                assert_eq!(<$T>::max_value().checked_next_power_of_ten(), None);
                assert_eq!(
                    (<$T>::max_value() - 1).checked_next_power_of_ten(),
                    None
                );
                assert_eq!(
                    (<$T>::max_value() - 2).checked_next_power_of_ten(),
                    None
                );
                let mut x: $T = 1;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(checked_next_power_of_ten(x), Some(x));
                    if x > 1 {
                        assert_eq!((x - 1).checked_next_power_of_ten(), Some(x));
                    }
                    assert_eq!(
                        (x + 1).checked_next_power_of_ten(),
                        Some(x * 10)
                    );
                    x *= 10;
                }
                assert_eq!(x.checked_next_power_of_ten(), Some(x));
                assert_eq!((x - 1).checked_next_power_of_ten(), Some(x));
                assert_eq!((x + 1).checked_next_power_of_ten(), None);
        
                //powers of 2
                x = 4;
                let mut pow10 = 10;
                while x != 0 {
                    let c = if pow10 == 0 { None } else { Some(pow10) };
                    assert_eq!(x.checked_next_power_of_ten(), c);
                    assert_eq!((x + 1).checked_next_power_of_ten(), c);
                    assert_eq!((x - 1).checked_next_power_of_ten(), c);
                    x <<= 1;
                    if x > pow10 {
                        if pow10 > end {
                            pow10 = 0;
                        } else {
                            pow10 *= 10;
                        }
                    }
                }
            }
        }
    };
}

unsigned_power10!(u8, u8);
unsigned_power10!(u16, u16);
unsigned_power10!(u32, u32);
unsigned_power10!(u64, u64);
#[cfg(has_i128)]
unsigned_power10!(u128, u128);
unsigned_power10!(usize, usize);

// this function generates the table used for floor_log10
// run with --release to avoid overflow
// it then has to be reversed (I used tac)
//#[test]
//fn print_digits() {
//    println!("({}, {}), ", 0, 1);
//    let mut x: u128 = 1;
//    let mut count: u32 = 0;
//    let mut pow10: u128 = 10;
//    while x != 0 {
//        print!("({}, {}), ", count, pow10);
//        let digits = (x as f64).log10().floor() as u32;
//        x <<= 1;
//        if x != 0 {
//            let next_digits = (x as f64).log10().floor() as u32;
//            if next_digits > digits {
//                count += 1;
//                pow10 *= 10;
//                println!();
//            }
//        }
//    }
//    println!("");
//}
