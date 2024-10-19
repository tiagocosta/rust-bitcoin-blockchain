#[derive(Debug)]
pub struct FiniteElement {
    num: u32,
    prime: u32,
}

impl FiniteElement {
    pub fn new(num: u32, prime:u32) -> Self {
        FiniteElement { num, prime }
    }
}

impl PartialEq for FiniteElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = FiniteElement::new(2, 31);
        let b = FiniteElement::new(2, 31);
        let c = FiniteElement::new(15, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
