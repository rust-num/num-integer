use traits::{Zero, checked_pow};

use Integer;

pub trait Power: Integer {
    /// Returns whether `self` is a power of `other`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.is_power_of(&10), true);
    /// assert_eq!(4u32.is_power_of(&10), false);
    /// ~~~
    #[inline]
    fn is_power_of(&self, other: &Self) -> bool {
        self.checked_next_power_of(other).map_or(false, |new| self == &new)
    }

    /// Returns the truncated base-`other` logarithm of `self`.
    /// Panics if `self.is_zero()`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.log(&10), 2);
    /// assert_eq!(4u32.log(&10), 0);
    /// ~~~
    #[inline]
    fn log(&self, other: &Self) -> u32 {
        self.checked_log(other).expect("log: `self` is zero")
    }

    /// Returns the truncated base-`other` logarithm of `self`, or `None` if `self.is_zero()`.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.checked_log(&10), Some(2));
    /// assert_eq!(4u32.checked_log(&10), Some(0));
    /// assert_eq!(0u32.checked_log(&10), None);
    /// ~~~
    fn checked_log(&self, other: &Self) -> Option<u32>;

    /// Returns the least power of `other` not less than `self`, or `0` if the value is out of
    /// bounds of the type.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.wrapping_next_power_of(&10), 100);
    /// assert_eq!(4u32.wrapping_next_power_of(&10), 10);
    /// assert_eq!(200u8.wrapping_next_power_of(&10), 0);
    /// ~~~
    #[inline]
    fn wrapping_next_power_of(&self, other: &Self) -> Self {
        self.checked_next_power_of(other).unwrap_or(Self::zero())
    }

    /// Returns the least power of `other` not less than `self`, or `None` if the value is out
    /// of bounds of the type.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.checked_next_power_of(&10), Some(100));
    /// assert_eq!(4u32.checked_next_power_of(&10), Some(10));
    /// assert_eq!(200u8.checked_next_power_of(&10), None);
    /// ~~~
    fn checked_next_power_of(&self, other: &Self) -> Option<Self>;

    /// Returns the least power of `other` not less than `self`.
    /// Panics if the value is out of bounds of the type.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power;
    /// assert_eq!(100u32.next_power_of(&10), 100);
    /// assert_eq!(4u32.next_power_of(&10), 10);
    /// ~~~
    #[inline]
    fn next_power_of(&self, other: &Self) -> Self {
        self.checked_next_power_of(other).expect("checked_next_power_of: out of range")
    }
}

macro_rules! impl_Power {
    ($t:ty) => (impl Power for $t {
        #[inline]
        fn checked_log(&self, other: &Self) -> Option<u32> {
            let mut n = self.clone();
            let mut a = None;
            while !n.is_zero() {
                a = Some(a.map_or(0, |a| a+1));
                n /= *other;
            }
            a
        }

        #[inline]
        fn checked_next_power_of(&self, other: &Self) -> Option<Self> {
            self.checked_sub(1)
                .map_or(Some(1), |new|
                        checked_pow(*other, new.checked_log(other)
                                        .map_or(0, |a| a as Self + 1) as usize))
        }
    });

    ($($t:ty),*) => { $(impl_Power!($t);)* };
}

impl_Power!(usize, u8, u16, u32, u64);
#[cfg(has_i128)]
impl_Power!(u128);
