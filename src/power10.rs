use Integer;

/// Provides methods to compute functions related to powers of 10.
pub trait Power10: Integer {
    /// Returns `true` if the number is a power of 10.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.is_power_of_ten(), true);
    /// assert_eq!(4u32.is_power_of_ten(), false);
    /// ~~~
    fn is_power_of_ten(&self) -> bool;

    /// Returns the base 10 logarithm value, truncated down.
    /// Panics if the input is zero.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.log10(), 2);
    /// assert_eq!(4u32.log10(), 0);
    /// ~~~
    fn log10(&self) -> u32; //note: u32 return type to allow BigInt types to implement. 10^(2^32) is 4 billion digits

    /// Returns the base 10 logarithm value, truncated down.
    /// Panics for zero.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.checked_log10(), Some(2));
    /// assert_eq!(4u32.checked_log10(), Some(0));
    /// assert_eq!(0u32.checked_log10(), None);
    /// ~~~
    fn checked_log10(&self) -> Option<u32>;

    /// Returns a power of ten greater than or equal to the supplied value.
    /// If the next power of ten is larger than `max_value()`, 0 is returned.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.wrapping_next_power_of_ten(), 100);
    /// assert_eq!(4u32.wrapping_next_power_of_ten(), 10);
    /// assert_eq!(20_000u16.wrapping_next_power_of_ten(), 0);
    /// ~~~
    fn wrapping_next_power_of_ten(&self) -> Self;

    /// Returns a power of ten greater than or equal to the supplied value.
    /// If the next power of ten is larger than `max_value()`, None is returned.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.checked_next_power_of_ten().unwrap(), 100);
    /// assert_eq!(4u32.checked_next_power_of_ten().unwrap(), 10);
    /// assert_eq!(20_000u16.checked_next_power_of_ten(), None);
    /// ~~~
    fn checked_next_power_of_ten(&self) -> Option<Self>;

    /// Returns a power of ten greater than or equal to the supplied value.
    /// If the next power of ten is larger than `max_value()`, 0 is returned in release mode.
    /// If the next power of ten is larger than `max_value()`, the function will panic in debug mode.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.checked_next_power_of_ten().unwrap(), 100);
    /// assert_eq!(4u32.checked_next_power_of_ten().unwrap(), 10);
    /// assert_eq!(20_000u16.checked_next_power_of_ten(), None);
    /// ~~~
    fn next_power_of_ten(&self) -> Self;
}

/// Returns `true` if the number is a power of 10.
#[inline]
pub fn is_power_of_ten<T: Power10>(x: T) -> bool {
    x.is_power_of_ten()
}

/// Returns the base 10 logarithm value, truncated down.
#[inline]
pub fn log10<T: Power10>(x: T) -> u32 {
    x.log10()
}

/// Returns the base 10 logarithm value, truncated down.
#[inline]
pub fn checked_log10<T: Power10>(x: T) -> Option<u32> {
    x.checked_log10()
}

/// Returns a power of ten greater than or equal to the supplied value.
/// If the next power of ten is larger than `max_value()`, 0 is returned.
#[inline]
pub fn wrapping_next_power_of_ten<T: Power10>(x: T) -> T {
    x.wrapping_next_power_of_ten()
}

/// Returns a power of ten greater than or equal to the supplied value.
/// If the next power of ten is larger than `max_value()`, None is returned.
#[inline]
pub fn checked_next_power_of_ten<T: Power10>(x: T) -> Option<T> {
    x.checked_next_power_of_ten()
}

/// Returns a power of ten greater than or equal to the supplied value.
/// If the next power of ten is larger than `max_value()`, 0 is returned in release more
/// and panics in debug mode.
#[inline]
pub fn next_power_of_ten<T: Power10>(x: T) -> T {
    x.next_power_of_ten()
}

// Implementation note: the is_power_of_ten algorithm for u16/u32 is based on a
// perfect hash setup with very simple hash functions. These hash functions only use 32-bit
// operations for portable-speed.
// This approach is slightly better than leading_zeros() (which is used for u64 and fast logarithms)
static POWER10_HASH_U16: [u32; 8] = [1, 10, 10000, 0, 100, 1000, 0, 0];
static POWER10_HASH_U32: [u32; 16] = [
    10000, 1, 10000000, 0, 100, 0, 100000, 100000000, 1000, 0, 10, 1000000000, 0, 1000000, 0, 0,
];
static POWER10_LZ_U64: [u64; 65] = [
    10000000000000000000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000000000000000000,
    -1i64 as u64,
    -1i64 as u64,
    100000000000000000,
    -1i64 as u64,
    -1i64 as u64,
    10000000000000000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000000000000000,
    -1i64 as u64,
    -1i64 as u64,
    100000000000000,
    -1i64 as u64,
    -1i64 as u64,
    10000000000000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000000000000,
    -1i64 as u64,
    -1i64 as u64,
    100000000000,
    -1i64 as u64,
    -1i64 as u64,
    10000000000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000000000,
    -1i64 as u64,
    -1i64 as u64,
    100000000,
    -1i64 as u64,
    -1i64 as u64,
    10000000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000000,
    -1i64 as u64,
    -1i64 as u64,
    100000,
    -1i64 as u64,
    -1i64 as u64,
    10000,
    -1i64 as u64,
    -1i64 as u64,
    -1i64 as u64,
    1000,
    -1i64 as u64,
    -1i64 as u64,
    100,
    -1i64 as u64,
    -1i64 as u64,
    10,
    -1i64 as u64,
    -1i64 as u64,
    1,
    1,
];

// implementation note: reverse search is a bit faster than hash lookup for u8
#[inline]
fn is_pow10_u8(v: u8) -> bool {
    if v >= 100 {
        return v == 100;
    }
    if v >= 10 {
        return v == 10;
    }
    v == 1
}

// implementation note: at least on x86-64, 32bit ops are far faster than 16 bit ones, even ==
#[inline]
fn is_pow10_u16(v: u16) -> bool {
    v as u32 == POWER10_HASH_U16[((v as u32 >> 3) & 7) as usize]
}

#[inline]
fn is_pow10_u32(v: u32) -> bool {
    let hash = v ^ (v >> 14);
    v == POWER10_HASH_U32[(hash & 15) as usize]
}

#[inline]
fn is_pow10_u64(v: u64) -> bool {
    v == POWER10_LZ_U64[(v.leading_zeros() & 63) as usize] // & 63 may look redundant, but it prevents a range check
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn is_pow10_usize(v: usize) -> bool {
    is_pow10_u64(v as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn is_pow10_usize(v: usize) -> bool {
    is_pow10_u32(v as u32)
}

static POWER10_DIGITS_U32: [(u32, u32); 34] = [
    (8, 1_000_000_000),
    (8, 1_000_000_000),
    (8, 1_000_000_000),
    (8, 1000000000),
    (8, 1000000000),
    (7, 100000000),
    (7, 100000000),
    (7, 100000000),
    (6, 10000000),
    (6, 10000000),
    (6, 10000000),
    (6, 10000000),
    (5, 1000000),
    (5, 1000000),
    (5, 1000000),
    (4, 100000),
    (4, 100000),
    (4, 100000),
    (3, 10000),
    (3, 10000),
    (3, 10000),
    (3, 10000),
    (2, 1000),
    (2, 1000),
    (2, 1000),
    (1, 100),
    (1, 100),
    (1, 100),
    (0, 10),
    (0, 10),
    (0, 10),
    (0, 10),
    (0, 1),
    (0, 1),
];

static POWER10_DIGITS_U64: [(u32, u64); 66] = [
    (18, 10000000000000000000),
    (18, 10000000000000000000),
    (18, 10000000000000000000),
    (18, 10000000000000000000),
    (17, 1000000000000000000),
    (17, 1000000000000000000),
    (17, 1000000000000000000),
    (16, 100000000000000000),
    (16, 100000000000000000),
    (16, 100000000000000000),
    (15, 10000000000000000),
    (15, 10000000000000000),
    (15, 10000000000000000),
    (15, 10000000000000000),
    (14, 1000000000000000),
    (14, 1000000000000000),
    (14, 1000000000000000),
    (13, 100000000000000),
    (13, 100000000000000),
    (13, 100000000000000),
    (12, 10000000000000),
    (12, 10000000000000),
    (12, 10000000000000),
    (12, 10000000000000),
    (11, 1000000000000),
    (11, 1000000000000),
    (11, 1000000000000),
    (10, 100000000000),
    (10, 100000000000),
    (10, 100000000000),
    (9, 10000000000),
    (9, 10000000000),
    (9, 10000000000),
    (9, 10000000000),
    (8, 1000000000),
    (8, 1000000000),
    (8, 1000000000),
    (7, 100000000),
    (7, 100000000),
    (7, 100000000),
    (6, 10000000),
    (6, 10000000),
    (6, 10000000),
    (6, 10000000),
    (5, 1000000),
    (5, 1000000),
    (5, 1000000),
    (4, 100000),
    (4, 100000),
    (4, 100000),
    (3, 10000),
    (3, 10000),
    (3, 10000),
    (3, 10000),
    (2, 1000),
    (2, 1000),
    (2, 1000),
    (1, 100),
    (1, 100),
    (1, 100),
    (0, 10),
    (0, 10),
    (0, 10),
    (0, 10),
    (0, 1),
    (0, 1),
];

#[inline]
fn log10_u8(v: u8) -> u32 {
    if v >= 100 {
        return 2;
    }
    if v >= 10 {
        return 1;
    }
    0
}

#[inline]
fn log10_u16(v: u16) -> u32 {
    log10_u32(v as u32)
}

#[inline]
fn log10_u32(v: u32) -> u32 {
    let lz = v.leading_zeros();
    let (digits, pow10) = POWER10_DIGITS_U32[lz as usize];
    digits + ((v >= pow10) as u32)
}

#[inline]
fn log10_u64(v: u64) -> u32 {
    let lz = v.leading_zeros();
    let (digits, pow10) = POWER10_DIGITS_U64[lz as usize];
    digits + ((v >= pow10) as u32)
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn log10_usize(v: usize) -> u32 {
    log10_u64(v as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn log10_usize(v: usize) -> u32 {
    log10_u32(v as u32)
}

#[inline]
fn wrapping_next_power_of_ten_u8(v: u8) -> u8 {
    if v > 100 {
        return 0;
    }
    if v > 10 {
        return 100;
    }
    if v > 1 {
        return 10;
    }
    1
}

#[inline]
fn wrapping_next_power_of_ten_u16(v: u16) -> u16 {
    if v > 10000 {
        return 0;
    }
    wrapping_next_power_of_ten_u32(v as u32) as u16
}

#[inline]
fn wrapping_next_power_of_ten_u32(v: u32) -> u32 {
    if v > POWER10_DIGITS_U32[0].1 {
        return 0;
    }
    let lz = v.leading_zeros();
    let (_, prev_pow10) = POWER10_DIGITS_U32[(lz + 1) as usize];
    if v == prev_pow10 {
        prev_pow10
    } else if v > prev_pow10 {
        POWER10_DIGITS_U32[(lz - 1) as usize].1
    } else {
        POWER10_DIGITS_U32[lz as usize].1
    }
}

#[inline]
fn wrapping_next_power_of_ten_u64(v: u64) -> u64 {
    let lz = v.leading_zeros();
    if lz == 0 {
        let max = POWER10_DIGITS_U64[0].1;
        if v > max {
            return 0;
        }
        return max;
    }
    let (_, prev_pow10) = POWER10_DIGITS_U64[(lz + 1) as usize];
    if v == prev_pow10 {
        prev_pow10
    } else if v > prev_pow10 {
        POWER10_DIGITS_U64[(lz - 1) as usize].1
    } else {
        POWER10_DIGITS_U64[lz as usize].1
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn wrapping_next_power_of_ten_usize(v: usize) -> usize {
    wrapping_next_power_of_ten_u64(v as u64) as usize
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn wrapping_next_power_of_ten_usize(v: usize) -> usize {
    wrapping_next_power_of_ten_u32(v as u32) as usize
}

macro_rules! hide_u128 {
    ($T:ty) => {
        static POWER10_HASH_U128: [$T; 64] = [
            100000000000000000000000000000000000,
            0,
            100000000000000000000000000000000,
            0,
            1000,
            0,
            0,
            0,
            0,
            1000000000000000000000000000000000000,
            1000000000,
            0,
            100000000000000,
            100,
            0,
            0,
            0,
            100000,
            0,
            0,
            10000000000000,
            100000000000,
            10000000000000000000,
            0,
            0,
            10000000000000000000000000000000000,
            100000000,
            0,
            1000000000000000000000000000000000,
            1000000000000,
            0,
            100000000000000000000000000000000000000,
            10000000000000000,
            100000000000000000000000000,
            0,
            10000000000000000000000000000000000000,
            1000000000000000000,
            1,
            10000000000000000000000000,
            1000000000000000000000000,
            100000000000000000000000000000,
            10000000,
            10000000000000000000000000000,
            0,
            1000000000000000000000000000,
            100000000000000000,
            10000,
            0,
            1000000,
            1000000000000000000000000000000,
            0,
            100000000000000000000,
            10,
            0,
            10000000000,
            10000000000000000000000,
            0,
            0,
            10000000000000000000000000000000,
            1000000000000000000000,
            0,
            100000000000000000000000,
            1000000000000000,
            0,
        ];
        
        #[inline]
        pub fn is_pow10_u128(v: $T) -> bool {
            let mut hash: u32 = v as u32 | (((v as u64) >> 32) as u32);
            hash = hash.wrapping_mul(1249991743) >> 25;
            v == POWER10_HASH_U128[(hash & 63) as usize]
        }
        
        static POWER10_DIGITS_U128: [(u32, u128); 130] = [
            (37, 100000000000000000000000000000000000000),
            (37, 100000000000000000000000000000000000000),
            (37, 100000000000000000000000000000000000000),
            (37, 100000000000000000000000000000000000000),
            (37, 100000000000000000000000000000000000000),
            (36, 10000000000000000000000000000000000000),
            (36, 10000000000000000000000000000000000000),
            (36, 10000000000000000000000000000000000000),
            (35, 1000000000000000000000000000000000000),
            (35, 1000000000000000000000000000000000000),
            (35, 1000000000000000000000000000000000000),
            (34, 100000000000000000000000000000000000),
            (34, 100000000000000000000000000000000000),
            (34, 100000000000000000000000000000000000),
            (34, 100000000000000000000000000000000000),
            (33, 10000000000000000000000000000000000),
            (33, 10000000000000000000000000000000000),
            (33, 10000000000000000000000000000000000),
            (32, 1000000000000000000000000000000000),
            (32, 1000000000000000000000000000000000),
            (32, 1000000000000000000000000000000000),
            (31, 100000000000000000000000000000000),
            (31, 100000000000000000000000000000000),
            (31, 100000000000000000000000000000000),
            (31, 100000000000000000000000000000000),
            (30, 10000000000000000000000000000000),
            (30, 10000000000000000000000000000000),
            (30, 10000000000000000000000000000000),
            (29, 1000000000000000000000000000000),
            (29, 1000000000000000000000000000000),
            (29, 1000000000000000000000000000000),
            (28, 100000000000000000000000000000),
            (28, 100000000000000000000000000000),
            (28, 100000000000000000000000000000),
            (27, 10000000000000000000000000000),
            (27, 10000000000000000000000000000),
            (27, 10000000000000000000000000000),
            (27, 10000000000000000000000000000),
            (26, 1000000000000000000000000000),
            (26, 1000000000000000000000000000),
            (26, 1000000000000000000000000000),
            (25, 100000000000000000000000000),
            (25, 100000000000000000000000000),
            (25, 100000000000000000000000000),
            (24, 10000000000000000000000000),
            (24, 10000000000000000000000000),
            (24, 10000000000000000000000000),
            (24, 10000000000000000000000000),
            (23, 1000000000000000000000000),
            (23, 1000000000000000000000000),
            (23, 1000000000000000000000000),
            (22, 100000000000000000000000),
            (22, 100000000000000000000000),
            (22, 100000000000000000000000),
            (21, 10000000000000000000000),
            (21, 10000000000000000000000),
            (21, 10000000000000000000000),
            (21, 10000000000000000000000),
            (20, 1000000000000000000000),
            (20, 1000000000000000000000),
            (20, 1000000000000000000000),
            (19, 100000000000000000000),
            (19, 100000000000000000000),
            (19, 100000000000000000000),
            (18, 10000000000000000000),
            (18, 10000000000000000000),
            (18, 10000000000000000000),
            (18, 10000000000000000000),
            (17, 1000000000000000000),
            (17, 1000000000000000000),
            (17, 1000000000000000000),
            (16, 100000000000000000),
            (16, 100000000000000000),
            (16, 100000000000000000),
            (15, 10000000000000000),
            (15, 10000000000000000),
            (15, 10000000000000000),
            (15, 10000000000000000),
            (14, 1000000000000000),
            (14, 1000000000000000),
            (14, 1000000000000000),
            (13, 100000000000000),
            (13, 100000000000000),
            (13, 100000000000000),
            (12, 10000000000000),
            (12, 10000000000000),
            (12, 10000000000000),
            (12, 10000000000000),
            (11, 1000000000000),
            (11, 1000000000000),
            (11, 1000000000000),
            (10, 100000000000),
            (10, 100000000000),
            (10, 100000000000),
            (9, 10000000000),
            (9, 10000000000),
            (9, 10000000000),
            (9, 10000000000),
            (8, 1000000000),
            (8, 1000000000),
            (8, 1000000000),
            (7, 100000000),
            (7, 100000000),
            (7, 100000000),
            (6, 10000000),
            (6, 10000000),
            (6, 10000000),
            (6, 10000000),
            (5, 1000000),
            (5, 1000000),
            (5, 1000000),
            (4, 100000),
            (4, 100000),
            (4, 100000),
            (3, 10000),
            (3, 10000),
            (3, 10000),
            (3, 10000),
            (2, 1000),
            (2, 1000),
            (2, 1000),
            (1, 100),
            (1, 100),
            (1, 100),
            (0, 10),
            (0, 10),
            (0, 10),
            (0, 10),
            (0, 1),
            (0, 1),
        ];
        
        #[inline]
        fn log10_u128(v: $T) -> u32 {
            let lz = v.leading_zeros();
            let (digits, pow10) = POWER10_DIGITS_U128[lz as usize];
            digits + ((v >= pow10) as u32)
        }
        
        #[inline]
        fn wrapping_next_power_of_ten_u128(v: u128) -> u128 {
            if v > POWER10_DIGITS_U128[0].1 {
                return 0;
            }
            let lz = v.leading_zeros();
            let (_, prev_pow10) = POWER10_DIGITS_U128[(lz + 1) as usize];
            if v == prev_pow10 {
                prev_pow10
            } else if v > prev_pow10 {
                POWER10_DIGITS_U128[(lz - 1) as usize].1
            } else {
                POWER10_DIGITS_U128[lz as usize].1
            }
        }
    };
}

#[cfg(has_i128)]
hide_u128!(u128);

macro_rules! unsigned_power10 {
    ($T:ty, $pow_fn: ident, $log_fn: ident, $wrap_next_fn: ident) => {
        impl Power10 for $T {
            #[inline]
            fn is_power_of_ten(&self) -> bool {
                $pow_fn(*self)
            }
        
            #[inline]
            fn checked_log10(&self) -> Option<u32> {
                if 0 == *self {
                    return None;
                }
                Some($log_fn(*self))
            }

            #[inline]
            fn log10(&self) -> u32 {
                if 0 == *self {
                    panic!("undefined value for log of zero");
                }
                $log_fn(*self)
            }

            #[inline]
            fn wrapping_next_power_of_ten(&self) -> $T {
                $wrap_next_fn(*self)
            }
        
            #[inline]
            fn checked_next_power_of_ten(&self) -> Option<$T> {
                let x = self.wrapping_next_power_of_ten();
                if x == 0 {
                    return None;
                }
                Some(x)
            }
        
            #[cfg(debug_assertions)]
            #[inline]
            fn next_power_of_ten(&self) -> $T {
                let x = self.wrapping_next_power_of_ten();
                if x == 0 {
                    panic!("overflow in next_power_of_ten for {}", *self);
                }
                x
            }
        
            #[cfg(not(debug_assertions))]
            #[inline]
            fn next_power_of_ten(&self) -> $T {
                self.wrapping_next_power_of_ten()
            }
        }
    };
}

unsigned_power10!(
    u8,
    is_pow10_u8,
    log10_u8,
    wrapping_next_power_of_ten_u8
); //https://github.com/rust-lang/rust/issues/29599
unsigned_power10!(
    u16,
    is_pow10_u16,
    log10_u16,
    wrapping_next_power_of_ten_u16
);
unsigned_power10!(
    u32,
    is_pow10_u32,
    log10_u32,
    wrapping_next_power_of_ten_u32
);
unsigned_power10!(
    u64,
    is_pow10_u64,
    log10_u64,
    wrapping_next_power_of_ten_u64
);
#[cfg(has_i128)]
unsigned_power10!(
    u128,
    is_pow10_u128,
    log10_u128,
    wrapping_next_power_of_ten_u128
);
unsigned_power10!(
    usize,
    is_pow10_usize,
    log10_usize,
    wrapping_next_power_of_ten_usize
);
