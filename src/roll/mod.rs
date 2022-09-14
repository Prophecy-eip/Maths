//! # Roll module
//!
//! This module contain all the functions to obtain the values of rolls

/// Compute the value to hit the opponent
///
/// # Parameters
/// offensive (usize): The offensive Stats of the attacking Regiment
///
/// defense (usize): The defense Stats of the defending Regiment
pub fn compute_roll_to_hit(offensive: usize, defense: usize) -> usize {
    let difference: i8 = offensive as i8 - defense as i8;

    match difference {
        i8::MIN..=-8 => 6,
        -7..=-4 => 5,
        -3..=0 => 4,
        1..=3 => 3,
        4..=i8::MAX => 2,
    }
}

/// Compute the value to wound the opponent
///
/// # Parameters
/// strength (usize): The strength Stats of the attacking Regiment
///
/// resilience (usize): The resilience Stats of the defending Regiment
pub fn compute_roll_to_wound(strength: usize, resilience: usize) -> usize {
    let difference: i8 = strength as i8 - resilience as i8;

    match difference {
        i8::MIN..=-2 => 6,
        -1 => 5,
        0 => 4,
        1 => 3,
        2..=i8::MAX => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_roll_to_hit, compute_roll_to_wound};

    #[test]
    fn test_hit_1() {
        assert_eq!(compute_roll_to_hit(1, 1), 4);
    }

    #[test]
    fn test_hit_2() {
        assert_eq!(compute_roll_to_hit(1, 5), 5);
    }

    #[test]
    fn test_hit_3() {
        assert_eq!(compute_roll_to_hit(1, 9), 6);
    }

    #[test]
    fn test_hit_4() {
        assert_eq!(compute_roll_to_hit(6, 4), 3);
    }

    #[test]
    fn test_hit_5() {
        assert_eq!(compute_roll_to_hit(8, 3), 2);
    }

    #[test]
    fn test_wound_1() {
        assert_eq!(compute_roll_to_wound(1, 1), 4);
    }

    #[test]
    fn test_wound_2() {
        assert_eq!(compute_roll_to_wound(1, 2), 5);
    }

    #[test]
    fn test_wound_3() {
        assert_eq!(compute_roll_to_wound(1, 3), 6);
    }

    #[test]
    fn test_wound_4() {
        assert_eq!(compute_roll_to_wound(3, 2), 3);
    }

    #[test]
    fn test_wound_5() {
        assert_eq!(compute_roll_to_wound(4, 2), 2);
    }
}
