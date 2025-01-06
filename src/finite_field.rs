use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref P: BigUint = BigUint::from(2u32).pow(256) - BigUint::from(2u32).pow(32) - BigUint::from(977u32);
    pub static ref ONE: BigUint = BigUint::from(1u32);
}

pub type Num = BigUint;
pub type Prime = Rc<BigUint>;

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: Num,
    prime: Prime,
}

impl FieldElement {
    pub fn new(num: Num, prime: Prime) -> Self {
        if num >= *prime {
            panic!("num {} not in field of order {}", num, prime);
        }
        FieldElement { num, prime }
    }

    pub fn pow(&self, exp: &BigInt) -> Self {
        if exp.sign() == Sign::Minus {
            let inv = &(self.num.modinv(&self.prime).unwrap()).to_bigint().unwrap();
            let num = inv.modpow(&exp.magnitude().to_bigint().unwrap(), &self.prime.to_bigint().unwrap());
            return FieldElement::new(num.to_biguint().unwrap(), Rc::clone(&self.prime));
        }
        let num = self.num.modpow(&exp.to_biguint().unwrap(), &self.prime);
        FieldElement::new(num, Rc::clone(&self.prime))
    }

    pub fn num(&self) -> &Num {
        &self.num
    }

    pub fn prime(&self) -> Prime {
        Rc::clone(&self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl<'a> Add<&'a FieldElement> for &FieldElement {
    type Output = Result<FieldElement, &'static str>;

    fn add(self, other: &'a FieldElement) -> Self::Output {
        if same_field(self, other) {
            return Err("can't add numbers in different fields");
        }
        let num = (&self.num + &other.num).modpow(&BigUint::from(1u32), &self.prime);
        Ok(FieldElement::new(num, Rc::clone(&self.prime)))
    }
}

impl<'a> Sub<&'a FieldElement> for &FieldElement {
    type Output = Result<FieldElement, &'static str>;

    fn sub(self, other: &'a FieldElement) -> Self::Output {
        if same_field(self, other) {
            return Err("can't subtract numbers in different fields");
        }
        // ((a % b) + b) % b // workaround for modulus operation of a negative number in rust
        let num = &self.num.to_bigint().unwrap() - &other.num.to_bigint().unwrap();
        let prime = &self.prime.to_bigint().unwrap();
        let res = ((num % prime) + prime) % prime;
        Ok(FieldElement::new(res.to_biguint().unwrap(), Rc::clone(&self.prime)))
    }
}

impl<'a> Mul<&'a FieldElement> for &FieldElement {
    type Output = Result<FieldElement, &'static str>;

    fn mul(self, other: &'a FieldElement) -> Self::Output {
        if same_field(self, other) {
            return Err("can't multiply numbers in different fields");
        }
        let num = (&self.num * &other.num).modpow(&BigUint::from(1u32), &self.prime);
        Ok(FieldElement::new(num, Rc::clone(&self.prime)))
    }
}

impl<'a> Div<&'a FieldElement> for &FieldElement {
    type Output = Result<FieldElement, &'static str>;

    fn div(self, other: &'a FieldElement) -> Self::Output {
        if other.num == BigUint::from(0u32) {
            return Err("can't divide by zero!");
        }
        if same_field(self, other) {
            return Err("can't divide numbers in different fields");
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
        let prime = Rc::new(BigUint::from(29u32));
        FieldElement::new(num, prime);
    }

    #[test]
    fn test_new_valid_finite_element() {
        let num = BigUint::from(29u32);
        let prime = Rc::new(BigUint::from(31u32));
        let field_element = FieldElement::new(num, prime);
        assert_eq!(field_element.num, BigUint::from(29u32));
        assert_eq!(field_element.prime, Rc::new(BigUint::from(31u32)));
    }

    #[test]
    fn test_eq() {

        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(2u32), Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(2u32), Rc::clone(&prime));
        let c = FieldElement::new(BigUint::from(15u32), Rc::clone(&prime));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
    
    #[test]
    fn test_add() {
        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(2u32), Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(15u32), Rc::clone(&prime));
        let res_add_a_b = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        assert_eq!((&a + &b).unwrap(), res_add_a_b);
        let c = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        let d = FieldElement::new(BigUint::from(21u32), Rc::clone(&prime));
        let res_add_c_d = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        assert_eq!((&c + &d).unwrap(), res_add_c_d);
    }

    #[test]
    fn test_add_elements_in_different_fields() {
        let prime_1 = Rc::new(BigUint::from(31u32));
        let prime_2 = Rc::new(BigUint::from(29u32));
        let a = FieldElement::new(BigUint::from(2u32), prime_1);
        let b = FieldElement::new(BigUint::from(15u32), prime_2);
        let res = &a + &b;
        assert!(res.is_err());
    }
    
    #[test]
    fn test_sub() {
        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(29u32), Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(4u32), Rc::clone(&prime));
        let res_sub_a_b = FieldElement::new(BigUint::from(25u32), Rc::clone(&prime));
        assert_eq!((&a - &b).unwrap(), res_sub_a_b);
        let c = FieldElement::new(BigUint::from(15u32), Rc::clone(&prime));
        let d = FieldElement::new(BigUint::from(30u32), Rc::clone(&prime));
        let res_sub_c_d = FieldElement::new(BigUint::from(16u32), Rc::clone(&prime));
        assert_eq!((&c - &d).unwrap(), res_sub_c_d);
    }

    #[test]
    fn test_sub_elements_in_different_fields() {
        let prime_1 = Rc::new(BigUint::from(31u32));
        let prime_2 = Rc::new(BigUint::from(29u32));
        let a = FieldElement::new(BigUint::from(2u32), prime_1);
        let b = FieldElement::new(BigUint::from(15u32), prime_2);
        let res = &a - &b;
        assert!(res.is_err());
    }

    #[test]
    fn test_mul() {
        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(24u32), Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(19u32), Rc::clone(&prime));
        let res_mul_a_b = FieldElement::new(BigUint::from(22u32), Rc::clone(&prime));
        assert_eq!((&a * &b).unwrap(), res_mul_a_b);
    }

    #[test]
    fn test_mul_elements_in_different_fields() {
        let prime_1 = Rc::new(BigUint::from(31u32));
        let prime_2 = Rc::new(BigUint::from(29u32));
        let a = FieldElement::new(BigUint::from(2u32), prime_1);
        let b = FieldElement::new(BigUint::from(15u32), prime_2);
        let res = &a * &b;
        assert!(res.is_err());
    }

    #[test]
    fn test_pow() {
        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        let a_pow_3 = FieldElement::new(BigUint::from(15u32), Rc::clone(&prime));
        assert_eq!(&a.pow(&BigInt::from(3)), &a_pow_3);
        assert_eq!(a.pow(&BigInt::from(3)), a_pow_3);
        let b = FieldElement::new(BigUint::from(5u32), Rc::clone(&prime));
        let c = FieldElement::new(BigUint::from(18u32), Rc::clone(&prime));
        let res_b_pow_5_mul_c = FieldElement::new(BigUint::from(16u32), Rc::clone(&prime));
        assert_eq!((&b.pow(&BigInt::from(5)) * &c).unwrap(), res_b_pow_5_mul_c);
        let d = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        let res_d_pow_minus3 =FieldElement::new(BigUint::from(29u32), Rc::clone(&prime));
        assert_eq!(&d.pow(&BigInt::from(-3)), &res_d_pow_minus3);
        assert_eq!(d.pow(&BigInt::from(-3)), res_d_pow_minus3);
        let e = FieldElement::new(BigUint::from(4u32), Rc::clone(&prime));
        let f = FieldElement::new(BigUint::from(11u32), Rc::clone(&prime));
        let res_e_pow_minus4_mul_f = FieldElement::new(BigUint::from(13u32), Rc::clone(&prime));
        assert_eq!((&e.pow(&BigInt::from(-4)) * &f).unwrap(), res_e_pow_minus4_mul_f);
    }

    #[test]
    fn test_div() {
        let prime = Rc::new(BigUint::from(31u32));
        let a = FieldElement::new(BigUint::from(3u32), Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(24u32), Rc::clone(&prime));
        let res_div_a_b = FieldElement::new(BigUint::from(4u32), Rc::clone(&prime));
        assert_eq!((&a/&b).unwrap(), res_div_a_b);
    }

    #[test]
    fn test_div_elements_in_different_fields() {
        let prime_1 = Rc::new(BigUint::from(31u32));
        let prime_2 = Rc::new(BigUint::from(29u32));
        let a = FieldElement::new(BigUint::from(2u32), prime_1);
        let b = FieldElement::new(BigUint::from(15u32), prime_2);
        let res = &a / &b;
        assert!(res.is_err());
    }
}
