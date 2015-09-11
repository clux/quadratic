extern crate num;
use num::Integer;

/// Supplementary law
fn two_over(n: u32) -> (i8) {
    if n % 8 == 1 || n % 8 == 7 {
        return 1;
    }
    return -1; // otherwise n is 3 or 5 mod 8 since n is odd
}

/// Legendre's version of quadratic reciprocity law
/// Returns the sign change needed to keep track of if flipping
fn reciprocity(num: u32, den: u32) -> (i8) {
    if num % 4 == 3 && den % 4 == 3 {
        return -1;
    }
    return 1;
}

/// Returns the value of the Jacobi symbol for (a \ n)
/// Where a is an integer and n is an odd positive integer
///
///
/// # Examples
///
/// ```rust
/// # extern crate quadratic;
/// // compute the Jacobi symbol of (2 \ 5)
/// let symb = quadratic::jacobi(2, 5);
/// # assert_eq!(-1, symb);
/// ```

pub fn jacobi(a_: i32, n_: i32) -> (i8) {
    assert!(n_.is_odd());
    assert!(n_ > 0);
    // ensure we start out with u32s
    let a = a_.mod_floor(&n_) as u32;
    let n = n_ as u32;

    // start algorithm:
    let mut acc = 1;
    let mut num = a;
    let mut den = n;
    loop {
        // reduce numerator
        num = num % den;
        if num == 0 {
            return 0;
        }

        // extract factors of two from numerator
        while num.is_even() {
            acc *= two_over(den);
            num /= 2;
        }
        // if numerator is 1 => this sub-symbol is 1
        if num == 1 {
            return acc;
        }
        // shared factors => one sub-symbol is zero
        if num.gcd(&den) > 1 {
            return 0;
        }
        // num and den are now odd co-prime, use reciprocity law:
        acc *= reciprocity(num, den);
        let tmp = num;
        num = den;
        den = tmp;
    }
}

#[cfg(test)]
mod tests {
    use jacobi;

    // legendre tests
    #[test]
    fn minus_one_over_p() {
        // 1 mod 4 => 1
        assert_eq!(1, jacobi(-1, 5));
        assert_eq!(1, jacobi(-1, 13));
        // 3 mod 4 => -1
        assert_eq!(-1, jacobi(-1, 3));
        assert_eq!(-1, jacobi(-1, 7));
    }
    #[test]
    fn two_over_p() {
        assert_eq!(-1, jacobi(2, 3));
        assert_eq!(-1, jacobi(2, 5));
        assert_eq!(1, jacobi(2, 7));
        assert_eq!(1, jacobi(2, 17)); // 17 = 1 mod 8
    }
    #[test]
    fn three_over_p() {
        assert_eq!(0, jacobi(3, 3));
        assert_eq!(-1, jacobi(3, 5));
        assert_eq!(-1, jacobi(3, 7));
    }
    #[test]
    fn periodicity() {
        assert_eq!(jacobi(3,5), jacobi(-2,5));
        assert_eq!(jacobi(-1,5), jacobi(4,5));
        assert_eq!(jacobi(11,7), jacobi(4,7));
        assert_eq!(jacobi(-3,7), jacobi(4,7));
        assert_eq!(jacobi(10,7), jacobi(3,7));
    }

    // jacobi tests
    #[test]
    fn a_over_n() { // 45 = 3*3*5
        assert_eq!(-1, jacobi(2, 45)); // -1 * -1 * -1
        assert_eq!(0, jacobi(3, 45)); // 0 * 0 * -1
        assert_eq!(-1, jacobi(7, 45)); // -1 * -1 * -1
        assert_eq!(1, jacobi(2, 15)); // (2\5) * (2\3)
    }
}
