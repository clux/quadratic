# Quadratic
[![build status](https://secure.travis-ci.org/clux/quadratic.svg)](http://travis-ci.org/clux/quadratic)

Number theoretic experiments with rust.

## Usage
Add as a git dependency (for now), and import quadratic:

```rust
extern crate quadratic;

let ls = quadratic::legendre(2, 5);
let js = quadratic::jacob(2, 15);
```
