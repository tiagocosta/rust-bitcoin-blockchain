mod finite_field;
mod elliptic_curve;

use finite_field::FiniteElement;

use num_bigint::{BigUint, BigInt};


fn main() {
    let a = FiniteElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let b = FiniteElement::new(BigUint::from(6u32), BigUint::from(13u32));
    println!("{}", a == b);
    println!("{}", a != b);

    let c = FiniteElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let d = FiniteElement::new(BigUint::from(12u32), BigUint::from(13u32));
    let e = FiniteElement::new(BigUint::from(6u32), BigUint::from(13u32));
    println!("{}", c + d == e);

    let f = FiniteElement::new(BigUint::from(3u32), BigUint::from(13u32));
    let g = FiniteElement::new(BigUint::from(12u32), BigUint::from(13u32));
    let h = FiniteElement::new(BigUint::from(10u32), BigUint::from(13u32));
    println!("{}", f * g == h);

    let i = FiniteElement::new(BigUint::from(3u32), BigUint::from(13u32));
    let j = FiniteElement::new(BigUint::from(1u32), BigUint::from(13u32));
    println!("{}", i.pow(&BigInt::from(3)) == j);

    let k = FiniteElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let l = FiniteElement::new(BigUint::from(8u32), BigUint::from(13u32));
    println!("{}", k.pow(&BigInt::from(-3)) == l);

    // let a = 5;
    // let b = 7;

    // let x1 = BigInt::from(-1);
    // let y1 = BigInt::from(-1);
    // let p1 = Point::new(Coords::Finite(x1, y1), a, b);
    // println!("{:?}", p1);

    // // let x2 = BigInt::from(-1);
    // // let y2 = BigInt::from(-2);
    // // let p2 = Point::new(Coords::Finite(x2, y2), a, b);
    // // println!("{:?}", p2);

    // let p3 = Point::new(Coords::Infinity, a, b);
    // println!("{:#?}", p3);
}
