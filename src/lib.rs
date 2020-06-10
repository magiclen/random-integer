/*!
# Random Integer
Generate a random integer between two integer numbers (including the two integer numbers).

## Example

```rust
extern crate random_integer;

let rnd = random_integer::random_u8(224, 255);

println!("{}", rnd);
```
*/

#![no_std]

extern crate core;
pub extern crate rand;

use core::cmp::Ordering;

use rand::distributions::uniform::{SampleBorrow, SampleUniform, Uniform};
use rand::distributions::Distribution;
use rand::Rng;

#[inline]
fn random<N>(a: N, b: N) -> N
where
    N: SampleUniform + SampleBorrow<N> + Ord + Sized, {
    let mut rng = rand::thread_rng();

    random_with_rng(a, b, &mut rng)
}

#[inline]
fn random_with_rng<N, T: Rng>(a: N, b: N, rng: &mut T) -> N
where
    N: SampleUniform + SampleBorrow<N> + Ord + Sized, {
    match a.cmp(&b) {
        Ordering::Greater => {
            let simpler = Uniform::new_inclusive(b, a);

            simpler.sample(rng)
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let simpler = Uniform::new_inclusive(a, b);

            simpler.sample(rng)
        }
    }
}

#[inline]
pub fn random_usize(a: usize, b: usize) -> usize {
    random(a, b)
}

#[inline]
pub fn random_usize_with_rng<T: Rng>(a: usize, b: usize, rng: &mut T) -> usize {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_u64(a: u64, b: u64) -> u64 {
    random(a, b)
}

#[inline]
pub fn random_u64_with_rng<T: Rng>(a: u64, b: u64, rng: &mut T) -> u64 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_u32(a: u32, b: u32) -> u32 {
    random(a, b)
}

#[inline]
pub fn random_u32_with_rng<T: Rng>(a: u32, b: u32, rng: &mut T) -> u32 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_u16(a: u16, b: u16) -> u16 {
    random(a, b)
}

#[inline]
pub fn random_u16_with_rng<T: Rng>(a: u16, b: u16, rng: &mut T) -> u16 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_u8(a: u8, b: u8) -> u8 {
    random(a, b)
}

#[inline]
pub fn random_u8_with_rng<T: Rng>(a: u8, b: u8, rng: &mut T) -> u8 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_isize(a: isize, b: isize) -> isize {
    random(a, b)
}

#[inline]
pub fn random_isize_with_rng<T: Rng>(a: isize, b: isize, rng: &mut T) -> isize {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_i64(a: i64, b: i64) -> i64 {
    random(a, b)
}

#[inline]
pub fn random_i64_with_rng<T: Rng>(a: i64, b: i64, rng: &mut T) -> i64 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_i32(a: i32, b: i32) -> i32 {
    random(a, b)
}

#[inline]
pub fn random_i32_with_rng<T: Rng>(a: i32, b: i32, rng: &mut T) -> i32 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_i16(a: i16, b: i16) -> i16 {
    random(a, b)
}

#[inline]
pub fn random_i16_with_rng<T: Rng>(a: i16, b: i16, rng: &mut T) -> i16 {
    random_with_rng(a, b, rng)
}

#[inline]
pub fn random_i8(a: i8, b: i8) -> i8 {
    random(a, b)
}

#[inline]
pub fn random_i8_with_rng<T: Rng>(a: i8, b: i8, rng: &mut T) -> i8 {
    random_with_rng(a, b, rng)
}
