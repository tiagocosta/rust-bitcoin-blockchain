use num_bigint::{BigInt, BigUint};
use std::ops::{Add, BitAnd, Mul};

use crate::cripto::Signature;
use crate::finite_field::{FieldElement, S256Field};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref N_S256: BigUint = BigUint::from_bytes_be(&hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap());

    static ref A: S256Field<'static> = S256Field::new(BigUint::from(0u32));
    static ref B: S256Field<'static> = S256Field::new(BigUint::from(7u32));
    static ref Gx: S256Field<'static> = S256Field::new(BigUint::from_bytes_be(&hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()));
    static ref Gy: S256Field<'static> = S256Field::new(BigUint::from_bytes_be(&hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap()));
}

#[derive(Debug, Clone)]
pub enum CoordsS256<'a> {
    Infinity,
    Finite(S256Field<'a>, S256Field<'a>),
}

#[derive(Debug, Clone)]
pub struct S256Point<'a>(pub Point<'a>);

impl<'a> S256Point<'a> {
    pub fn new (xy: CoordsS256<'a>) -> Self {
        match xy {
            CoordsS256::Infinity => S256Point(Point::at_inifity(&A.0, &B.0)),
            CoordsS256::Finite(s256_x, s256_y ) => {

                return S256Point(Point::new(Coords::Finite(s256_x.0, s256_y.0), &A.0, &B.0));
            }
        }
    }

    pub fn generator() -> Self {
        S256Point::new(CoordsS256::Finite(Gx.clone(), Gy.clone()))
    }

    pub fn xy(&self) -> Option<(&BigUint, &BigUint)> {
        match &self.0.xy {
            Coords::Finite(x, y) => Some((&x.num, &y.num)),
            _ => None,
        }
    }

    pub fn verify(&self, z: &BigUint, sig: &Signature) -> bool {
        let one = BigUint::from(1u32);
        let s_inv = &sig.s.modinv(&N_S256).unwrap();
        let u = (z * s_inv).modpow(&one, &N_S256);
        let v = (&sig.r * s_inv).modpow(&one, &N_S256);
        let g = S256Point::generator();
        let ug = &g * &u;
        let vp = self * &v;
        let r_point = &ug + &vp;
        match r_point.xy() {
            Some((x, _)) => *x == sig.r,
            None => false,
        }
    }
}

impl<'a> Mul<&'a BigUint> for &'a S256Point<'a> {
    type Output = S256Point<'a>;

    fn mul(self, other: &'a BigUint) -> S256Point<'a> {
        let one = BigUint::from(1u32);
        let coef = other.modpow(&one, &N_S256);
        let point = &self.0 * coef;
        S256Point(point)
    }   
}

impl<'a> Add<&'a S256Point<'a>> for &'a S256Point<'_> {
    type Output = S256Point<'a>;

    fn add(self, other: &'a S256Point) -> S256Point<'a> {
        let p = &self.0 + &other.0;
        match p.xy {
            Coords::Infinity => S256Point::new(CoordsS256::Infinity),
            Coords::Finite(x, y) => 
                S256Point::new(CoordsS256::Finite(S256Field::new(x.num), S256Field::new(y.num)))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Coords<'a> {
    Infinity,
    Finite(FieldElement<'a>, FieldElement<'a>),
}

#[derive(Debug, Clone)]
pub struct Point<'a> {
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

impl<'a> Mul<BigUint> for &'a Point<'a> {
    type Output = Point<'a>;

    // fn mul(self, other: u32) -> Point<'a> {
        
    //     let mut prod = Point::new(Coords::Infinity, self.a, self.b);
    //     for _ in 0..other {
    //         prod = self + prod;
    //     };
    //     prod
    // }   
    fn mul(self, other: BigUint) -> Point<'a> {
        
        let mut result= Point::new(Coords::Infinity, self.a, self.b);
        let mut current = self.clone();
        let mut coef = other;
        let zero = BigUint::ZERO;
        let one = BigUint::from(1u32);
        while coef > zero {
            let current1 = current.clone();
            let current2 = current;
            if coef.clone().bitand(&one) == one {
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

        assert_eq!(&p1 * scalar, p2);

        // (2, 143, 98, 64, 168)
        let x_1 = FieldElement::new(BigUint::from(143u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(98u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(64u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(168u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let scalar = BigUint::from(2u32);

        assert_eq!(&p1 * scalar, p2);

        // (2, 47, 71, 36, 111)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let x_2 = FieldElement::new(BigUint::from(36u32), &prime);
        let y_2 = FieldElement::new(BigUint::from(111u32), &prime);
        let coord_2 = Coords::Finite(x_2, y_2);
        let p2 = Point::new(coord_2, &a, &b);

        let scalar = BigUint::from(2u32);

        assert_eq!(&p1 * scalar, p2);

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

        assert_eq!(&p1 * scalar, p2);

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

        assert_eq!(&p1 * scalar, p2);

        // (21, 47, 71, None, None)
        let x_1 = FieldElement::new(BigUint::from(47u32), &prime);
        let y_1 = FieldElement::new(BigUint::from(71u32), &prime);
        let coord_1 = Coords::Finite(x_1, y_1);
        let p1 = Point::new(coord_1, &a, &b);

        let p2 = Point::new(Coords::Infinity, &a, &b);

        let scalar = BigUint::from(21u32);

        assert_eq!(&p1 * scalar, p2);

    }

    #[test]
    fn test_s256_point_generator() {
        let g = S256Point::generator();
        let p = &g * &N_S256;
        assert_eq!(p.0.xy, Coords::Infinity);
    }

    #[test]
    fn test_s256_verify() {
        let x = BigUint::from_bytes_be(&hex::decode("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c").unwrap());
        let y = BigUint::from_bytes_be(&hex::decode("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let point = S256Point::new(CoordsS256::Finite(S256Field::new(x), S256Field::new(y)));
        let z = BigUint::from_bytes_be(&hex::decode("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60").unwrap());
        let r = BigUint::from_bytes_be(&hex::decode("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395").unwrap());
        let s = BigUint::from_bytes_be(&hex::decode("068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4").unwrap());
        let sig = Signature::new(r, s);
        assert!(point.verify(&z, &sig));
        let z = BigUint::from_bytes_be(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let r = BigUint::from_bytes_be(&hex::decode("00eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap());
        let s = BigUint::from_bytes_be(&hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let sig = Signature::new(r, s);
        assert!(point.verify(&z, &sig));
    }
}