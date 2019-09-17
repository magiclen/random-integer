extern crate random_integer;

#[test]
fn test_random_u64() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_u64(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_u32() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_u32(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_u16() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_u16(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_u8() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_u8(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_usize() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_usize(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_i64() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_i64(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_i32() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_i32(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_i16() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_i16(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_i8() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_i8(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}

#[test]
fn test_random_isize() {
    let mut result = Vec::new();

    let n = 1_000_000;

    let nn = n / 10;

    for _ in 0..n {
        result.push(random_integer::random_isize(0, 9));
    }

    let mut counter = [0usize; 10];

    for i in result {
        counter[i as usize] += 1;
    }

    let mut errs = [0f64; 10];

    for (i, &c) in counter.iter().enumerate() {
        errs[i] = (((nn as isize) - (c as isize)) as f64).abs() / f64::from(nn);
    }

    for &err in errs.iter() {
        assert!(err < 0.025);
    }
}
