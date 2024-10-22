mod finite_field;

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
}
