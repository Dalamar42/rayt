use std::cmp::Ordering;

pub fn max(left: f64, right: f64) -> f64 {
    // If any value is NaN return the right value
    let ord = left.partial_cmp(&right).unwrap_or(Ordering::Less);
    match ord {
        Ordering::Less => right,
        Ordering::Greater => left,
        Ordering::Equal => right,
    }
}

pub fn min(left: f64, right: f64) -> f64 {
    // If any value is NaN return the right value
    let ord = left.partial_cmp(&right).unwrap_or(Ordering::Greater);
    match ord {
        Ordering::Less => left,
        Ordering::Greater => right,
        Ordering::Equal => right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_max() {
        assert_eq!(max(1.0, 2.0), 2.0);
        assert_eq!(max(2.0, 1.0), 2.0);
        assert_eq!(max(2.0, 2.0), 2.0);
        assert_eq!(max(std::f64::NAN, 2.0), 2.0);
        assert!(max(2.0, std::f64::NAN).is_nan());
        assert!(max(std::f64::NAN, std::f64::NAN).is_nan());
    }

    #[test]
    pub fn test_min() {
        assert_eq!(min(1.0, 2.0), 1.0);
        assert_eq!(min(2.0, 1.0), 1.0);
        assert_eq!(min(2.0, 2.0), 2.0);
        assert_eq!(min(std::f64::NAN, 2.0), 2.0);
        assert!(min(2.0, std::f64::NAN).is_nan());
        assert!(min(std::f64::NAN, std::f64::NAN).is_nan());
    }
}
