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
}