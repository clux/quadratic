# Quadratic
[![build status](https://secure.travis-ci.org/clux/quadratic.svg)](http://travis-ci.org/clux/quadratic)

Number theoretic explorations with rust. Calculates the [Legendre](http://mathworld.wolfram.com/LegendreSymbol.html) and [Jacobi symbol](http://mathworld.wolfram.com/JacobiSymbol.html) using the [standard algorithm](https://en.wikipedia.org/wiki/Jacobi_symbol#Calculating_the_Jacobi_symbol). Since the Jacobi symbol is a generalization of the Legendre symbol, only this function is provided.

## Usage
Add as a git dependency (for now), and import quadratic:

```rust
extern crate quadratic;

assert_eq!(-1, quadratic::jacobi(2, 5));
assert_eq(1, quadratic::jacobi(2, 15));
```

## License
MIT-Licensed. See LICENSE file for details.
