use num_bigint::{BigInt, BigUint};
use std::ops::{Add, BitAnd, Mul};

use crate::finite_field::{FieldElement, S256Field};

use lazy_static::lazy_static;

lazy_static! {
    static ref A: S256Field<'static> = S256Field::new(BigUint::from(0u32));
    static ref B: S256Field<'static> = S256Field::new(BigUint::from(7u32));
}

#[derive(Debug, Clone)]
pub enum CoordsS256<'a> {
    Infinity,
    Finite(S256Field<'a>, S256Field<'a>),
}

#[derive(Debug, Clone)]
pub struct S256Point<'a> {
    point: Point<'a>
}

impl<'a> S256Point<'a> {
    pub fn new (xy: CoordsS256<'a>) -> Self {
        match xy {
            CoordsS256::Infinity => S256Point { point: Point::at_inifity(&A.element, &B.element) },
            CoordsS256::Finite(s256_x, s256_y ) => {

                return S256Point { point: Point::new(Coords::Finite(s256_x.element, s256_y.element), &A.element, &B.element) };
            }
        }
    }
}

impl<'a> Mul<&'a BigUint> for &'a S256Point<'a> {
    type Output = S256Point<'a>;

    fn mul(self, other: &'a BigUint) -> S256Point<'a> {
        // TODO use other % N for coef
        S256Point{ point: &self.point * other }
    }   
}

#[derive(Debug, PartialEq, Clone)]
enum Coords<'a> {
    Infinity,
    Finite(FieldElement<'a>, FieldElement<'a>),
}

#[derive(Debug, Clone)]
struct Point<'a> {
    xy: Coords<'a>,
    a: &'a FieldElement<'a>,
    b: &'a FieldElement<'a>,
}

impl<'a> Point<'a> {
    fn new(xy: Coords<'a>, a: &'a FieldElement, b: &'a FieldElement) -> Self {
        if let Coords::Finite(x, y) = &xy {
            if y.pow(&BigInt::from(2)) != &(x.pow(&BigInt::from(3)) + x*a) + b {
                    panic!("({:#?}, {:#?}) is not on the curve", x, y);     
            };
        };

        Point { xy, a, b }
    }

    fn at_inifity(a: &'a FieldElement, b: &'a FieldElement) -> Self {
        Point::new(Coords::Infinity, a, b)
    }

    fn from_slope(x1: FieldElement<'a>, y1: FieldElement<'a>, x2: FieldElement<'a>, y2: FieldElement<'a>, a: &'a FieldElement<'a>, b: &'a FieldElement<'a>) -> Self {
        let exp = BigInt::from(2);
        if x1 == x2 && y1 == y2 {
            let field_elem_2 = FieldElement::new(BigUint::from(2u32), x1.prime);
            let field_elem_3 = FieldElement::new(BigUint::from(3u32), x1.prime);
            let s = (field_elem_3 * x1.pow(&exp) + a.clone()) / (field_elem_2.clone() * y1.clone());
            let x = s.pow(&BigInt::from(2)) - field_elem_2 * x1.clone();
            let y = s.clone() * (x1 - x.clone()) - y1;
            Point {xy: Coords::Finite(x, y), a, b}
        } else {
           let s = (y2.clone() - y1.clone()) / (x2.clone() - x1.clone());
           let x = s.pow(&exp) - x1.clone() - x2;
           let y = (s.clone() * (x1 - x.clone())) - y1;
           Point {xy: Coords::Finite(x, y), a, b}
        }
    }
}



impl PartialEq for Point<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy && self.a == other.a && self.b == other.b
    }
}

impl Add for Point<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
        }

        match self.xy {
            Coords::Infinity => other,
            Coords::Finite(ref x1, ref y1) => {
                match other.xy {
                    Coords::Infinity => self.clone(),
                    Coords::Finite(ref x2, ref y2) => {
                        if x1 == x2 && y1 != y2 {
                            Point::at_inifity(self.a, self.b)
                        } else {
                            if y1 == &(&FieldElement::new(BigUint::ZERO, y1.prime) * x1) {
                                return Point::at_inifity(self.a, self.b);
                            }
                            Point::from_slope(x1.clone(), y1.clone(), x2.clone(), y2.clone(), self.a, self.b)
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Add<&'a Point<'a>> for &'a Point<'_> {
    type Output = Point<'a>;

    fn add(self, other: &'a Point) -> Point<'a> {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
        }

        match &self.xy {
            Coords::Infinity => other.clone(),
            Coords::Finite(x1, y1) => {
                match &other.xy {
                    Coords::Infinity => self.clone(),
                    Coords::Finite(x2, y2) => {
                        if x1 == x2 && y1 != y2 {
                            Point::at_inifity(self.a, self.b)
                        } else {
                            if *y1 == FieldElement::new(BigUint::ZERO, y1.prime) * x1.clone() {
                                return Point::at_inifity(self.a, self.b);
                            }
                            Point::from_slope(x1.clone(), y1.clone(), x2.clone(), y2.clone(), self.a, self.b)
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Add<Point<'a>> for &'a Point<'a> {
    type Output = Point<'a>;

    fn add(self, other: Point<'a>) -> Point<'a> {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
        }

        match &self.xy {
            Coords::Infinity => other,
            Coords::Finite(x1, y1) => {
                match &other.xy {
                    Coords::Infinity => self.clone(),
                    Coords::Finite(x2, y2) => {
                        if x1 == x2 && y1 != y2 {
                            Point::at_inifity(self.a, self.b)
                        } else {
                            if *y1 == FieldElement::new(BigUint::ZERO, y1.prime) * x1.clone() {
                                return Point::at_inifity(self.a, self.b);
                            }
                            Point::from_slope(x1.clone(), y1.clone(), x2.clone(), y2.clone(), self.a, self.b)
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Mul<&'a BigUint> for &'a Point<'a> {
    type Output = Point<'a>;

    // fn mul(self, other: u32) -> Point<'a> {
        
    //     let mut prod = Point::new(Coords::Infinity, self.a, self.b);
    //     for _ in 0..other {
    //         prod = self + prod;
    //     };
    //     prod
    // }   
    fn mul(self, other: &'a BigUint) -> Point<'a> {
        
        let mut result= Point::new(Coords::Infinity, self.a, self.b);
        let mut current = self.clone();
        let mut coef = other.clone();
        let zero = BigUint::ZERO;
        let one = BigUint::from(1u32);
        while &coef > &zero {
            let current1 = current.clone();
            let current2 = current;
            if &coef.clone().bitand(&one) == &one {
                result = result + current1.clone();
            }
            current = current1 + current2;
            coef >>= 1u32;
        };

        result
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_points() {
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);
        
        let x_1 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(17u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(56u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        Point::new(coord_2, &a, &b);

        let x_3 = FieldElement::new(BigUint::from(1u32), &prime);
        let y_3 = FieldElement::new(BigUint::from(193u32), &prime);
        let coord_3 = Coords::Finite(x_3, y_3);
        Point::new(coord_3, &a, &b);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_points() {
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);
        
        let x_1 = FieldElement::new(BigUint::from(200u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(119u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(42u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(99u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        Point::new(coord_2, &a, &b);
    }

    #[test]
    fn test_eq() {
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);
        
        let x_1 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let x_3 = FieldElement::new(BigUint::from(1u32), &prime);
        let y_3 = FieldElement::new(BigUint::from(193u32), &prime);
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, &a, &b);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_add_inf() {
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);

        let p_inf = Point::new(Coords::Infinity, &a, &b);
        
        let x_1 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        assert_eq!(&p_inf + &p1, p1.clone());
        assert_eq!(&p1 + &p_inf, p1.clone());
        assert_eq!(&p_inf + &p_inf, p_inf.clone());

        assert_eq!(p_inf.clone() + p1.clone(), p1.clone());
        assert_eq!(p1.clone() + p_inf.clone(), p1.clone());
        assert_eq!(p_inf.clone() + p_inf.clone(), p_inf);
    }

    #[test]
    fn test_add() {
        // (192, 105, 17, 56, 170, 142)
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);
        
        let x_1 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(17u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(56u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let x_3 = FieldElement::new(BigUint::from(170u32), &prime);
        let y_3 = FieldElement::new(BigUint::from(142u32), &prime);
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, &a, &b);

        assert_eq!(p1 + p2, p3);

        // (47, 71, 117, 141, 60, 139)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(117u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(141u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let x_3 = FieldElement::new(BigUint::from(60u32), &prime);
        let y_3 = FieldElement::new(BigUint::from(139u32), &prime);
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, &a, &b);

        assert_eq!(p1 + p2, p3);

        // (143, 98, 76, 66, 47, 71)
        let x_1 = FieldElement::new(BigUint::from(143u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(98u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(76u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(66u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let x_3 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_3 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, &a, &b);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_mul() {
        // (2, 192, 105, 49, 71)
        let prime = BigUint::from(223u32);
        let a = FieldElement::new(BigUint::ZERO, &prime);
        let b = FieldElement::new(BigUint::from(7u32), &prime);
        
        let x_1 = FieldElement::new(BigUint::from(192u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(105u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(49u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let scalar = BigUint::from(2u32);

        assert_eq!(&p1 * &scalar, p2);

        // (2, 143, 98, 64, 168)
        let x_1 = FieldElement::new(BigUint::from(143u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(98u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(64u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(168u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        assert_eq!(&p1 * &scalar, p2);

        // (2, 47, 71, 36, 111)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(36u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(111u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        assert_eq!(&p1 * &scalar, p2);

        // (4, 47, 71, 194, 51)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(194u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(51u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let scalar = BigUint::from(4u32);

        assert_eq!(&p1 * &scalar, p2);

        // (8, 47, 71, 116, 55)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(116u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(55u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let scalar = BigUint::from(8u32);

        assert_eq!(&p1 * &scalar, p2);

        // (21, 47, 71, None, None)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let p2 = Point::new(Coords::Infinity, &a, &b);

        let scalar = BigUint::from(21u32);

        assert_eq!(&p1 * &scalar, p2);

    }
}