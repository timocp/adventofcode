// Numeric functions on integers
// TODO: currently all i128, should make generic over any integer type

// calculate (b ** e) mod m
//
// Implementation of:
// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
#[allow(dead_code)]
pub fn mod_pow(mut b: i128, mut e: i128, m: i128) -> i128 {
    if m == 1 {
        return 0;
    }
    let mut result: i128 = 1;
    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            result = result.checked_mul(b).expect("overflow in modular_pow") % m;
        }
        e >>= 1;
        b = b.checked_mul(b).expect("overflow in modular_pow") % m;
    }
    result
}

#[test]
fn test_mod_pow() {
    assert_eq!(445, mod_pow(4, 13, 497));
    assert_eq!(
        19,
        mod_pow(
            4776913109852041418248056622882488319,
            195845982777569926302400511,
            100
        )
    );
}

// GCD via Euclidean algorithm
// https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
#[allow(dead_code)]
fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(6, gcd(54, 24));
    assert_eq!(6, gcd(24, 54));
    assert_eq!(1, gcd(9, 28));
}

// Modular inverse of n
// Replacement for division in modular arithmetic
// return Some(x) such that: (n * x) mod m = 1
// return None if not possible
//
// Reference: https://rosettacode.org/wiki/Modular_inverse
pub fn mod_inv(n: i128, m: i128) -> Option<i128> {
    let mut mn = (m, n);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    if mn.0 > 1 {
        return None;
    }

    while xy.0 < 0 {
        xy.0 += m;
    }
    Some(xy.0)
}

#[test]
fn test_mod_inv() {
    assert_eq!(Some(1969), mod_inv(42, 2017));
    assert_eq!(None, mod_inv(3, 12));
}
