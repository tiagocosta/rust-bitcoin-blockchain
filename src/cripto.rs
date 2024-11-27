use num_bigint::{BigUint, RandBigInt};


use crate::elliptic_curve::{S256Point, N_S256};

pub struct Signature {
    pub r: BigUint,
    pub s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Signature { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        let mut rbin = self.r.to_bytes_be();
        if rbin[0] >= hex::decode("80").unwrap()[0] {
            rbin.insert(0, hex::decode("00").unwrap()[0]);
        }
        
        rbin.insert(0, rbin.len() as u8);
        rbin.insert(0, 2);
        
        let mut sbin = self.s.to_bytes_be();
        if sbin[0] >= hex::decode("80").unwrap()[0] {
            sbin.insert(0, hex::decode("00").unwrap()[0]);
        }
        sbin.insert(0, sbin.len() as u8);
        sbin.insert(0, 2);

        let mut result= rbin;
        result.append(&mut sbin);
        result.insert(0, result.len() as u8);
        result.insert(0, hex::decode("30").unwrap()[0]);
        result
    }
}

pub struct PrivateKey {
    secret: BigUint,
}

impl PrivateKey {

    pub fn new(secret: BigUint) -> Self {
        PrivateKey { secret }
    }

    pub fn sign(&self, z: &BigUint) -> Signature {
        let mut rng = rand::thread_rng();
        let k = rng.gen_biguint_below(&N_S256);
        let one = BigUint::from(1u32);
        let g = S256Point::generator();
        let r_point = &g * &k;
        let r = match r_point.xy() {
            Some((x, _)) => x,
            None => panic!("infinity"),
        };
        let k_inv = &k.modinv(&N_S256).unwrap();
        let mut s = ((z + r*&self.secret) * k_inv).modpow(&one, &N_S256);
        if s > N_S256.clone()/BigUint::from(2u32) {
            s = N_S256.clone() - s;
        }
        Signature::new(r.clone(), s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let mut rng = rand::thread_rng();
        let secret = rng.gen_biguint_below(&N_S256);
        let pk = PrivateKey::new(secret.clone());
        let z = rng.gen_biguint_below(&BigUint::from(2u32).pow(256u32));
        let sig = pk.sign(&z);
        let g = S256Point::generator();
        let point = &g * &secret;
        assert!(point.verify(&z, &sig))
    }

    #[test]
    fn test_der() {
        let r = BigUint::from_bytes_be(&hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap());
        let s = BigUint::from_bytes_be(&hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap());
        let sig = Signature::new(r, s);
        assert_eq!(
            "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            hex::encode(sig.der())
        );
    }
}