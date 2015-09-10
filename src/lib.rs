
/// Returns Legendre symbol for (a \ p)
///
///
/// # Examples
///
/// ```rust
/// # extern crate quadratic;
/// // compute the legendre symbol of (2 \ 5)
/// let symb = quadratic::legendre(2, 5);
/// ```

pub fn legendre(a: i32, p: u32) -> (i8) {
    let ls = (a.pow((p-1)/2) as u32) % p; // awkward type coercions here..
    if ls == p - 1 {
      return -1;
    }
    return ls as i8;
}


#[cfg(test)]
mod tests {
    use legendre;

    #[test]
    fn two_over_p() {
        assert_eq!(-1, legendre(2, 5));
        assert_eq!(-1, legendre(2, 3));
        assert_eq!(1, legendre(2, 7));
        assert_eq!(1, legendre(2, 17)); // 17 = 1 mod 8
    }
}
