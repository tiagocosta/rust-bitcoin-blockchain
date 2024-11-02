mod finite_field;
mod elliptic_curve;

use finite_field::FieldElement;

use elliptic_curve::{S256Point, N_S256};

use num_bigint::{BigUint, BigInt};

fn main() {
    // let prime = BigUint::from(13u32);
    // let a = FieldElement::new(BigUint::from(7u32), &prime);
    // let b = FieldElement::new(BigUint::from(6u32), &prime);
    // println!("{}", a == b);
    // println!("{}", a != b);

    // let c = FieldElement::new(BigUint::from(7u32), &prime);
    // let d = FieldElement::new(BigUint::from(12u32), &prime);
    // let e = FieldElement::new(BigUint::from(6u32), &prime);
    // println!("{}", c + d == e);

    // let f = FieldElement::new(BigUint::from(3u32), &prime);
    // let g = FieldElement::new(BigUint::from(12u32), &prime);
    // let h = FieldElement::new(BigUint::from(10u32), &prime);
    // println!("{}", f * g == h);

    // let i = FieldElement::new(BigUint::from(3u32), &prime);
    // let j = FieldElement::new(BigUint::from(1u32), &prime);
    // println!("{}", i.pow(&BigInt::from(3)) == j);

    // let k = FieldElement::new(BigUint::from(7u32), &prime);
    // let l = FieldElement::new(BigUint::from(8u32), &prime);
    // println!("{}", k.pow(&BigInt::from(-3)) == l);

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
    
    // let x_1 = S256Field::new(BigUint::from(47u32));
    // let y_1 = S256Field::new(BigUint::from(71u32));

    // let p =  S256Point::new(CoordsS256::Finite(x_1, y_1));

    // for i in 1..=20 {
    //     let res = &p * i;
    //     println!("{} * (47, 71) = {:#?}", i, res);

    // }

    // println!("{}", BigUint::from_bytes_be(&hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap()));
    println!("{:#?}", &S256Point::generator() * &N_S256);
    
}
