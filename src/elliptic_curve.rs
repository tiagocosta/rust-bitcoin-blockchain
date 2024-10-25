use num_bigint::{BigInt, BigUint};
use std::ops::Add;

use crate::finite_field::FiniteElement;

#[derive(Debug, PartialEq, Clone)]
pub enum Coords {
    Infinity,
    Finite(FiniteElement, FiniteElement),
}

#[derive(Debug, Clone)]
pub struct Point {
    xy: Coords,
    a: FiniteElement,
    b: FiniteElement,
}

impl Point {
    pub fn new(xy: Coords, a: FiniteElement, b: FiniteElement) -> Self {
        if let Coords::Finite(x, y) = &xy {
            if y.pow(&BigInt::from(2)) != &(x.pow(&BigInt::from(3)) + x*&a) + &b {
                    panic!("({:#?}, {:#?}) is not on the curve", x, y);     
            };
        };

        Point { xy, a, b }
    }  
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy && self.a == other.a && self.b == other.b
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
        }

        match &self.xy {
            Coords::Infinity => other,
            Coords::Finite(x1, y1) => {
                match &other.xy {
                    Coords::Infinity => self,
                    Coords::Finite(x2, y2) => {
                        if x1 == x2 && y1 != y2 {
                            Point::new(Coords::Infinity, self.a, self.b)
                        } else {
                            if self == other {
                                if *y1 == &FiniteElement::new(BigUint::ZERO, y1.prime.clone()) * x1 {
                                    return Point::new(Coords::Infinity, self.a, self.b);
                                }
                                let s= (&(&FiniteElement::new(BigUint::from(3u32), y1.prime.clone()) * &x1.pow(&BigInt::from(2u32))) + &self.a) / (&FiniteElement::new(BigUint::from(2u32), y1.prime.clone()) * y1);
                                let x = s.pow(&BigInt::from(2)) - &FiniteElement::new(BigUint::from(2u32), x1.prime.clone()) * x1;
                                let y = &(s * (x1 - &x)) - y1;
                                return Point::new(Coords::Finite(x, y), self.a, self.b);
                            } 
                                let s = (y2 - y1) / (x2 - x1);
                                let x = &(&(s.pow(&BigInt::from(2))) - x1) - x2;
                                let y = &(&s * &(x1 - &x)) - y1;
                                Point::new(Coords::Finite(x, y), self.a, self.b)
                            
                        }
                    }
                }
            }
        }
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
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
                            Point::new(Coords::Infinity, self.a.clone(), self.b.clone())
                        } else {
                            if self == other {
                                if *y1 == FiniteElement::new(BigUint::ZERO, y1.prime.clone()) * x1.clone() {
                                    return Point::new(Coords::Infinity, self.a.clone(), self.b.clone());
                                }
                                let s= (&(&FiniteElement::new(BigUint::from(3u32), y1.prime.clone()) * &x1.pow(&BigInt::from(2u32))) + &self.a) / (&FiniteElement::new(BigUint::from(2u32), y1.prime.clone()) * y1);
                                let x = s.pow(&BigInt::from(2)) - &FiniteElement::new(BigUint::from(2u32), x1.prime.clone()) * x1;
                                let y = &(s * (x1 - &x)) - y1;
                                return Point::new(Coords::Finite(x, y), self.a.clone(), self.b.clone());
                            } 
                                let s = (y2 - y1) / (x2 - x1);
                                let x = &(&(s.pow(&BigInt::from(2))) - x1) - x2;
                                let y = &(&s * &(x1 - &x)) - y1;
                                Point::new(Coords::Finite(x, y), self.a.clone(), self.b.clone())
                            
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_points() {
        let prime = BigUint::from(223u32);
        let a = FiniteElement::new(BigUint::ZERO, prime.clone());
        let b = FiniteElement::new(BigUint::from(7u32), prime.clone());
        
        let x_1 = FiniteElement::new(BigUint::from(192u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(105u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(17u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(56u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        Point::new(coord_2, a.clone(), b.clone());

        let x_3 = FiniteElement::new(BigUint::from(1u32), prime.clone());
        let y_3 = FiniteElement::new(BigUint::from(193u32), prime.clone());
        let coord_3 = Coords::Finite(x_3, y_3);
        Point::new(coord_3, a.clone(), b.clone());
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_points() {
        let prime = BigUint::from(223u32);
        let a = FiniteElement::new(BigUint::ZERO, prime.clone());
        let b = FiniteElement::new(BigUint::from(7u32), prime.clone());
        
        let x_1 = FiniteElement::new(BigUint::from(200u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(119u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(42u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(99u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        Point::new(coord_2, a.clone(), b.clone());
    }

    #[test]
    fn test_eq() {
        let prime = BigUint::from(223u32);
        let a = FiniteElement::new(BigUint::ZERO, prime.clone());
        let b = FiniteElement::new(BigUint::from(7u32), prime.clone());
        
        let x_1 = FiniteElement::new(BigUint::from(192u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(105u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(192u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(105u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, a.clone(), b.clone());

        let x_3 = FiniteElement::new(BigUint::from(1u32), prime.clone());
        let y_3 = FiniteElement::new(BigUint::from(193u32), prime.clone());
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, a.clone(), b.clone());

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_add_inf() {
        let prime = BigUint::from(223u32);
        let a = FiniteElement::new(BigUint::ZERO, prime.clone());
        let b = FiniteElement::new(BigUint::from(7u32), prime.clone());

        let p_inf = Point::new(Coords::Infinity, a.clone(), b.clone());
        
        let x_1 = FiniteElement::new(BigUint::from(192u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(105u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, a.clone(), b.clone());

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
        let a = FiniteElement::new(BigUint::ZERO, prime.clone());
        let b = FiniteElement::new(BigUint::from(7u32), prime.clone());
        
        let x_1 = FiniteElement::new(BigUint::from(192u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(105u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(17u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(56u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, a.clone(), b.clone());

        let x_3 = FiniteElement::new(BigUint::from(170u32), prime.clone());
        let y_3 = FiniteElement::new(BigUint::from(142u32), prime.clone());
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);

        // (47, 71, 117, 141, 60, 139)
        let x_1 = FiniteElement::new(BigUint::from(47u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(71u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(117u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(141u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, a.clone(), b.clone());

        let x_3 = FiniteElement::new(BigUint::from(60u32), prime.clone());
        let y_3 = FiniteElement::new(BigUint::from(139u32), prime.clone());
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);

        // (143, 98, 76, 66, 47, 71)
        let x_1 = FiniteElement::new(BigUint::from(143u32), prime.clone());
        let y_1 = FiniteElement::new(BigUint::from(98u32), prime.clone());
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, a.clone(), b.clone());

        let x_2 = FiniteElement::new(BigUint::from(76u32), prime.clone());
        let y_2 = FiniteElement::new(BigUint::from(66u32), prime.clone());
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, a.clone(), b.clone());

        let x_3 = FiniteElement::new(BigUint::from(47u32), prime.clone());
        let y_3 = FiniteElement::new(BigUint::from(71u32), prime.clone());
        let coord_3 = Coords::Finite(x_3, y_3);
        let p3 = Point::new(coord_3, a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);
    }
}