use num_bigint::{BigUint, RandBigInt};
use sha2::{Digest, Sha256};


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

    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let prefix = if testnet {
            hex::decode("ef").unwrap()[0]
        } else {
            hex::decode("80").unwrap()[0]
        };

        let mut secret_bytes = self.secret.to_bytes_be();
        secret_bytes.insert(0, prefix);
        
        if compressed {
            let sufix = hex::decode("01").unwrap()[0];
            secret_bytes.push(sufix);
        }

        encode_base58_checksum(&secret_bytes)
    }
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    let mut sha_hasher = Sha256::new();
    sha_hasher.update(bytes);
    let res_1 = sha_hasher.finalize();

    let mut ripemd_hasher = ripemd::Ripemd160::new();
    ripemd_hasher.update(res_1);
    let res = ripemd_hasher.finalize();
    
    res.to_vec()
}

pub fn hash256(bytes: &[u8]) -> Vec<u8> {
    let mut sha_hasher_1 = Sha256::new();
    sha_hasher_1.update(bytes);
    let res_1 = sha_hasher_1.finalize();

    let mut sha_hasher_2 = Sha256::new();
    sha_hasher_2.update(res_1);
    let res_2 = sha_hasher_2.finalize();
    
    res_2.to_vec()
}

pub fn encode_base58_checksum(bytes: &[u8]) -> String {
    let checksum = &hash256(bytes)[..4];
    let mut to_be_encoded = bytes.to_vec();
    to_be_encoded.append(&mut checksum.to_vec());
    bs58::encode(to_be_encoded).into_string()
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

    #[test]
    fn test_hash160() {
        let hash_160 = hash160(b"my secret");
        let hash160_my_secret = [144, 228, 193, 75, 235, 6, 103, 99, 58, 142, 47, 92, 110, 148, 240, 140, 171, 139, 187, 93];
        assert_eq!(hash_160, hash160_my_secret);
    }

    #[test]
    fn test_wif() {
        let mut pk = PrivateKey::new(BigUint::from(2u32).pow(256) - BigUint::from(2u32).pow(199));
        let mut expected = "L5oLkpV3aqBJ4BgssVAsax1iRa77G5CVYnv9adQ6Z87te7TyUdSC";
        assert_eq!(pk.wif(true, false), expected);

        pk = PrivateKey::new(BigUint::from(2u32).pow(256) - BigUint::from(2u32).pow(201));
        expected = "93XfLeifX7Jx7n7ELGMAf1SUR6f9kgQs8Xke8WStMwUtrDucMzn";
        assert_eq!(pk.wif(false, true), expected);

        pk = PrivateKey::new(BigUint::from(2u32).pow(256) - BigUint::from(2u32).pow(201));
        expected = "93XfLeifX7Jx7n7ELGMAf1SUR6f9kgQs8Xke8WStMwUtrDucMzn";
        assert_eq!(pk.wif(false, true), expected);

        pk = PrivateKey::new(BigUint::from_bytes_be(
            &hex::decode("0dba685b4511dbd3d368e5c4358a1277de9486447af7b3604a69b8d9d8b7889d").unwrap())
        );
        expected = "5HvLFPDVgFZRK9cd4C5jcWki5Skz6fmKqi1GQJf5ZoMofid2Dty";
        assert_eq!(pk.wif(false, false), expected);

        pk = PrivateKey::new(
            BigUint::from_bytes_be(&hex::decode("1cca23de92fd1862fb5b76e5f4f50eb082165e5191e116c18ed1a6b24be6a53f").unwrap())
        );
        expected = "cNYfWuhDpbNM1JWc3c6JTrtrFVxU4AGhUKgw5f93NP2QaBqmxKkg";
        assert_eq!(pk.wif(true, true), expected);
    }
}