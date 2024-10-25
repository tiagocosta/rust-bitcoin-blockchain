use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Coords {
    Infinity,
    Finite(BigInt, BigInt),
}

#[derive(Debug)]
pub struct Point {
    xy: Coords,
    a: i32,
    b: i32,
}

impl Point {
    pub fn new(xy: Coords, a: i32, b: i32) -> Self {
        if let Coords::Finite(x, y) = &xy {
            if y.pow(2) != x.pow(3) + a*x + b {
                    panic!("({}, {}) is not on the curve", x, y);     
            };
        };

        Point { xy, a, b }
    }  
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.xy == other.xy && self.a == other.a && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new_invalid_point() {
        let x = BigInt::from(-1);
        let y = BigInt::from(-2);
        Point::new(Coords::Finite(x, y), 5, 7);
    }

    #[test]
    fn test_new_valid_point() {
        let a = 5;
        let b = 7;
        
        let x1 = BigInt::from(-1);
        let y1 = BigInt::from(-1);
        let p1 = Point::new(Coords::Finite(x1, y1), a, b);
        assert_eq!(p1.xy, Coords::Finite(BigInt::from(-1), BigInt::from(-1)));
        
        let p2 = Point::new(Coords::Infinity, a, b);
        assert_eq!(p2.xy, Coords::Infinity);
    }

    #[test]
    fn test_eq() {
        let a = 5;
        let b = 7;
        
        let x1 = BigInt::from(3);
        let y1 = BigInt::from(7);
        let p1 = Point::new(Coords::Finite(x1, y1), a, b);
        
        let x2 = BigInt::from(3);
        let y2 = BigInt::from(7);
        let p2 = Point::new(Coords::Finite(x2, y2), a, b);
        
        let x3 = BigInt::from(18);
        let y3 = BigInt::from(77);
        let p3 = Point::new(Coords::Finite(x3, y3), a, b);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }
}