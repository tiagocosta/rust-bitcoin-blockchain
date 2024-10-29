use std::ops::{Add, Sub, Mul, Div};
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            panic!("num {} not in field range 0 to {}", num, prime - BigUint::from(1u32));
        }
        FieldElement { num, prime }
    }

    pub fn pow(&self, exp: &BigInt) -> Self {
        if exp.sign() == Sign::Minus {
            let inv = self.num.modinv(&self.prime).unwrap().to_bigint().unwrap();
            let num = inv.modpow(&exp.magnitude().to_bigint().unwrap(), &self.prime.to_bigint().unwrap());
            return FieldElement::new(num.to_biguint().unwrap(), self.prime.clone());
        }
        let num = self.num.modpow(&exp.to_biguint().unwrap(), &self.prime);
        FieldElement::new(num, self.prime.clone())
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if same_field(&self, &other) {
            panic!("can't add numbers in different fields");
        }
        let num = (&self.num + &other.num) % &self.prime;
        Self::new(num, self.prime)
    }
}

impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, other: &'b FieldElement) -> FieldElement {
        if same_field(self, other) {
            panic!("can't add numbers in different fields");
        }
        let num = (&self.num + &other.num) % &self.prime;
        FieldElement::new(num, self.prime.clone())
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if same_field(&self, &other) {
            panic!("can't subtract numbers in different fields");
        }
        // ((a % b) + b) % b // workaround for modulus operation of a negative number in rust
        let num = &self.num.to_bigint().unwrap() - &other.num.to_bigint().unwrap();
        let prime = &self.prime.to_bigint().unwrap();
        let res = ((num % prime) + prime) % prime;
        Self::new(res.to_biguint().unwrap(), self.prime)
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &'b FieldElement) -> FieldElement {
        if same_field(self, other) {
            panic!("can't subtract numbers in different fields");
        }
        // ((a % b) + b) % b // workaround for modulus operation of a negative number in rust
        let num = &self.num.to_bigint().unwrap() - &other.num.to_bigint().unwrap();
        let prime = &self.prime.to_bigint().unwrap();
        let res = ((num % prime) + prime) % prime;
        FieldElement::new(res.to_biguint().unwrap(), self.prime.clone())
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if same_field(&self, &rhs) {
            panic!("can't multiply numbers in different fields");
        }
        let num = (&self.num * &rhs.num) % &self.prime;
        Self::new(num, self.prime)
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &'b FieldElement) -> FieldElement {
        if same_field(self, other) {
            panic!("can't multiply numbers in different fields");
        }
        let num = (&self.num * &other.num) % &self.prime;
        FieldElement::new(num, self.prime.clone())
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.num == BigUint::from(0u32) {
            panic!("can't divide by zero!");
        }
        if same_field(&self, &rhs) {
            panic!("can't divide numbers in different fields");
        }
        self * rhs.pow(&BigInt::from(-1))
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn div(self, other: &'b FieldElement) -> FieldElement {
        if other.num == BigUint::from(0u32) {
            panic!("can't divide by zero!");
        }
        if same_field(self, other) {
            panic!("can't divide numbers in different fields");
        }
        self * &other.pow(&BigInt::from(-1))
    }
}

fn same_field(elem_1: &FieldElement, elem_2: &FieldElement) -> bool {
    elem_1.prime != elem_2.prime 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new_invalid_finite_element() {
        let num = BigUint::from(31u32);
        let prime = BigUint::from(29u32);
        FieldElement::new(num, prime);
    }

    #[test]
    fn test_new_valid_finite_element() {
        let num = BigUint::from(29u32);
        let prime = BigUint::from(31u32);
        let field_element = FieldElement::new(num, prime);
        assert_eq!(field_element.num, BigUint::from(29u32));
        assert_eq!(field_element.prime, BigUint::from(31u32));
    }

    #[test]
    fn test_eq() {

        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let c = FieldElement::new(BigUint::from(15u32), BigUint::from(31u32));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
    
    #[test]
    fn test_add() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(15u32), BigUint::from(31u32));
        let res_add_a_b = FieldElement::new(BigUint::from(17u32), BigUint::from(31u32));
        assert_eq!(&a + &b, res_add_a_b);
        assert_eq!(a + b, res_add_a_b);
        let c = FieldElement::new(BigUint::from(17u32), BigUint::from(31u32));
        let d = FieldElement::new(BigUint::from(21u32), BigUint::from(31u32));
        let res_add_c_d = FieldElement::new(BigUint::from(7u32), BigUint::from(31u32));
        assert_eq!(&c + &d, res_add_c_d);
        assert_eq!(c + d, res_add_c_d);
    }

    #[test]
    #[should_panic]
    fn test_add_elements_in_different_fields() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(15u32), BigUint::from(29u32));
        let _ = a + b;
    }
    
    #[test]
    fn test_sub() {
        let a = FieldElement::new(BigUint::from(29u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(4u32), BigUint::from(31u32));
        let res_sub_a_b = FieldElement::new(BigUint::from(25u32), BigUint::from(31u32));
        assert_eq!(&a - &b, res_sub_a_b);
        assert_eq!(a - b, res_sub_a_b);
        let c = FieldElement::new(BigUint::from(15u32), BigUint::from(31u32));
        let d = FieldElement::new(BigUint::from(30u32), BigUint::from(31u32));
        let res_sub_c_d = FieldElement::new(BigUint::from(16u32), BigUint::from(31u32));
        assert_eq!(&c - &d, res_sub_c_d);
        assert_eq!(c - d, res_sub_c_d);
    }

    #[test]
    #[should_panic]
    fn test_sub_elements_in_different_fields() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(15u32), BigUint::from(29u32));
        let _ = a - b;
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(BigUint::from(24u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(19u32), BigUint::from(31u32));
        let res_mul_a_b = FieldElement::new(BigUint::from(22u32), BigUint::from(31u32));
        assert_eq!(&a * &b, res_mul_a_b);
        assert_eq!(a * b, res_mul_a_b);
    }

    #[test]
    #[should_panic]
    fn test_mul_elements_in_different_fields() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(15u32), BigUint::from(29u32));
        let _ = a * b;
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(BigUint::from(17u32), BigUint::from(31u32));
        let a_pow_3 = FieldElement::new(BigUint::from(15u32), BigUint::from(31u32));
        assert_eq!(&a.pow(&BigInt::from(3)), &a_pow_3);
        assert_eq!(a.pow(&BigInt::from(3)), a_pow_3);
        let b = FieldElement::new(BigUint::from(5u32), BigUint::from(31u32));
        let c = FieldElement::new(BigUint::from(18u32), BigUint::from(31u32));
        let res_b_pow_5_mul_c = FieldElement::new(BigUint::from(16u32), BigUint::from(31u32));
        assert_eq!(&b.pow(&BigInt::from(5)) * &c, res_b_pow_5_mul_c);
        assert_eq!(b.pow(&BigInt::from(5)) * c, res_b_pow_5_mul_c);
        let d = FieldElement::new(BigUint::from(17u32), BigUint::from(31u32));
        let res_d_pow_minus3 =FieldElement::new(BigUint::from(29u32), BigUint::from(31u32));
        assert_eq!(&d.pow(&BigInt::from(-3)), &res_d_pow_minus3);
        assert_eq!(d.pow(&BigInt::from(-3)), res_d_pow_minus3);
        let e = FieldElement::new(BigUint::from(4u32), BigUint::from(31u32));
        let f = FieldElement::new(BigUint::from(11u32), BigUint::from(31u32));
        let res_e_pow_minus4_mul_f = FieldElement::new(BigUint::from(13u32), BigUint::from(31u32));
        assert_eq!(&e.pow(&BigInt::from(-4)) * &f, res_e_pow_minus4_mul_f);
        assert_eq!(e.pow(&BigInt::from(-4)) * f, res_e_pow_minus4_mul_f);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(BigUint::from(3u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(24u32), BigUint::from(31u32));
        let res_div_a_b = FieldElement::new(BigUint::from(4u32), BigUint::from(31u32));
        assert_eq!(&a/&b, res_div_a_b);
        assert_eq!(a/b, res_div_a_b);
    }

    #[test]
    #[should_panic]
    fn test_div_elements_in_different_fields() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(15u32), BigUint::from(29u32));
        let _ = a / b;
    }
}
