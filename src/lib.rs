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
extern crate rand;

use core::cmp::Ordering;

use rand::Rng;

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn random_usize(a: usize, b: usize) -> usize {
    let mut rng = rand::thread_rng();

    random_usize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "64")]
pub fn random_usize_with_rng<T: Rng>(a: usize, b: usize, rng: &mut T) -> usize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let rnd: u64 = rng.gen();

            let rnd = u128::from(rnd);
            let a = a as u128;
            let b = b as u128;

            ((rnd % (a - b + 1)) + b) as usize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let rnd: u64 = rng.gen();

            let rnd = u128::from(rnd);
            let a = a as u128;
            let b = b as u128;

            ((rnd % (b - a + 1)) + a) as usize
        }
    }
}

#[inline]
pub fn random_u64(a: u64, b: u64) -> u64 {
    let mut rng = rand::thread_rng();

    random_u64_with_rng(a, b, &mut rng)
}

pub fn random_u64_with_rng<T: Rng>(a: u64, b: u64, rng: &mut T) -> u64 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let rnd: u64 = rng.gen();

            let rnd = u128::from(rnd);
            let a = u128::from(a);
            let b = u128::from(b);

            ((rnd % (a - b + 1)) + b) as u64
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let rnd: u64 = rng.gen();

            let rnd = u128::from(rnd);
            let a = u128::from(a);
            let b = u128::from(b);

            ((rnd % (b - a + 1)) + a) as u64
        }
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn random_usize(a: usize, b: usize) -> usize {
    let mut rng = rand::thread_rng();

    random_usize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "32")]
pub fn random_usize_with_rng<T: Rng>(a: usize, b: usize, rng: &mut T) -> usize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u64::from(a);
            let b = u64::from(b);

            rng.gen_range(b, a + 1) as usize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u64::from(a);
            let b = u64::from(b);

            rng.gen_range(a, b + 1) as usize
        }
    }
}

#[inline]
pub fn random_u32(a: u32, b: u32) -> u32 {
    let mut rng = rand::thread_rng();

    random_u32_with_rng(a, b, &mut rng)
}

pub fn random_u32_with_rng<T: Rng>(a: u32, b: u32, rng: &mut T) -> u32 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u64::from(a);
            let b = u64::from(b);

            rng.gen_range(b, a + 1) as u32
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u64::from(a);
            let b = u64::from(b);

            rng.gen_range(a, b + 1) as u32
        }
    }
}

#[cfg(target_pointer_width = "16")]
#[inline]
pub fn random_usize(a: usize, b: usize) -> usize {
    let mut rng = rand::thread_rng();

    random_usize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "16")]
pub fn random_usize_with_rng<T: Rng>(a: usize, b: usize, rng: &mut T) -> usize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u32::from(a);
            let b = u32::from(b);

            rng.gen_range(b, a + 1) as usize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u32::from(a);
            let b = u32::from(b);

            rng.gen_range(a, b + 1) as usize
        }
    }
}

#[inline]
pub fn random_u16(a: u16, b: u16) -> u16 {
    let mut rng = rand::thread_rng();

    random_u16_with_rng(a, b, &mut rng)
}

pub fn random_u16_with_rng<T: Rng>(a: u16, b: u16, rng: &mut T) -> u16 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u32::from(a);
            let b = u32::from(b);

            rng.gen_range(b, a + 1) as u16
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u32::from(a);
            let b = u32::from(b);

            rng.gen_range(a, b + 1) as u16
        }
    }
}

#[cfg(target_pointer_width = "8")]
#[inline]
pub fn random_usize(a: usize, b: usize) -> usize {
    let mut rng = rand::thread_rng();

    random_usize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "8")]
pub fn random_usize_with_rng<T: Rng>(a: usize, b: usize, rng: &mut T) -> usize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u16::from(a);
            let b = u16::from(b);

            rng.gen_range(b, a + 1) as usize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u16::from(a);
            let b = u16::from(b);

            rng.gen_range(a, b + 1) as usize
        }
    }
}

#[inline]
pub fn random_u8(a: u8, b: u8) -> u8 {
    let mut rng = rand::thread_rng();

    random_u8_with_rng(a, b, &mut rng)
}

pub fn random_u8_with_rng<T: Rng>(a: u8, b: u8, rng: &mut T) -> u8 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = u16::from(a);
            let b = u16::from(b);

            rng.gen_range(b, a + 1) as u8
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = u16::from(a);
            let b = u16::from(b);

            rng.gen_range(a, b + 1) as u8
        }
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn random_isize(a: isize, b: isize) -> isize {
    let mut rng = rand::thread_rng();

    random_isize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "64")]
pub fn random_isize_with_rng<T: Rng>(a: isize, b: isize, rng: &mut T) -> isize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let rnd: u64 = rng.gen();

            let rnd = i128::from(rnd);
            let a = a as i128;
            let b = b as i128;

            ((rnd % (a - b + 1)) + b) as isize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let rnd: u64 = rng.gen();

            let rnd = i128::from(rnd);
            let a = a as i128;
            let b = b as i128;

            ((rnd % (b - a + 1)) + a) as isize
        }
    }
}

#[inline]
pub fn random_i64(a: i64, b: i64) -> i64 {
    let mut rng = rand::thread_rng();

    random_i64_with_rng(a, b, &mut rng)
}

pub fn random_i64_with_rng<T: Rng>(a: i64, b: i64, rng: &mut T) -> i64 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let rnd: u64 = rng.gen();

            let rnd = i128::from(rnd);
            let a = i128::from(a);
            let b = i128::from(b);

            ((rnd % (a - b + 1)) + b) as i64
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let rnd: u64 = rng.gen();

            let rnd = i128::from(rnd);
            let a = i128::from(a);
            let b = i128::from(b);

            ((rnd % (b - a + 1)) + a) as i64
        }
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn random_isize(a: isize, b: isize) -> isize {
    let mut rng = rand::thread_rng();

    random_isize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "32")]
pub fn random_isize_with_rng<T: Rng>(a: isize, b: isize, rng: &mut T) -> isize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i64::from(a);
            let b = i64::from(b);

            rng.gen_range(b, a + 1) as isize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i64::from(a);
            let b = i64::from(b);

            rng.gen_range(a, b + 1) as isize
        }
    }
}

#[inline]
pub fn random_i32(a: i32, b: i32) -> i32 {
    let mut rng = rand::thread_rng();

    random_i32_with_rng(a, b, &mut rng)
}

pub fn random_i32_with_rng<T: Rng>(a: i32, b: i32, rng: &mut T) -> i32 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i64::from(a);
            let b = i64::from(b);

            rng.gen_range(b, a + 1) as i32
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i64::from(a);
            let b = i64::from(b);

            rng.gen_range(a, b + 1) as i32
        }
    }
}

#[cfg(target_pointer_width = "16")]
#[inline]
pub fn random_isize(a: isize, b: isize) -> isize {
    let mut rng = rand::thread_rng();

    random_isize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "16")]
pub fn random_isize_with_rng<T: Rng>(a: isize, b: isize, rng: &mut T) -> isize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i32::from(a);
            let b = i32::from(b);

            rng.gen_range(b, a + 1) as isize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i32::from(a);
            let b = i32::from(b);

            rng.gen_range(a, b + 1) as isize
        }
    }
}

#[inline]
pub fn random_i16(a: i16, b: i16) -> i16 {
    let mut rng = rand::thread_rng();

    random_i16_with_rng(a, b, &mut rng)
}

pub fn random_i16_with_rng<T: Rng>(a: i16, b: i16, rng: &mut T) -> i16 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i32::from(a);
            let b = i32::from(b);

            rng.gen_range(b, a + 1) as i16
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i32::from(a);
            let b = i32::from(b);

            rng.gen_range(a, b + 1) as i16
        }
    }
}

#[cfg(target_pointer_width = "8")]
#[inline]
pub fn random_isize(a: isize, b: isize) -> isize {
    let mut rng = rand::thread_rng();

    random_isize_with_rng(a, b, &mut rng)
}

#[cfg(target_pointer_width = "8")]
pub fn random_isize_with_rng<T: Rng>(a: isize, b: isize, rng: &mut T) -> isize {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i16::from(a);
            let b = i16::from(b);

            rng.gen_range(b, a + 1) as isize
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i16::from(a);
            let b = i16::from(b);

            rng.gen_range(a, b + 1) as isize
        }
    }
}

#[inline]
pub fn random_i8(a: i8, b: i8) -> i8 {
    let mut rng = rand::thread_rng();

    random_i8_with_rng(a, b, &mut rng)
}

pub fn random_i8_with_rng<T: Rng>(a: i8, b: i8, rng: &mut T) -> i8 {
    match a.cmp(&b) {
        Ordering::Greater => {
            let a = i16::from(a);
            let b = i16::from(b);

            rng.gen_range(b, a + 1) as i8
        }
        Ordering::Equal => a,
        Ordering::Less => {
            let a = i16::from(a);
            let b = i16::from(b);

            rng.gen_range(a, b + 1) as i8
        }
    }
}
