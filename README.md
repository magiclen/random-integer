Random Integer
====================

[![Build Status](https://travis-ci.org/magiclen/random-integer.svg?branch=master)](https://travis-ci.org/magiclen/random-integer)

Generate a random integer between two integer numbers (including the two integer numbers).

## Examples

```rust
extern crate random_integer;

let rnd = random_integer::random_u8(224, 255);

println!("{}", rnd);
```

```rust
extern crate random_integer;

let rnd: isize = random_integer::random(50, -20);

println!("{}", rnd);
```

## Crates.io

https://crates.io/crates/random-integer

## Documentation

https://docs.rs/random-integer

## License

[MIT](LICENSE)