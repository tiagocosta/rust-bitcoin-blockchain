// use std::rc::Rc;

// use std::ops::{Add, Div, Mul, Sub};

// use num_bigint::{BigInt, BigUint, ToBigInt};

// use crate::finite_field::{FieldElement, P};

// #[derive(Debug, Clone)]
// pub struct S256Field (pub FieldElement);

// impl S256Field {
//     pub fn new(num: BigUint) -> Self {
//         S256Field(FieldElement::new(num, Rc::new(&P)))
//     }

//     fn from_field_element(field_element: FieldElement) -> Self {
//         S256Field::new(field_element.num().clone())
//     }

//     pub fn from_S256_field(other: S256Field) -> Self {
//         S256Field::new(other.num().clone())
//     }

//     pub fn num(&self) -> &BigUint {
//         self.0.num()
//     }

//     pub fn sqrt(&self) -> Self {
//         let exp = (&P.clone() + BigUint::from(1u32)) / BigUint::from(4u32);
//         S256Field(self.0.pow(&exp.to_bigint().unwrap()))
//     }

//     pub fn pow(&self, exp: &BigInt) -> Self {
//         S256Field(self.0.pow(&exp.to_bigint().unwrap()))
//     }
// }

// impl PartialEq for S256Field {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }

// impl Add for S256Field {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         S256Field::from_field_element(self.0 + other.0)
//     }
// }

// impl<'a, 'b> Add<&'b S256Field> for &'a S256Field {
//     type Output = S256Field;

//     fn add(self, other: &'b S256Field) -> S256Field {
//         S256Field::from_field_element(&self.0 + &other.0)
//     }
// }

// impl Sub for S256Field {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         S256Field::from_field_element(self.0 - other.0)
//     }
// }

// impl<'a, 'b> Sub<&'b S256Field> for &'a S256Field {
//     type Output = S256Field;

//     fn sub(self, other: &'b S256Field) -> S256Field {
//         S256Field::from_field_element(&self.0 - &other.0)
//     }
// }

// impl Mul for S256Field {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self {
//         S256Field::from_field_element(self.0 * rhs.0)
//     }
// }

// impl<'a, 'b> Mul<&'b S256Field> for &'a S256Field {
//     type Output = S256Field;

//     fn mul(self, other: &'b S256Field) -> S256Field {
//         S256Field::from_field_element(&self.0 * &other.0)
//     }
// }

// impl Div for S256Field {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self::Output {
//         S256Field::from_field_element(self.0 / rhs.0)
//     }
// }

// impl<'a, 'b> Div<&'b S256Field> for &'a S256Field {
//     type Output = S256Field;

//     fn div(self, other: &'b S256Field) -> S256Field {
//         S256Field::from_field_element(&self.0 / &other.0)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new_s256field() {
//         let new_s256field = S256Field::new(BigUint::from(15u32));
//         assert_eq!(*new_s256field.0.prime(), Rc::new(P.clone()))
        
//     }
// }