mod finite_field;
mod elliptic_curve;
mod cripto;

use elliptic_curve::{S256Point, N_S256};
use num_bigint::BigUint;
use sha2::{Sha256, Digest};

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
    // println!("{:#?}", &S256Point::generator() * &N_S256);
    // println!("{:?}", hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap());


    // use sha2::{Sha256, Digest};

    // let mut hasher = Sha256::new();
    // let data = b"hello world!";
    // hasher.update(data);
    // // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    // // hasher.update("String data");
    // // Note that calling `finalize()` consumes hasher
    // let hash = hasher.finalize();
    // println!("Binary hash: {:?}", hash[..]);

    let e = BigUint::from_bytes_be(&Sha256::digest(Sha256::digest(b"my secret")));
    let z = BigUint::from_bytes_be(&Sha256::digest(Sha256::digest(b"my message")));
    let k = BigUint::from(1234567890u32);
    // let k = BigUint::from(1u32);

    let g = S256Point::generator();

    let p = &g * &k;

    let r = p.xy().unwrap().0;

    let k_inv = k.modinv(&N_S256).unwrap();
    // println!("{:#?}", hex::decode(hash).unwrap());
    // println!("{}", k_inv);
    let one = BigUint::from(1u32);
    let s = ((&z + r*&e) * k_inv).modpow(&one, &N_S256);

    let point = &g * &e;

    // println!("{:#?}", point);
    println!("{}", hex::encode(point.xy().unwrap().0.to_bytes_be()));
    println!("{}", hex::encode(point.xy().unwrap().1.to_bytes_be()));
    println!("{}", hex::encode(z.to_bytes_be()));
    println!("{}", hex::encode(r.to_bytes_be()));
    // println!("{}", hex::encode(s.to_bytes_be()));

    let coef = BigUint::from(999u32).pow(3);
    let point = &g * &coef;
    println!("{:#?}", hex::encode(point.uncompressed_sec()));
    println!("{:#?}", hex::encode(point.compressed_sec()));
}
