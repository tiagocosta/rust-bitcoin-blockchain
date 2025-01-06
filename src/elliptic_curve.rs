use num_bigint::{BigInt, BigUint};
use std::ops::{Add, BitAnd, Mul};
use std::rc::Rc;

// use crate::cripto::{hash160, encode_base58_checksum, Signature};
use crate::finite_field::{FieldElement, P};
// use crate::secp256k1::S256Field;

use lazy_static::lazy_static;

// lazy_static! {
//     pub static ref N_S256: BigUint = BigUint::from_bytes_be(&hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap());

//     static ref A: S256Field = S256Field::new(BigUint::from(0u32));
//     static ref B: S256Field = S256Field::new(BigUint::from(7u32));
//     static ref Gx: S256Field = S256Field::new(BigUint::from_bytes_be(&hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap()));
//     static ref Gy: S256Field = S256Field::new(BigUint::from_bytes_be(&hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap()));
// }

// #[derive(Debug, Clone)]
// pub enum CoordsS256 {
//     Infinity,
//     Finite(S256Field, S256Field),
// }

// #[derive(Debug, Clone)]
// pub struct S256Point(pub Point);

// impl S256Point {
//     pub fn new (xy: CoordsS256) -> Self {
//         match xy {
//             CoordsS256::Infinity => S256Point(Point::at_inifity(&A.0, &B.0)),
//             CoordsS256::Finite(s256_x, s256_y ) => {

//                 return S256Point(Point::new(Coords::Finite(s256_x.0, s256_y.0), &A.0, &B.0));
//             }
//         }
//     }

//     pub fn generator() -> Self {
//         S256Point::new(CoordsS256::Finite(Gx.clone(), Gy.clone()))
//     }

//     pub fn xy(&self) -> Option<(&BigUint, &BigUint)> {
//         match &self.0.xy {
//             Coords::Finite(x, y) => Some((&x.num, &y.num)),
//             _ => None,
//         }
//     }

//     pub fn verify(&self, z: &BigUint, sig: &Signature) -> bool {
//         let one = BigUint::from(1u32);
//         let s_inv = &sig.s.modinv(&N_S256).unwrap();
//         let u = (z * s_inv).modpow(&one, &N_S256);
//         let v = (&sig.r * s_inv).modpow(&one, &N_S256);
//         let g = S256Point::generator();
//         let ug = &g * &u;
//         let vp = self * &v;
//         let r_point = &ug + &vp;
//         match r_point.xy() {
//             Some((x, _)) => *x == sig.r,
//             None => false,
//         }
//     }

//     pub fn uncompressed_sec(&self) -> Vec<u8> {
//         let (x, y) = self.xy().unwrap();
//         let mut x_be = x.to_bytes_be();
//         let mut y_be = y.to_bytes_be();
//         let mut marker_be = BigUint::parse_bytes(b"04", 32).unwrap().to_bytes_be();
//         marker_be.append(&mut x_be);
//         marker_be.append(&mut y_be);
//         marker_be
//     }

//     pub fn compressed_sec(&self) -> Vec<u8> {
//         let (x, y) = self.xy().unwrap();
//         let mut marker = b"03";
//         if y.modpow(&BigUint::from(1u32), &BigUint::from(2u32)) == BigUint::ZERO {
//             marker = b"02";
//         }
//         let mut marker_be = BigUint::parse_bytes(marker, 32).unwrap().to_bytes_be();
//         let mut x_be = x.to_bytes_be();
//         marker_be.append(&mut x_be);
//         marker_be
//     }

//     pub fn parse(&self, sec_bin: &[u8]) -> Self {
//         if sec_bin[0] == 4u8 {
//             let x = BigUint::from_bytes_be(&sec_bin[1..33]);
//             let y = BigUint::from_bytes_be(&sec_bin[33..]);
//             return S256Point::new(CoordsS256::Finite(S256Field::new(x), S256Field::new(y)));
//         }

//         let is_even = sec_bin[0] == 2u8;
//         let x = S256Field::new(BigUint::from_bytes_be(&sec_bin[1..]));
//         let alpha = S256Field::from_S256_field(x.pow(&BigInt::from(3)) + B.clone());
//         let beta = alpha.sqrt();
//         let mut even_beta = beta.clone();
//         let mut odd_beta = S256Field::new(P.clone() - beta.num());
//         if beta.num().modpow(&BigUint::from(1u32), &BigUint::from(2u32)) != BigUint::ZERO {
//             even_beta = S256Field::new(P.clone() - beta.num());
//             odd_beta = beta;
//         }
//         if is_even {
//             S256Point::new(CoordsS256::Finite(x, even_beta))
//         } else {
//             S256Point::new(CoordsS256::Finite(x, odd_beta))
//         }
//     }

//     pub fn address(&self, compressed: bool, testnet: bool) -> String {
//         let mut h160= if compressed {
//             hash160(&self.compressed_sec())
//         } else {
//             hash160(&self.uncompressed_sec())
//         };

//         let prefix = if testnet {
//             hex::decode("6f").unwrap()[0]
//         } else {
//             hex::decode("00").unwrap()[0]
//         };

//         h160.insert(0, prefix);
//         encode_base58_checksum(&h160)
//     }
// }

// impl<'a> Mul<&'a BigUint> for &'a S256Point<'a> {
//     type Output = S256Point<'a>;

//     fn mul(self, other: &'a BigUint) -> S256Point<'a> {
//         let one = BigUint::from(1u32);
//         let coef = other.modpow(&one, &N_S256);
//         let point = &self.0 * coef;
//         S256Point(point)
//     }   
// }

// impl<'a> Add<&'a S256Point<'a>> for &'a S256Point<'_> {
//     type Output = S256Point<'a>;

//     fn add(self, other: &'a S256Point) -> S256Point<'a> {
//         let p = &self.0 + &other.0;
//         match p.xy {
//             Coords::Infinity => S256Point::new(CoordsS256::Infinity),
//             Coords::Finite(x, y) => 
//                 S256Point::new(CoordsS256::Finite(S256Field::new(x.num), S256Field::new(y.num)))
//         }
//     }
// }

pub type A = FieldElement;
pub type B = FieldElement;
pub type X = FieldElement;
pub type Y = FieldElement;
pub type Curve = EllipticCurve;

// pub type Curve = dyn Fn(A, B);

#[derive(Debug, PartialEq, Clone)]
enum XY {
    Infinity,
    Finite{ x: X, y: Y },
}

// pub trait Pointe {}

// pub struct Infinity;

// impl Pointe for Infinity {}
// pub struct Finite {
//     x: X,
//     y: Y,
// }

// impl Pointe for Finite {}

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    a: A,
    b: B,
}

impl EllipticCurve {
    pub fn new(a: A, b: B) -> Self {
        EllipticCurve { a, b }
    }

    pub fn a(&self) -> &A {
        &self.a
    }

    pub fn b(&self) -> &B {
        &self.b
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    xy: XY,
    curve: Rc<EllipticCurve>,
}

impl Point {
    pub fn from(curve: Rc<Curve>, x: X, y: Y) -> Result<Point, &'static str> {
        let left = y.pow(&BigInt::from(2));
        let right = (&(&x.pow(&BigInt::from(3)) + &(&x * &curve.a)?)? + &curve.b)?;
        if left != right {
            return Err("(x, y) is not on the curve");
        };

        Ok(Point{ xy: XY::Finite { x, y }, curve: Rc::clone(&curve)})
    }

    pub fn at_inifity(curve: Rc<Curve>) -> Result<Point, &'static str> {
        Ok(Point { xy: XY::Infinity, curve: Rc::clone(&curve)})
    }

    fn add(curve: Rc<Curve>, p1: &Point, p2: &Point) -> Result<Point, &'static str> {
        let exp = BigInt::from(2);

        let (x1, y1) = match &p1.xy {
            XY::Finite { x, y } => (x, y),
            XY::Infinity => return Ok(p2.clone()),
        };

        let (x2, y2) = match &p2.xy {
            XY::Finite { x, y} => (x, y),
            XY::Infinity => return Ok(p1.clone()),
        };

        if x1 == x2 && y1 != y2 {
            Ok(Point::at_inifity(Rc::clone(&curve)))?
        } else {
            if y1 == &(&FieldElement::new(BigUint::ZERO, y1.prime()) * x1)? {
                return Ok(Point::at_inifity(Rc::clone(&curve)))?;
            } 
            if x1 == x2 && y1 == y2 {
                let field_elem_2 = FieldElement::new(BigUint::from(2u32), Rc::clone(&x1.prime()));
                let field_elem_3 = FieldElement::new(BigUint::from(3u32), Rc::clone(&x1.prime()));
                let s = (&(&(&field_elem_3 * &x1.pow(&exp))? + &curve.a)? / &(&field_elem_2 * y1)?)?;
                let x = (&s.pow(&BigInt::from(2)) - &(&field_elem_2 * x1)?)?;
                let y = (&(&s * &(x1 - &x)?)? - y1)?;
                Ok(Point::from(Rc::clone(&curve), x, y))?
            } else {
                let s = (&(y2 - y1)? / &(x2 - x1)?)?;
                let x = (&(&s.pow(&exp) - x1)? - x2)?;
                let y = (&(&s * &(x1 - &x)?)? - y1)?;
                Ok(Point::from(Rc::clone(&curve), x, y))?
            }   
        }
    }
}

// impl Point {
//     fn new(xy: XY, curve: Rc<EllipticCurve>) -> Self {
//         if let XY::Finite{x, y} = xy {
//             if y.pow(&BigInt::from(2)) != &(x.pow(&BigInt::from(3)) + x*a) + &b {
//                     panic!("({:#?}, {:#?}) is not on the curve", x, y);     
//             };
//         };

//         Point { xy, a, b }
//     }

//     fn at_inifity(a: Rc<FieldElement>, b: Rc<FieldElement>) -> Self {
//         Point::new(XY::Infinity, a, b)
//     }

//     fn from_slope(x1: Rc<FieldElement>, y1: Rc<FieldElement>, x2: Rc<FieldElement>, y2: Rc<FieldElement>, a: Rc<FieldElement>, b: Rc<FieldElement>) -> Self {
//         let exp = BigInt::from(2);
//         if x1 == x2 && y1 == y2 {
//             let field_elem_2 = FieldElement::new(BigUint::from(2u32), Rc::clone(x1.prime()));
//             let field_elem_3 = FieldElement::new(BigUint::from(3u32), Rc::clone(x1.prime()));
//             let s = (field_elem_3 * x1.pow(&exp) + Rc::clone(&a)) / (field_elem_2 * *y1);
//             let x = s.pow(&BigInt::from(2)) - field_elem_2 * *x1;
//             let y = s.clone() * (*x1 - x) - *y1;
//             Point {xy: XY::Finite{x: Rc::new(x), y: Rc::new(y)}, a, b}
//         } else {
//            let s = (*y2 - *y1) / (*x2 - *x1);
//            let x = s.pow(&exp) - *x1 - *x2;
//            let y = (s * (*x1 - x)) - *y1;
//            Point {xy: XY::Finite{x: Rc::new(x), y: Rc::new(y)}, a, b}
//         }
//     }
// }



impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy && self.curve.a() == other.curve.a() && self.curve.b() == other.curve.b()
    }
}

// impl Add for Point {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         if self.a != other.a || self.b != other.b {
//             panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
//         }

//         match self.xy {
//             XY::Infinity => other,
//             XY::Finite{x: x1, y: y1} => {
//                 match other.xy {
//                     XY::Infinity => self,
//                     XY::Finite{x: x2, y: y2} => {
//                         if x1 == x2 && y1 != y2 {
//                             Point::at_inifity(self.a, self.b)
//                         } else {
//                             if y1 == &(&FieldElement::new(BigUint::ZERO, y1.prime) * x1) {
//                                 return Point::at_inifity(self.a, self.b);
//                             }
//                             Point::from_slope(x1, y1, x2, y2, self.a, self.b)
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

impl<'a> Add<&'a Point> for &'a Point {
    type Output = Result<Point, &'static str>;

    fn add(self, other: &'a Point) -> Self::Output {
        if self.curve.a() != other.curve.a() || self.curve.b() != other.curve.b() {
            return Err("Points are not in the same curve");
        }

        Point::add(Rc::clone(&self.curve), self,other)
        // match self.xy {
        //     XY::Infinity => other.clone(),
        //     XY::Finite{x: x1, y: y1} => {
        //         match other.xy {
        //             XY::Infinity => self.clone(),
        //             XY::Finite{x: x2, y: y2} => {
        //                 if x1 == x2 && y1 != y2 {
        //                     Point::at_inifity(self.a, self.b)
        //                 } else {
        //                     if *y1 == FieldElement::new(BigUint::ZERO, y1.prime) * x1 {
        //                         return Point::at_inifity(self.a, self.b);
        //                     }
        //                     Point::from_slope(x1, y1, x2, y2, self.a, self.b)
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

// impl<'a> Add<Point> for &'a Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         if self.a != other.a || self.b != other.b {
//             panic!("Points {:#?} and {:#?} are not in the same curve", self, other);
//         }

//         match self.xy {
//             XY::Infinity => other,
//             XY::Finite{x: x1, y: y1} => {
//                 match other.xy {
//                     XY::Infinity => self.clone(),
//                     XY::Finite{x: x2, y: y2} => {
//                         if x1 == x2 && y1 != y2 {
//                             Point::at_inifity(self.a, self.b)
//                         } else {
//                             if *y1 == FieldElement::new(BigUint::ZERO, y1.prime) * x1 {
//                                 return Point::at_inifity(self.a, self.b);
//                             }
//                             Point::from_slope(x1, y1, x2, y2, self.a, self.b)
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

impl Mul<BigUint> for &Point {
    type Output = Result<Point, &'static str>;

    // fn mul(self, other: u32) -> Point {
        
    //     let mut prod = Point::new(XY::Infinity, self.a, self.b);
    //     for _ in 0..other {
    //         prod = self + prod;
    //     };
    //     prod
    // }   
    fn mul(self, other: BigUint) -> Self::Output {
        let mut result = Point::at_inifity(Rc::clone(&self.curve))?;
        let mut current = self.clone();
        let mut coef = other;
        let zero = BigUint::ZERO;
        let one = BigUint::from(1u32);
        while coef > zero {
            let current1 = current.clone();
            let current2 = current;
            if coef.clone().bitand(&one) == one {
                result = (&result + &current1)?;
            }
            // if coef.clone().bitand(&one) == one {
            //     result = match &result + &current1 {
            //         Ok(point) => point,
            //         Err(err) => return Err(err),
            //     }
            // }
            current = (&current1 + &current2)?;
            coef >>= 1u32;
        };

        Ok(result)
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_points() {
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));
        
        let x1 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let point = Point::from(Rc::clone(&curve), x1, y1);
        assert!(point.is_ok());

        let x2 = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(56u32), Rc::clone(&prime));
        let point = Point::from(Rc::clone(&curve), x2, y2);
        assert!(point.is_ok());

        let x3 = FieldElement::new(BigUint::from(1u32), Rc::clone(&prime));
        let y3 = FieldElement::new(BigUint::from(193u32), Rc::clone(&prime));
        let point = Point::from(Rc::clone(&curve), x3, y3);
        assert!(point.is_ok());
    }

    #[test]
    fn test_new_invalid_points() {
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));
        
        let x1 = FieldElement::new(BigUint::from(200u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(119u32), Rc::clone(&prime));
        let point = Point::from(Rc::clone(&curve), x1, y1);
        assert!(point.is_err());

        let x2 = FieldElement::new(BigUint::from(42u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(99u32), Rc::clone(&prime));
        let point = Point::from(Rc::clone(&curve), x2, y2);
        assert!(point.is_err());
    }

    #[test]
    fn test_eq() {
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));
        
        let x1 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1);

        let x2 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2);

        let x3 = FieldElement::new(BigUint::from(1u32), Rc::clone(&prime));
        let y3 = FieldElement::new(BigUint::from(193u32), Rc::clone(&prime));
        let p3 = Point::from(Rc::clone(&curve), x3, y3);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_add_inf() {
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));

        let p_inf = Point::at_inifity(Rc::clone(&curve)).unwrap();
        
        let x1 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        assert_eq!((&p_inf + &p1).unwrap(), p1.clone());
        assert_eq!((&p1 + &p_inf).unwrap(), p1.clone());
        assert_eq!((&p_inf + &p_inf).unwrap(), p_inf.clone());

        assert_eq!((&p_inf + &p1).unwrap(), p1.clone());
        assert_eq!((&p1 + &p_inf).unwrap(), p1.clone());
        assert_eq!((&p_inf + &p_inf).unwrap(), p_inf);
    }

    #[test]
    fn test_add() {
        // (192, 105, 17, 56, 170, 142)
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));
        
        let x1 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(17u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(56u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let x3 = FieldElement::new(BigUint::from(170u32), Rc::clone(&prime));
        let y3 = FieldElement::new(BigUint::from(142u32), Rc::clone(&prime));
        let p3 = Point::from(Rc::clone(&curve), x3, y3).unwrap();

        assert_eq!((&p1 + &p2).unwrap(), p3);

        // (47, 71, 117, 141, 60, 139)
        let x1 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(117u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(141u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let x3 = FieldElement::new(BigUint::from(60u32), Rc::clone(&prime));
        let y3 = FieldElement::new(BigUint::from(139u32), Rc::clone(&prime));
        let p3 = Point::from(Rc::clone(&curve), x3, y3).unwrap();

        assert_eq!((&p1 + &p2).unwrap(), p3);

        // (143, 98, 76, 66, 47, 71)
        let x1 = FieldElement::new(BigUint::from(143u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(98u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(76u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(66u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let x3 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y3 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p3 = Point::from(Rc::clone(&curve), x3, y3).unwrap();

        assert_eq!((&p1 + &p2).unwrap(), p3);
    }

    #[test]
    fn test_mul() {
        // (2, 192, 105, 49, 71)
        let prime = Rc::new(BigUint::from(223u32));
        let a = FieldElement::new(BigUint::ZERO, Rc::clone(&prime));
        let b = FieldElement::new(BigUint::from(7u32), Rc::clone(&prime));
        let curve = Rc::new(EllipticCurve::new(a, b));
        
        let x1 = FieldElement::new(BigUint::from(192u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(105u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(49u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let scalar = BigUint::from(2u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

        // (2, 143, 98, 64, 168)
        let x1 = FieldElement::new(BigUint::from(143u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(98u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(64u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(168u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let scalar = BigUint::from(2u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

        // (2, 47, 71, 36, 111)
        let x1 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(36u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(111u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let scalar = BigUint::from(2u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

        // (4, 47, 71, 194, 51)
        let x1 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(194u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(51u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let scalar = BigUint::from(4u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

        // (8, 47, 71, 116, 55)
        let x1 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let x2 = FieldElement::new(BigUint::from(116u32), Rc::clone(&prime));
        let y2 = FieldElement::new(BigUint::from(55u32), Rc::clone(&prime));
        let p2 = Point::from(Rc::clone(&curve), x2, y2).unwrap();

        let scalar = BigUint::from(8u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

        // (21, 47, 71, None, None)
        let x1 = FieldElement::new(BigUint::from(47u32), Rc::clone(&prime));
        let y1 = FieldElement::new(BigUint::from(71u32), Rc::clone(&prime));
        let p1 = Point::from(Rc::clone(&curve), x1, y1).unwrap();

        let p2 = Point::at_inifity(Rc::clone(&curve)).unwrap();

        let scalar = BigUint::from(21u32);

        assert_eq!((&p1 * scalar).unwrap(), p2);

    }

    // #[test]
    // fn test_s256_point_generator() {
    //     let g = S256Point::generator();
    //     let p = &g * &N_S256;
    //     assert_eq!(p.0.xy, XY::Infinity);
    // }

    // #[test]
    // fn test_s256_verify() {
    //     let x = BigUint::from_bytes_be(&hex::decode("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c").unwrap());
    //     let y = BigUint::from_bytes_be(&hex::decode("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
    //     let point = S256Point::new(CoordsS256::Finite(S256Field::new(x), S256Field::new(y)));
    //     let z = BigUint::from_bytes_be(&hex::decode("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60").unwrap());
    //     let r = BigUint::from_bytes_be(&hex::decode("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395").unwrap());
    //     let s = BigUint::from_bytes_be(&hex::decode("068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4").unwrap());
    //     let sig = Signature::new(r, s);
    //     assert!(point.verify(&z, &sig));
    //     let z = BigUint::from_bytes_be(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
    //     let r = BigUint::from_bytes_be(&hex::decode("00eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap());
    //     let s = BigUint::from_bytes_be(&hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
    //     let sig = Signature::new(r, s);
    //     assert!(point.verify(&z, &sig));
    // }

    // #[test]
    // fn test_s256_sec() {
    //     let mut uncompressed = "049d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d56fa15cc7f3d38cda98dee2419f415b7513dde1301f8643cd9245aea7f3f911f9";
    //     let mut compressed = "039d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d5";
    //     let g = S256Point::generator();
    //     let mut coef = BigUint::from(999u32).pow(3);
    //     let mut point = &g * &coef;
    //     assert_eq!(point.uncompressed_sec(), hex::decode(uncompressed).unwrap());
    //     assert_eq!(point.compressed_sec(), hex::decode(compressed).unwrap());

    //     coef = BigUint::from(123u32);
    //     uncompressed = "04a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5204b5d6f84822c307e4b4a7140737aec23fc63b65b35f86a10026dbd2d864e6b";
    //     compressed = "03a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5";
    //     point = &g * &coef;
    //     assert_eq!(point.uncompressed_sec(), hex::decode(uncompressed).unwrap());
    //     assert_eq!(point.compressed_sec(), hex::decode(compressed).unwrap());

    //     coef = BigUint::from(42424242u32);
    //     uncompressed = "04aee2e7d843f7430097859e2bc603abcc3274ff8169c1a469fee0f20614066f8e21ec53f40efac47ac1c5211b2123527e0e9b57ede790c4da1e72c91fb7da54a3";
    //     compressed = "03aee2e7d843f7430097859e2bc603abcc3274ff8169c1a469fee0f20614066f8e";
    //     point = &g * &coef;
    //     assert_eq!(point.uncompressed_sec(), hex::decode(uncompressed).unwrap());
    //     assert_eq!(point.compressed_sec(), hex::decode(compressed).unwrap());
    // }

    // #[test]
    // fn test_s256_parse() {
    //     let g = S256Point::generator();
    //     let mut coef = BigUint::from(999u32).pow(3);
    //     let mut point = &g * &coef;
    //     let mut sec_bin = point.uncompressed_sec();
    //     let mut parsed_point = point.parse(&sec_bin);
    //     assert_eq!(parsed_point.0, point.0);

    //     coef = BigUint::from(123u32);
    //     point = &g * &coef;
    //     sec_bin = point.compressed_sec();
    //     parsed_point = point.parse(&sec_bin);
    //     assert_eq!(parsed_point.0, point.0);
        
    // }

    // #[test]
    // fn test_s256_address() {
    //     let g = S256Point::generator();
    //     let mut secret = BigUint::from(888u32).pow(3);
    //     let mut point = &g * &secret;
    //     let mut mainnet_address = "148dY81A9BmdpMhvYEVznrM45kWN32vSCN";
    //     let mut testnet_address = "mieaqB68xDCtbUBYFoUNcmZNwk74xcBfTP";
    //     assert_eq!(point.address(true, false), mainnet_address);
    //     assert_eq!(point.address(true, true), testnet_address);

    //     secret = BigUint::from(321u32);
    //     point = &g * &secret;
    //     mainnet_address = "1S6g2xBJSED7Qr9CYZib5f4PYVhHZiVfj";
    //     testnet_address = "mfx3y63A7TfTtXKkv7Y6QzsPFY6QCBCXiP";
    //     assert_eq!(point.address(false, false), mainnet_address);
    //     assert_eq!(point.address(false, true), testnet_address);

    //     secret = BigUint::from(4242424242u32);
    //     point = &g * &secret;
    //     mainnet_address = "1226JSptcStqn4Yq9aAmNXdwdc2ixuH9nb";
    //     testnet_address = "mgY3bVusRUL6ZB2Ss999CSrGVbdRwVpM8s";
    //     assert_eq!(point.address(false, false), mainnet_address);
    //     assert_eq!(point.address(false, true), testnet_address);
    // }
}