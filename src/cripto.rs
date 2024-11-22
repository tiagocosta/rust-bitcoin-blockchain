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
}