extern crate primes;

/// Returns the value of the Legendre symbol for (a \ p)
/// Where a is an integer and p is an odd prime number
///
///
/// # Examples
///
/// ```rust
/// # extern crate quadratic;
/// // compute the Legendre symbol of (2 \ 5)
/// let symb = quadratic::legendre(2, 5);
/// # assert_eq!(-1, symb);
/// ```

pub fn legendre(a: i32, p: u32) -> (i8) {
    assert!(p % 2 == 1); // also assume p is prime - but silly to check here..
    let mut a_ = a;
    while a_ < 0 {
        a_ += p as i32;
    }
    let b = a_ as u32; // safe to assume >= 0 now
    let ls = b.pow((p-1)/2) % p; // can use u32 version of pow + rem..
    if ls == p - 1 {
      return -1 as i8;
    }
    ls as i8
}

/// Returns the value of the Jacobi symbol for (a \ n)
/// Where a is an integer and n is an odd integer > 2
///
///
/// # Examples
///
/// ```rust
/// # extern crate quadratic;
/// // compute the Jacobi symbol of (2 \ 15)
/// let symb = quadratic::jacobi(2, 15);
/// # assert_eq!(1, symb);
/// ```

pub fn jacobi(a: i32, n: u32) -> (i8) {
    assert!(n % 2 == 1);
    primes::factors(n as u64).iter().fold(1, |acc, &el| acc * legendre(a, el as u32))
}


#[cfg(test)]
mod tests {
    use legendre;

    #[test]
    fn minus_one_over_p() {
        // 1 mod 4 => 1
        assert_eq!(1, legendre(-1, 5));
        assert_eq!(1, legendre(-1, 13));
        // 3 mod 4 => -1
        assert_eq!(-1, legendre(-1, 3));
        assert_eq!(-1, legendre(-1, 7));
    }
    #[test]
    fn two_over_p() {
        assert_eq!(-1, legendre(2, 3));
        assert_eq!(-1, legendre(2, 5));
        assert_eq!(1, legendre(2, 7));
        assert_eq!(1, legendre(2, 17)); // 17 = 1 mod 8
    }
    #[test]
    fn three_over_p() {
        assert_eq!(0, legendre(3, 3));
        assert_eq!(-1, legendre(3, 5));
        assert_eq!(-1, legendre(3, 7));
    }
    #[test]
    fn legendre_periodicity() {
        assert_eq!(legendre(3,5), legendre(-2,5));
        assert_eq!(legendre(-1,5), legendre(4,5));
        assert_eq!(legendre(11,7), legendre(4,7));
        assert_eq!(legendre(-3,7), legendre(4,7));
        assert_eq!(legendre(10,7), legendre(3,7));
    }

    use jacobi;
    #[test]
    fn a_over_45() { // 45 = 3*3*5
        assert_eq!(-1, jacobi(2, 45)); // -1 * -1 * -1
        assert_eq!(0, jacobi(3, 45)); // 0 * 0 * -1
        assert_eq!(-1, jacobi(7, 45)); // -1 * -1 * -1
    }
}
