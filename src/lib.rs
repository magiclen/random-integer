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

extern crate rand;

use rand::Rng;

#[cfg(target_pointer_width = "64")]
pub fn random_usize(a: usize, b: usize) -> usize {
    if a > b {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = u128::from(rnd);
        let a = a as u128;
        let b = b as u128;

        ((rnd % (a - b + 1)) + b) as usize
    } else if a == b {
        a
    } else {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = u128::from(rnd);
        let a = a as u128;
        let b = b as u128;

        ((rnd % (b - a + 1)) + a) as usize
    }
}

pub fn random_u64(a: u64, b: u64) -> u64 {
    if a > b {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = u128::from(rnd);
        let a = u128::from(a);
        let b = u128::from(b);

        ((rnd % (a - b + 1)) + b) as u64
    } else if a == b {
        a
    } else {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = u128::from(rnd);
        let a = u128::from(a);
        let b = u128::from(b);

        ((rnd % (b - a + 1)) + a) as u64
    }
}

#[cfg(target_pointer_width = "32")]
pub fn random_usize(a: usize, b: usize) -> usize {
    if a > b {
        let a = u64::from(a);
        let b = u64::from(b);

        rand::thread_rng().gen_range(b, a + 1) as usize
    } else if a == b {
        a
    } else {
        let a = u64::from(a);
        let b = u64::from(b);

        rand::thread_rng().gen_range(a, b + 1) as usize
    }
}

pub fn random_u32(a: u32, b: u32) -> u32 {
    if a > b {
        let a = u64::from(a);
        let b = u64::from(b);

        rand::thread_rng().gen_range(b, a + 1) as u32
    } else if a == b {
        a
    } else {
        let a = u64::from(a);
        let b = u64::from(b);

        rand::thread_rng().gen_range(a, b + 1) as u32
    }
}

#[cfg(target_pointer_width = "16")]
pub fn random_usize(a: usize, b: usize) -> usize {
    if a > b {
        let a = u32::from(a);
        let b = u32::from(b);

        rand::thread_rng().gen_range(b, a + 1) as usize
    } else if a == b {
        a
    } else {
        let a = u32::from(a);
        let b = u32::from(b);

        rand::thread_rng().gen_range(a, b + 1) as usize
    }
}

pub fn random_u16(a: u16, b: u16) -> u16 {
    if a > b {
        let a = u32::from(a);
        let b = u32::from(b);

        rand::thread_rng().gen_range(b, a + 1) as u16
    } else if a == b {
        a
    } else {
        let a = u32::from(a);
        let b = u32::from(b);

        rand::thread_rng().gen_range(a, b + 1) as u16
    }
}

#[cfg(target_pointer_width = "8")]
pub fn random_usize(a: usize, b: usize) -> usize {
    if a > b {
        let a = u16::from(a);
        let b = u16::from(b);

        rand::thread_rng().gen_range(b, a + 1) as usize
    } else if a == b {
        a
    } else {
        let a = u16::from(a);
        let b = u16::from(b);

        rand::thread_rng().gen_range(a, b + 1) as usize
    }
}

pub fn random_u8(a: u8, b: u8) -> u8 {
    if a > b {
        let a = u16::from(a);
        let b = u16::from(b);

        rand::thread_rng().gen_range(b, a + 1) as u8
    } else if a == b {
        a
    } else {
        let a = u16::from(a);
        let b = u16::from(b);

        rand::thread_rng().gen_range(a, b + 1) as u8
    }
}

#[cfg(target_pointer_width = "64")]
pub fn random_isize(a: isize, b: isize) -> isize {
    if a > b {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = i128::from(rnd);
        let a = a as i128;
        let b = b as i128;

        ((rnd % (a - b + 1)) + b) as isize
    } else if a == b {
        a
    } else {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = i128::from(rnd);
        let a = a as i128;
        let b = b as i128;

        ((rnd % (b - a + 1)) + a) as isize
    }
}

pub fn random_i64(a: i64, b: i64) -> i64 {
    if a > b {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = i128::from(rnd);
        let a = i128::from(a);
        let b = i128::from(b);

        ((rnd % (a - b + 1)) + b) as i64
    } else if a == b {
        a
    } else {
        let rnd: u64 = rand::thread_rng().gen();

        let rnd = i128::from(rnd);
        let a = i128::from(a);
        let b = i128::from(b);

        ((rnd % (b - a + 1)) + a) as i64
    }
}

#[cfg(target_pointer_width = "32")]
pub fn random_isize(a: isize, b: isize) -> isize {
    if a > b {
        let a = i64::from(a);
        let b = i64::from(b);

        rand::thread_rng().gen_range(b, a + 1) as isize
    } else if a == b {
        a
    } else {
        let a = i64::from(a);
        let b = i64::from(b);

        rand::thread_rng().gen_range(a, b + 1) as isize
    }
}

pub fn random_i32(a: i32, b: i32) -> i32 {
    if a > b {
        let a = i64::from(a);
        let b = i64::from(b);

        rand::thread_rng().gen_range(b, a + 1) as i32
    } else if a == b {
        a
    } else {
        let a = i64::from(a);
        let b = i64::from(b);

        rand::thread_rng().gen_range(a, b + 1) as i32
    }
}

#[cfg(target_pointer_width = "16")]
pub fn random_isize(a: isize, b: isize) -> isize {
    if a > b {
        let a = i32::from(a);
        let b = i32::from(b);

        rand::thread_rng().gen_range(b, a + 1) as isize
    } else if a == b {
        a
    } else {
        let a = i32::from(a);
        let b = i32::from(b);

        rand::thread_rng().gen_range(a, b + 1) as isize
    }
}

pub fn random_i16(a: i16, b: i16) -> i16 {
    if a > b {
        let a = i32::from(a);
        let b = i32::from(b);

        rand::thread_rng().gen_range(b, a + 1) as i16
    } else if a == b {
        a
    } else {
        let a = i32::from(a);
        let b = i32::from(b);

        rand::thread_rng().gen_range(a, b + 1) as i16
    }
}

#[cfg(target_pointer_width = "8")]
pub fn random_isize(a: isize, b: isize) -> isize {
    if a > b {
        let a = i16::from(a);
        let b = i16::from(b);

        rand::thread_rng().gen_range(b, a + 1) as isize
    } else if a == b {
        a
    } else {
        let a = i16::from(a);
        let b = i16::from(b);

        rand::thread_rng().gen_range(a, b + 1) as isize
    }
}

pub fn random_i8(a: i8, b: i8) -> i8 {
    if a > b {
        let a = i16::from(a);
        let b = i16::from(b);

        rand::thread_rng().gen_range(b, a + 1) as i8
    } else if a == b {
        a
    } else {
        let a = i16::from(a);
        let b = i16::from(b);

        rand::thread_rng().gen_range(a, b + 1) as i8
    }
}
