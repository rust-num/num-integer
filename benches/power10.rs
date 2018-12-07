#![feature(test)]

extern crate num_integer;
extern crate num_traits;
extern crate test;

use num_integer::log10;
use num_integer::is_power_of_ten;
use num_integer::Power10;
use num_traits::One;
use num_traits::PrimInt;
use num_traits::Zero;
use test::{black_box, Bencher};

#[bench]
fn benchl10_u32_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u32>();
    bench_log10_slice(b, &v, log10);
}

#[bench]
fn benchl10_u32_up_to_10000(b: &mut Bencher) {
    let v = first_10000_vec::<u32>();
    bench_log10_slice(b, &v, log10);
}

#[bench]
fn benchl10_u64_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u64>();
    bench_log10_slice(b, &v, log10);
}

#[bench]
fn benchl10_u64_up_to_10000(b: &mut Bencher) {
    let v = first_10000_vec::<u64>();
    bench_log10_slice(b, &v, log10);
}

#[bench]
fn benchp10_u32_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u32>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u32_up_to_10000(b: &mut Bencher) {
    let v = first_10000_vec::<u32>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u64_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u64>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u64_up_to_10000(b: &mut Bencher) {
    let v = first_10000_vec::<u64>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u64_10000_random(b: &mut Bencher) {
    let v = random_10000_vec_u64();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u64_only_powers_of_ten_simple(b: &mut Bencher) {
    let v = powers_10_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_simple_u64);
}

#[bench]
fn benchp10_u64_up_to_10000_simple(b: &mut Bencher) {
    let v = first_10000_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_simple_u64);
}

#[bench]
fn benchp10_u64_10000_random_simple(b: &mut Bencher) {
    let v = random_10000_vec_u64();
    bench_pow10_slice(b, &v, is_pow_10_simple_u64);
}

#[bench]
fn benchp10_u64_only_powers_of_ten_rev_search(b: &mut Bencher) {
    let v = powers_10_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u64);
}

#[bench]
fn benchp10_u64_up_to_10000_rev_search(b: &mut Bencher) {
    let v = first_10000_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u64);
}

#[bench]
fn benchp10_u64_10000_random_rev_search(b: &mut Bencher) {
    let v = random_10000_vec_u64();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u64);
}

#[bench]
fn benchp10_u64_only_powers_of_ten_lz(b: &mut Bencher) {
    let v = powers_10_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_lz_u64);
}

#[bench]
fn benchp10_u64_up_to_10000_lz(b: &mut Bencher) {
    let v = first_10000_vec::<u64>();
    bench_pow10_slice(b, &v, is_pow_10_lz_u64);
}

#[bench]
fn benchp10_u64_10000_random_lz(b: &mut Bencher) {
    let v = random_10000_vec_u64();
    bench_pow10_slice(b, &v, is_pow_10_lz_u64);
}

static POWER10_LZ_U64: [u64; 64] = [
    10000000000000000000,
    0,
    0,
    0,
    1000000000000000000,
    0,
    0,
    100000000000000000,
    0,
    0,
    10000000000000000,
    0,
    0,
    0,
    1000000000000000,
    0,
    0,
    100000000000000,
    0,
    0,
    10000000000000,
    0,
    0,
    0,
    1000000000000,
    0,
    0,
    100000000000,
    0,
    0,
    10000000000,
    0,
    0,
    0,
    1000000000,
    0,
    0,
    100000000,
    0,
    0,
    10000000,
    0,
    0,
    0,
    1000000,
    0,
    0,
    100000,
    0,
    0,
    10000,
    0,
    0,
    0,
    1000,
    0,
    0,
    100,
    0,
    0,
    10,
    0,
    0,
    1,
];

#[inline]
fn is_pow_10_lz_u64(v: u64) -> bool {
    v == POWER10_LZ_U64[(v.leading_zeros() & 63) as usize]
}

#[inline]
fn is_pow_10_simple_u64(v: u64) -> bool {
    v == 1
        || v == 10
        || v == 100
        || v == 1_000
        || v == 10_000
        || v == 100_000
        || v == 1_000_000
        || v == 10_000_000
        || v == 100_000_000
        || v == 1_000_000_000
        || v == 10_000_000_000
        || v == 100_000_000_000
        || v == 1_000_000_000_000
        || v == 10_000_000_000_000
        || v == 100_000_000_000_000
        || v == 1_000_000_000_000_000
        || v == 10_000_000_000_000_000
        || v == 100_000_000_000_000_000
        || v == 1_000_000_000_000_000_000u64
        || v == 10_000_000_000_000_000_000
}

#[inline]
fn is_pow_10_rev_search_u64(v: u64) -> bool {
    if v >= 10_000_000_000_000_000_000 {
        return v == 10_000_000_000_000_000_000;
    }
    if v >= 1_000_000_000_000_000_000 {
        return v == 1_000_000_000_000_000_000;
    }
    if v >= 100_000_000_000_000_000 {
        return v == 100_000_000_000_000_000;
    }
    if v >= 10_000_000_000_000_000 {
        return v == 10_000_000_000_000_000;
    }
    if v >= 1_000_000_000_000_000 {
        return v == 1_000_000_000_000_000;
    }
    if v >= 100_000_000_000_000 {
        return v == 100_000_000_000_000;
    }
    if v >= 10_000_000_000_000 {
        return v == 10_000_000_000_000;
    }
    if v >= 1_000_000_000_000 {
        return v == 1_000_000_000_000;
    }
    if v >= 100_000_000_000 {
        return v == 100_000_000_000;
    }
    if v >= 10_000_000_000 {
        return v == 10_000_000_000;
    }
    if v >= 1_000_000_000 {
        return v == 1_000_000_000;
    }
    if v >= 100_000_000 {
        return v == 100_000_000;
    }
    if v >= 10_000_000 {
        return v == 10_000_000;
    }
    if v >= 1_000_000 {
        return v == 1_000_000;
    }
    if v >= 100_000 {
        return v == 100_000;
    }
    if v >= 10_000 {
        return v == 10_000;
    }
    if v >= 1_000 {
        return v == 1_000;
    }
    if v >= 100 {
        return v == 100;
    }
    if v >= 10 {
        return v == 10;
    }
    if v >= 1 {
        return v == 1;
    }
    false
}

fn random_10000_vec_u64() -> Vec<u64> {
    let mut v = Vec::new();
    let mut x: u64 = 0;
    let mut rng = SplitMix {
        v: 0xCAFE_1234_FEED_5678,
    };
    while x < 10000 {
        v.push(rng.next_u64());
        x += 1;
    }
    v
}

#[bench]
fn benchp10_u16_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u16>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u16_up_to_10000(b: &mut Bencher) {
    let v = first_10000_vec::<u16>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u16_10000_random(b: &mut Bencher) {
    let v = random_10000_vec_u16();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u16_only_powers_of_ten_simple(b: &mut Bencher) {
    let v = powers_10_vec::<u16>();
    bench_pow10_slice(b, &v, is_pow_10_simple_u16);
}

#[bench]
fn benchp10_u16_up_to_10000_simple(b: &mut Bencher) {
    let v = first_10000_vec::<u16>();
    bench_pow10_slice(b, &v, is_pow_10_simple_u16);
}

#[bench]
fn benchp10_u16_10000_random_simple(b: &mut Bencher) {
    let v = random_10000_vec_u16();
    bench_pow10_slice(b, &v, is_pow_10_simple_u16);
}

#[bench]
fn benchp10_u16_only_powers_of_ten_rev_search(b: &mut Bencher) {
    let v = powers_10_vec::<u16>();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u16);
}

#[bench]
fn benchp10_u16_up_to_10000_rev_search(b: &mut Bencher) {
    let v = first_10000_vec::<u16>();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u16);
}

#[bench]
fn benchp10_u16_10000_random_rev_search(b: &mut Bencher) {
    let v = random_10000_vec_u16();
    bench_pow10_slice(b, &v, is_pow_10_rev_search_u16);
}

#[inline]
fn is_pow_10_simple_u16(v: u16) -> bool {
    v == 1 || v == 10 || v == 100 || v == 1_000 || v == 10_000
}

#[inline]
fn is_pow_10_rev_search_u16(v: u16) -> bool {
    if v >= 10_000 {
        return v == 10_000;
    }
    if v >= 1_000 {
        return v == 1_000;
    }
    if v >= 100 {
        return v == 100;
    }
    if v >= 10 {
        return v == 10;
    }
    if v >= 1 {
        return v == 1;
    }
    false
}

fn random_10000_vec_u16() -> Vec<u16> {
    let mut v = Vec::new();
    let mut x: u16 = 0;
    let mut rng = SplitMix {
        v: 0xCAFE_1234_FEED_5678,
    };
    while x < 10000 {
        v.push(rng.next_u16());
        x += 1;
    }
    v
}

#[bench]
fn benchp10_u8_only_powers_of_ten(b: &mut Bencher) {
    let v = powers_10_vec::<u8>();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u8_10000_random(b: &mut Bencher) {
    let v = random_10000_vec_u8();
    bench_pow10_slice(b, &v, is_power_of_ten);
}

#[bench]
fn benchp10_u8_only_powers_of_ten_simple(b: &mut Bencher) {
    let v = powers_10_vec::<u8>();
    bench_pow10_slice(b, &v, is_pow_10_simple_u8);
}

#[bench]
fn benchp10_u8_10000_random_simple(b: &mut Bencher) {
    let v = random_10000_vec_u8();
    bench_pow10_slice(b, &v, is_pow_10_simple_u8);
}

#[bench]
fn benchp10_u8_only_powers_of_ten_hash(b: &mut Bencher) {
    let v = powers_10_vec::<u8>();
    bench_pow10_slice(b, &v, is_pow_10_hash_u8);
}

#[bench]
fn benchp10_u8_10000_random_hash(b: &mut Bencher) {
    let v = random_10000_vec_u8();
    bench_pow10_slice(b, &v, is_pow_10_hash_u8);
}

#[inline]
fn is_pow_10_simple_u8(v: u8) -> bool {
    v == 1 || v == 10 || v == 100
}

static POWER10_HASH_U8: [u32; 8] = [1, 1, 10, 0, 100, 0, 0, 0];

#[inline]
fn is_pow_10_hash_u8(v: u8) -> bool {
    let u = v as u32;
    u == POWER10_HASH_U8[(u & 7) as usize]
}

fn random_10000_vec_u8() -> Vec<u8> {
    let mut v = Vec::new();
    let mut x: u32 = 0;
    let mut rng = SplitMix {
        v: 0xCAFE_1234_FEED_5678,
    };
    while x < 10000 {
        v.push(rng.next_u8());
        x += 1;
    }
    v
}

fn first_10000_vec<T: PrimInt + One + Zero>() -> Vec<T> {
    let mut v = Vec::with_capacity(10000);
    let mut x: T = <T>::zero();
    let one: T = <T>::one();
    let ten = (one << 3) + (one << 1);
    let tenk = ten * ten * ten * ten;
    while x < tenk {
        v.push(x);
        x = x + one;
    }
    v
}

fn powers_10_vec<T: PrimInt + One>() -> Vec<T> {
    let mut v = Vec::with_capacity(10000);
    let mut count = 0;
    let one = <T>::one();
    let mut x: T = one;
    let ten = (one << 3) + (one << 1);
    let end: T = <T>::max_value() / ten;
    while count < 10000 {
        v.push(x);
        if x > end {
            x = one;
        } else {
            x = x * ten;
        }
        count += 1;
    }
    v
}

#[inline]
fn bench_pow10_slice<T: Power10 + Copy, F>(b: &mut Bencher, slice: &[T], func: F)
where
    F: Fn(T) -> bool,
{
    b.iter(|| {
        let mut sum = 0;
        for i in slice.iter() {
            if func(*i) {
                sum += 1;
            }
        }
        black_box(sum);
    });
}

#[inline]
fn bench_log10_slice<T: Power10 + Copy, F>(b: &mut Bencher, slice: &[T], func: F)
where
    F: Fn(T) -> u32,
{
    b.iter(|| {
        let mut sum = 0;
        for i in slice.iter() {
            sum += func(*i);
        }
        black_box(sum);
    });
}

struct SplitMix {
    v: u64,
}

impl SplitMix {
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        self.next_u64() as u8
    }

    #[inline]
    pub fn next_u16(&mut self) -> u16 {
        self.next_u64() as u16
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut r = self.v;
        r ^= r >> 26;
        r = r.wrapping_mul(8238576523158062045u64);
        r ^= r >> 35;
        r = r.wrapping_mul(-6410243847380211633i64 as u64);
        r ^= r >> 34;
        self.v = r;;
        self.v
    }
}
