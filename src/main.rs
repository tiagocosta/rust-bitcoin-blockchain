mod finite_field;
mod elliptic_curve;

use elliptic_curve::{Coords, Point};
use finite_field::FieldElement;

use num_bigint::{BigUint, BigInt};


fn main() {
    let a = FieldElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let b = FieldElement::new(BigUint::from(6u32), BigUint::from(13u32));
    println!("{}", a == b);
    println!("{}", a != b);

    let c = FieldElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let d = FieldElement::new(BigUint::from(12u32), BigUint::from(13u32));
    let e = FieldElement::new(BigUint::from(6u32), BigUint::from(13u32));
    println!("{}", c + d == e);

    let f = FieldElement::new(BigUint::from(3u32), BigUint::from(13u32));
    let g = FieldElement::new(BigUint::from(12u32), BigUint::from(13u32));
    let h = FieldElement::new(BigUint::from(10u32), BigUint::from(13u32));
    println!("{}", f * g == h);

    let i = FieldElement::new(BigUint::from(3u32), BigUint::from(13u32));
    let j = FieldElement::new(BigUint::from(1u32), BigUint::from(13u32));
    println!("{}", i.pow(&BigInt::from(3)) == j);

    let k = FieldElement::new(BigUint::from(7u32), BigUint::from(13u32));
    let l = FieldElement::new(BigUint::from(8u32), BigUint::from(13u32));
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
    let prime = BigUint::from(223u32);
    let a = FieldElement::new(BigUint::ZERO, prime.clone());
    let b = FieldElement::new(BigUint::from(7u32), prime.clone());
    
    let x_1 = FieldElement::new(BigUint::from(47u32), prime.clone());
    let y_1 = FieldElement::new(BigUint::from(71u32), prime.clone());

    let p =  Point::new(Coords::Finite(x_1, y_1), &a, &b);

    for i in 1..=20 {
        let res = &p * i;
        println!("{} * (47, 71) = {:#?}", i, res);

    }

}
