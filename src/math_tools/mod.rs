//! # Math Tools module
//!
//! This module contain some useful mathematic tools
//! Those tools will be used for our computations

use factorial::Factorial;

/// This is the math sigma function
/// It execute the given function on the given range and return the sum of the result of the function.
///
/// ## Parameters
/// initial (isize): The first value of the range
///
/// max (isize): The last value of the range
///
/// function (fn(isize, Option<P>, isize, isize)) -> T: The function to execute on the range.
/// This function params are : 1. The current value of the range 2. The parameter of the function 3. The initial value of the range 4. The final value of the range
///
/// function_parameter (Option<P>): A parameter that can be passed to the function
///
/// ## Return
/// T -> The result of our sigma computation
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
    return sum;
}

/// This function compute the bernoulli coefficient
///
/// ## Parameters
/// trials(usize): The number of trials
///
/// nb_success(usize): The number of sucess
///
/// ## Return
/// usize -> The bernouilli coefficient for our case
fn compute_coefficient(trials: usize, nb_success: usize) -> usize {
    if nb_success == 0 || trials == 0 {
        return 1;
    }
    let fac_trials: u128 = trials.factorial() as u128;
    let fac_nb_success: u128 = nb_success.factorial() as u128;
    let fac_product: u128 = (trials - nb_success).factorial() as u128;

    (fac_trials / (fac_nb_success * fac_product)) as usize
}

/// This is the mathematical bernouilli function
///
/// ## Parameters
/// trials (usize): The number of trials
///
/// nb_success (usize): The number of successes
///
/// probability (f64): The probability of success
///
/// ## Return
/// f64 -> The probability that we succeed k time out of n trials
pub fn compute_bernoulli(trials: usize, nb_success: usize, probability: f64) -> f64 {
    let coefficient: usize = compute_coefficient(trials, nb_success);
    return coefficient as f64
        * probability.powi(nb_success as i32)
        * (1.0 - probability).powi((trials - nb_success) as i32);
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
}
