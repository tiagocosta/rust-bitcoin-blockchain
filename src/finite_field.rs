use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct FiniteElement {
    num: i32,
    prime: i32,
}

impl FiniteElement {
    pub fn new(num: i32, prime: i32) -> Self {
        FiniteElement { num, prime }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let num = self.num.pow(exponent) % self.prime;
        Self::new(num, self.prime)
    }
}

impl PartialEq for FiniteElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FiniteElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let num = (self.num + other.num) % self.prime;
        Self::new(num, self.prime)
    }
}

impl Sub for FiniteElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // ((a % b) + b) % b // workaround for modulus operation in rust
        let num = self.num - other.num;
        let res = ((num % self.prime) + self.prime) % self.prime;
        Self::new(res, self.prime)
    }
}

impl Mul for FiniteElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let num = (self.num * rhs.num) % self.prime;
        Self::new(num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = FiniteElement::new(2, 31);
        let b = FiniteElement::new(2, 31);
        let c = FiniteElement::new(15, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
    
    #[test]
    fn test_add() {
        let a = FiniteElement::new(2, 31);
        let b = FiniteElement::new(15, 31);
        let res_add_a_b = FiniteElement::new(17, 31);
        assert_eq!(a + b, res_add_a_b);
        let c = FiniteElement::new(17, 31);
        let d = FiniteElement::new(21, 31);
        let res_add_c_d = FiniteElement::new(7, 31);
        assert_eq!(c + d, res_add_c_d);
    }
    
    #[test]
    fn test_sub() {
        let a = FiniteElement::new(29, 31);
        let b = FiniteElement::new(4, 31);
        let res_sub_a_b = FiniteElement::new(25, 31);
        assert_eq!(a - b, res_sub_a_b);
        let c = FiniteElement::new(15, 31);
        let d = FiniteElement::new(30, 31);
        let res_sub_c_d = FiniteElement::new(16, 31);
        assert_eq!(c - d, res_sub_c_d);
    }

    #[test]
    fn test_mul() {
        let a = FiniteElement::new(24, 31);
        let b = FiniteElement::new(19, 31);
        let res_mul_a_b = FiniteElement::new(22, 31);
        assert_eq!(a * b, res_mul_a_b);
    }

    #[test]
    fn test_pow() {
        let a = FiniteElement::new(17, 31);
        let a_pow_3 = FiniteElement::new(15, 31);
        assert_eq!(a.pow(3), a_pow_3);
        let b = FiniteElement::new(5, 31);
        let c = FiniteElement::new(18, 31);
        let res_b_pow_5_mul_c = FiniteElement::new(16, 31);
        assert_eq!(b.pow(5) * c, res_b_pow_5_mul_c);
    }
}
