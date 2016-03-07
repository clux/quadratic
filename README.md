# Quadratic
[![build status](https://secure.travis-ci.org/clux/quadratic.svg)](http://travis-ci.org/clux/quadratic)
[![coverage status](http://img.shields.io/coveralls/clux/quadratic.svg)](https://coveralls.io/r/clux/quadratic)
[![crates status](https://img.shields.io/crates/v/quadratic.svg)](https://crates.io/crates/quadratic)

Number theoretic helpers. Calculates the [Legendre](http://mathworld.wolfram.com/LegendreSymbol.html) and [Jacobi symbol](http://mathworld.wolfram.com/JacobiSymbol.html) using the [standard algorithm](https://en.wikipedia.org/wiki/Jacobi_symbol#Calculating_the_Jacobi_symbol). Since the Jacobi symbol is a generalization of the Legendre symbol, only this function is provided.

## Usage
Add `quadratic` as a dependency in `Cargo.toml`, then:

```rust
extern crate quadratic;

assert_eq!(-1, quadratic::jacobi(2, 5));
assert_eq!(1, quadratic::jacobi(2, 15));
```

## License
MIT-Licensed. See LICENSE file for details.
