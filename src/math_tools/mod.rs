//! # Math Tools module
//!
//! This module contain some useful mathematic tools
//! Those tools will be used for our computations

use factorial::Factorial;

/// This is the math sigma function
/// It execute the given function on the given range and return the sum of the result of the function.
///
/// # Parameters
/// initial (isize): The first value of the range
///
/// max (isize): The last value of the range
///
/// function ((fn(isize, Option\<P>, isize, isize)) -> T): The function to execute on the range.
/// This function params are :
///
/// 1. The current value of the range
///
/// 2. The parameter of the function
///
/// 3. The initial value of the range
///
/// 4. The final value of the range
///
/// function_parameter (Option\<P>): A parameter that can be passed to the function
///
/// # Return
///
/// T: The result of our sigma computation
pub fn sigma<T: std::ops::AddAssign, P: Copy>(
    initial: isize,
    max: isize,
    function: fn(isize, Option<P>, isize, isize) -> T,
    function_parameter: Option<P>,
) -> T {
    let mut sum: T = function(initial, function_parameter, initial, max);
    for j in (initial + 1)..max {
        sum += function(j, function_parameter, initial, max);
    }
    sum
}

/// This function compute the bernoulli coefficient
///
/// # Parameters
/// trials (usize): The number of trials
///
/// nb_success (usize): The number of sucess
///
/// # Return
///
/// usize: The bernouilli coefficient for our case
fn compute_coefficient(trials: usize, nb_success: usize) -> usize {
    if nb_success == 0 || trials == 0 {
        return 1;
    }
    let trials: u128 = trials as u128;
    let nb_success: u128 = nb_success as u128;
    let fac_trials: u128 = trials.factorial();
    let fac_nb_success: u128 = nb_success.factorial();
    let fac_product: u128 = (trials - nb_success).factorial();

    (fac_trials / (fac_nb_success * fac_product)) as usize
}

/// This is the mathematical bernouilli function
///
/// # Parameters
/// trials (usize): The number of trials
///
/// nb_success (usize): The number of successes
///
/// probability (f64): The probability of success
///
/// # Return
///
/// f64: The probability that we succeed k time out of n trials
pub fn compute_bernoulli(trials: usize, nb_success: usize, probability: f64) -> f64 {
    let coefficient: usize = compute_coefficient(trials, nb_success);
    coefficient as f64
        * probability.powi(nb_success as i32)
        * (1.0 - probability).powi((trials - nb_success) as i32)
}

/// This function return the sum of an isize and a usize
///
/// # Parameters
/// a (usize): The usize t
///
/// rhs (isize): The isize to add
///
/// # Return
///
/// usize: The sum of a and rhs
pub fn safe_add_signed_unsigned(a: usize, rhs: isize) -> usize {
    if rhs < 0 {
        a.saturating_sub(rhs.unsigned_abs())
    } else {
        a.saturating_add(rhs as usize)
    }
}

/// Evaluate the area covered on a gaussian curve
///
/// # Parameters
///
/// start (usize): The start of the inverval
///
/// end (usize): The end of the interval
///
/// gauss_len (usize): The lenght of the X axis of the curve
///
/// success_probability (f64): The probability that one success occurs
///
/// # Return
///
/// f64: The percentage of the curve covered by the interval
pub fn evaluate_gauss_interval(
    start: usize,
    end: usize,
    gauss_len: usize,
    success_probability: f64,
) -> f64 {
    crate::math_tools::sigma(
        start as isize,
        end as isize,
        |current, params, _, _| {
            crate::math_tools::compute_bernoulli(
                params.unwrap().0,
                current as usize,
                params.unwrap().1,
            )
        },
        Some((gauss_len, success_probability)),
    )
}

/// Find two thresholds for a gaussien assuring that the area covered reprenset at least 0.06% of the curve
///
/// # Parameters
///
/// nb_touch (usize): The number of hit that the attacker can assume
///
/// wound_probability (f64): The probability that a hit wound the enemy
///
/// defender_hp (usize): The amount of health point of the defender
///
/// # Return
///
/// (usize, usize): Our two gaussian threshold
pub fn find_great_gauss_checkpoints(
    nb_touch: usize,
    wound_probability: f64,
    defender_hp: usize,
) -> (usize, usize) {
    let max_hit: usize = std::cmp::min(nb_touch, defender_hp);
    let mut low_checkpoint: usize = (max_hit as f64 * (1.0_f64 / 3.0_f64)).round() as usize;
    let mut high_checkpoint: usize = (max_hit as f64 * (2.0_f64 / 3.0_f64)).round() as usize;
    let compute_proba = |e: usize| compute_bernoulli(nb_touch, e, wound_probability);

    while compute_proba(low_checkpoint) < 0.03 && low_checkpoint <= max_hit / 2 {
        low_checkpoint += 1;
    }
    while compute_proba(high_checkpoint) < 0.03 && high_checkpoint >= max_hit / 2 {
        high_checkpoint -= 1;
    }
    (low_checkpoint, high_checkpoint)
}

#[cfg(test)]
mod tests {

    use super::{compute_bernoulli, compute_coefficient, sigma};

    #[test]
    fn test_sigma() {
        let result: isize =
            sigma::<isize, isize>(0, 5, |curr, _: Option<_>, _, _| curr + 1isize, None);
        let mut expected_result = 0;

        for i in 0..5 {
            expected_result += i + 1;
        }

        assert_eq!(result, expected_result);
    }

    #[test]
    fn compute_coefficient_no_trial() {
        let res = compute_coefficient(0, 4);
        assert_eq!(res, 1);
    }

    #[test]
    fn compute_coefficient_never_succeed() {
        let res = compute_coefficient(4, 0);
        assert_eq!(res, 1);
    }

    #[test]
    fn compute_coefficient_nothing() {
        let res = compute_coefficient(0, 0);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_compute_coefficient() {
        let mut res = compute_coefficient(6, 2);
        assert_eq!(res, 15);
        res = compute_coefficient(10, 4);
        assert_eq!(res, 210);
        res = compute_coefficient(15, 13);
        assert_eq!(res, 105);
    }

    #[test]
    fn test_compute_bernoulli() {
        let mut res = compute_bernoulli(10, 5, 1.0_f64 / 6.0_f64);
        assert_eq!(res - 0.013 < 0.001, true);
        res = compute_bernoulli(10, 1, 1.0_f64 / 6.0_f64);
        assert_eq!(res - 0.324 < 0.001, true);
    }

    #[test]
    fn test_signed_adder() {
        let res: usize = super::safe_add_signed_unsigned(5, 2);
        assert_eq!(res, 7);
        let res: usize = super::safe_add_signed_unsigned(5, -2);
        assert_eq!(res, 3);
        let res: usize = super::safe_add_signed_unsigned(5, -10);
        assert_eq!(res, 0);
    }
}
