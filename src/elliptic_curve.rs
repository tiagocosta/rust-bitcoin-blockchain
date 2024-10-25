use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Coord {
    Infinity,
    Finite(BigInt),
}

#[derive(Debug)]
pub struct Point {
    x: Coord,
    y: Coord,
    a: i32,
    b: i32,
}

impl Point {
    pub fn new(x: Coord, y: Coord, a: i32, b: i32) -> Self {
        if let Coord::Finite(x_coord) = &x {
            if let Coord::Finite(y_coord) = &y {
                if y_coord.pow(2) != x_coord.pow(3) + a*x_coord + b {
                    panic!("({}, {}) is not on the curve", x_coord, y_coord);     
                }
            };
        };

        Point { x, y, a, b }
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
        Point::new(Coord::Finite(x), Coord::Finite(y), 5, 7);
    }

    #[test]
    fn test_new_valid_point() {
        let a = 5;
        let b = 7;
        
        let x1 = BigInt::from(-1);
        let y1 = BigInt::from(-1);
        let p1 = Point::new(Coord::Finite(x1), Coord::Finite(y1), a, b);
        assert_eq!(p1.x, Coord::Finite(BigInt::from(-1)));
        assert_eq!(p1.y, Coord::Finite(BigInt::from(-1)));
        
        let p2 = Point::new(Coord::Infinity, Coord::Infinity, a, b);
        assert_eq!(p2.x, Coord::Infinity);
        assert_eq!(p2.y, Coord::Infinity);
    }
}