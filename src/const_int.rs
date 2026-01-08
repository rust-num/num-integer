macro_rules! const_impl_signed_integer {
    ($t:ty as $mod:ident) => {
        /// Integer functions that can be used in a `const` context.
        pub mod $mod {

            /// Calculates the Greatest Common Divisor (GCD) of two integers. 
            /// The result is always non-negative.
            pub const fn gcd(mut m: $t, mut n: $t) -> $t {
                while n != 0 {
                    (m, n) = (n, m % n);
                }
                m.abs()
            }

            /// Calculates the Lowest Common Multiple (LCM) of two integers.
            pub const fn lcm(m: $t, n: $t) -> $t {
                if m == 0 || n == 0 {
                    return 0;
                }
                (m * (n / gcd(m, n))).abs()
            }

            #[cfg(test)]
            pub mod const_test {
                use super::*;

                #[test]
                fn test_gcd() {
                    const GCD: $t = gcd(12, 8);
                    assert_eq!(GCD, 4);
                }

                #[test]
                fn test_lcm() {
                    const LCM: $t = lcm(12, 8);
                    assert_eq!(LCM, 24);
                }
            }
        }
    };
}

const_impl_signed_integer!(i8 as i8);
const_impl_signed_integer!(i16 as i16);
const_impl_signed_integer!(i32 as i32);
const_impl_signed_integer!(i64 as i64);
const_impl_signed_integer!(i128 as i128);
