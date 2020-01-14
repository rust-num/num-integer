//! Benchmark sqrt and cbrt

#![feature(test)]

extern crate num_integer;
extern crate num_traits;
extern crate test;

use num_integer::Integer;
use num_traits::{AsPrimitive, PrimInt, WrappingAdd, WrappingMul};
use test::{black_box, Bencher};

trait BenchInteger: Integer + PrimInt + WrappingAdd + WrappingMul + 'static {}

impl<T> BenchInteger for T where T: Integer + PrimInt + WrappingAdd + WrappingMul + 'static {}

trait NaiveAverage {
    fn naive_average_floor(&self, other: &Self) -> Self;
    fn naive_average_ceil(&self, other: &Self) -> Self;
}

trait UncheckedAverage {
    fn unchecked_average_floor(&self, other: &Self) -> Self;
    fn unchecked_average_ceil(&self, other: &Self) -> Self;
}

fn bench<T, F>(b: &mut Bencher, v: &[(T, T)], f: F)
where
    T: Integer,
    F: Fn(&T, &T) -> T,
{
    b.iter(|| {
        for (x, y) in v {
            black_box(f(x, y));
        }
    });
}

// Simple PRNG so we don't have to worry about rand compatibility
fn lcg<T>(x: T) -> T
where
    u32: AsPrimitive<T>,
    T: BenchInteger,
{
    // LCG parameters from Numerical Recipes
    // (but we're applying it to arbitrary sizes)
    const LCG_A: u32 = 1664525;
    const LCG_C: u32 = 1013904223;
    x.wrapping_mul(&LCG_A.as_()).wrapping_add(&LCG_C.as_())
}

macro_rules! naive_average {
    ($T:ident) => {
        impl super::NaiveAverage for $T {
            fn naive_average_floor(&self, other: &$T) -> $T {
                match self.checked_add(*other) {
                    Some(z) => z / 2,
                    None => {
                        if self > other {
                            let diff = self - other;
                            other + diff / 2
                        } else {
                            let diff = other - self;
                            self + diff / 2
                        }
                    }
                }
            }
            fn naive_average_ceil(&self, other: &$T) -> $T {
                match self.checked_add(*other).and_then(|x| x.checked_add(1)) {
                    Some(z) => z / 2,
                    None => {
                        if self > other {
                            let diff = self - other;
                            self - diff / 2
                        } else {
                            let diff = other - self;
                            other - diff / 2
                        }
                    }
                }
            }
        }
    };
}

macro_rules! unchecked_average {
    ($T:ident) => {
        impl super::UncheckedAverage for $T {
            fn unchecked_average_floor(&self, other: &$T) -> $T {
                self.wrapping_add(*other) / 2
            }
            fn unchecked_average_ceil(&self, other: &$T) -> $T {
                (self.wrapping_add(*other) / 2).wrapping_add(1)
            }
        }
    };
}

macro_rules! bench_average {
    ($($T:ident),*) => {$(
        mod $T {
            use test::Bencher;
            use num_integer::Average;
            use UncheckedAverage;
            use NaiveAverage;

            naive_average!($T);
            unchecked_average!($T);

            const SIZE: $T = 30;

            fn overflowing() -> Vec<($T, $T)> {
                (($T::max_value()-SIZE)..$T::max_value())
                    .flat_map(|x| -> Vec<_> {
                        (($T::max_value()-100)..($T::max_value()-100+SIZE))
                            .map(|y| (x, y))
                            .collect()
                    })
                    .collect()
            }

            fn small() -> Vec<($T, $T)> {
                (0..SIZE)
                   .flat_map(|x| -> Vec<_> {(0..SIZE).map(|y| (x, y)).collect()})
                   .collect()
            }

            fn rand() -> Vec<($T, $T)> {
                small()
                    .into_iter()
                    .map(|(x, y)| (super::lcg(x), super::lcg(y)))
                    .collect()
            }

            #[bench]
            fn average_floor_small(b: &mut Bencher) {
                let v = small();
                super::bench(b, &v, |x: &$T, y: &$T| x.average_floor(y));
            }

            #[bench]
            fn average_floor_small_naive(b: &mut Bencher) {
                let v = small();
                super::bench(b, &v, |x: &$T, y: &$T| x.naive_average_floor(y));
            }

            #[bench]
            fn average_floor_small_unchecked(b: &mut Bencher) {
                let v = small();
                super::bench(b, &v, |x: &$T, y: &$T| x.unchecked_average_floor(y));
            }

            #[bench]
            fn average_floor_overflowing(b: &mut Bencher) {
                let v = overflowing();
                super::bench(b, &v, |x: &$T, y: &$T| x.average_floor(y));
            }

            #[bench]
            fn average_floor_overflowing_naive(b: &mut Bencher) {
                let v = overflowing();
                super::bench(b, &v, |x: &$T, y: &$T| x.naive_average_floor(y));
            }

            #[bench]
            fn average_floor_overflowing_unchecked(b: &mut Bencher) {
                let v = overflowing();
                super::bench(b, &v, |x: &$T, y: &$T| x.unchecked_average_floor(y));
            }

            #[bench]
            fn average_floor_rand(b: &mut Bencher) {
                let v = rand();
                super::bench(b, &v, |x: &$T, y: &$T| x.average_floor(y));
            }

            #[bench]
            fn average_floor_rand_naive(b: &mut Bencher) {
                let v = rand();
                super::bench(b, &v, |x: &$T, y: &$T| x.naive_average_floor(y));
            }

            #[bench]
            fn average_floor_rand_unchecked(b: &mut Bencher) {
                let v = rand();
                super::bench(b, &v, |x: &$T, y: &$T| x.unchecked_average_floor(y));
            }
        }
    )*}
}

bench_average!(i8, i16, i32, i64, i128, isize);
bench_average!(u8, u16, u32, u64, u128, usize);
