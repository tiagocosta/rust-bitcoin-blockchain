mod finite_field;

use finite_field::FiniteElement;   

fn main() {
    let a = FiniteElement::new(7, 13);
    let b = FiniteElement::new(6, 13);
    println!("{}", a == b);
    println!("{}", a != b);

    let c = FiniteElement::new(7, 13);
    let d = FiniteElement::new(12, 13);
    let e = FiniteElement::new(6, 13);
    println!("{}", c + d == e);

    let f = FiniteElement::new(3, 13);
    let g = FiniteElement::new(12, 13);
    let h = FiniteElement::new(10, 13);
    println!("{}", f * g == h);

    let i = FiniteElement::new(3, 13);
    let j = FiniteElement::new(1, 13);
    println!("{}", i.pow(3) == j);
}
