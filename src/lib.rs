//! # Batch Maestro
//!
//! A Rust crate for flexible batch splitting and management with various strategies.
//!
//! This crate provides a comprehensive set of functions to divide a total number into batches,
//! offering different splitting strategies to suit various scenarios such as task distribution,
//! load balancing, and resource allocation.
//!
//! ## Features
//!
//! - Even and uneven splitting of totals into batches
//! - Splitting by count, with remainder, or based on weights
//! - Range-based splitting and optimization
//! - Minimum batch size enforcement
//! - Batch merging and rebalancing
//!
//! ## Usage
//!
//! ```rust
//! use batch_maestro::even_split;
//!
//! fn main() {
//!     match even_split(128, 8) {
//!         Ok((num_batches, batch_sizes)) => {
//!             println!("Number of batches: {}", num_batches);
//!             println!("Batch sizes: {:?}", batch_sizes);
//!         },
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! For more information and examples, please visit the [GitHub repository](https://github.com/aeromilai/batch-maestro).

use std::num::NonZeroUsize;
use std::cmp;

/// Splits a total number into even batches.
///
/// This function takes a total number and a maximum batch size, and attempts to divide the total
/// into as many even batches as possible, without exceeding the maximum batch size.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `max_batch_size` - The maximum allowed size for each batch.
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// 1. The number of batches.
/// 2. A vector of `NonZeroUsize` representing the size of each batch.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The max_batch_size is zero.
///
/// # Examples
///
/// ```
/// use batch_maestro::even_split;
/// use std::num::NonZeroUsize;
///
/// let (num_batches, batch_sizes) = even_split(50, 8).unwrap();
/// assert_eq!(num_batches, 10);
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(5).unwrap(); 10]);
/// ```

pub fn even_split(total: usize, max_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if max_batch_size == 0 {
        return Err(String::from("Max batch size must be a positive number"));
    }

    if total <= max_batch_size {
        return Ok((1, vec![NonZeroUsize::new(total).unwrap()]));
    }

    let mut batch_size = max_batch_size;
    while batch_size > 1 {
        if total % batch_size == 0 {
            let num_batches = total / batch_size;
            return Ok((num_batches, vec![NonZeroUsize::new(batch_size).unwrap(); num_batches]));
        }
        batch_size -= 1;
    }

    Ok((total, vec![NonZeroUsize::new(1).unwrap(); total]))
}

/// Splits the total based on provided weights for each batch.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `weights` - A vector of weights for each batch.
///
/// # Returns
///
/// A `Result` containing a vector of `NonZeroUsize` representing the size of each batch.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The weights vector is empty.
/// * Any weight is zero.
///
/// # Examples
///
/// ```
/// use batch_maestro::split_weighted;
/// use std::num::NonZeroUsize;
///
/// let batch_sizes = split_weighted(100, vec![1, 2, 3]).unwrap();
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(17).unwrap(), NonZeroUsize::new(33).unwrap(), NonZeroUsize::new(50).unwrap()]);
/// ```
pub fn split_weighted(total: usize, weights: Vec<usize>) -> Result<Vec<NonZeroUsize>, String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if weights.is_empty() {
        return Err(String::from("Weights vector must not be empty"));
    }
    if weights.iter().any(|&w| w == 0) {
        return Err(String::from("All weights must be positive numbers"));
    }

    let weight_sum: usize = weights.iter().sum();
    let mut batches = Vec::with_capacity(weights.len());
    let mut remaining = total;

    for (i, &weight) in weights.iter().enumerate() {
        let size = if i == weights.len() - 1 {
            remaining
        } else {
            (total * weight) / weight_sum
        };
        batches.push(NonZeroUsize::new(size).unwrap());
        remaining -= size;
    }

    Ok(batches)
}

/// Generates a range of possible split configurations based on a min and max batch size.
///
/// # Arguments
///
/// * `total` - The total number to be split. 
/// * `min_batch_size` - The minimum allowed size for each batch.
/// * `max_batch_size` - The maximum allowed size for each batch.
///
/// # Returns
///
/// A `Result` containing a vector of tuples, each representing a possible split configuration:
/// (number of batches, batch size, remainder)
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The min_batch_size is zero.
/// * The max_batch_size is less than min_batch_size.
///
/// # Examples
///
/// ```
/// use batch_maestro::split_range;
///
/// let configurations = split_range(100, 20, 40).unwrap();
/// assert_eq!(configurations, vec![(3, 33, 1), (4, 25, 0), (5, 20, 0)]);
/// ```
pub fn split_range(total: usize, min_batch_size: usize, max_batch_size: usize) -> Result<Vec<(usize, usize, usize)>, String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if min_batch_size == 0 {
        return Err(String::from("Minimum batch size must be a positive number"));
    }
    if max_batch_size < min_batch_size {
        return Err(String::from("Maximum batch size must be greater than or equal to minimum batch size"));
    }

    let mut configurations = Vec::new();
    for batch_size in (min_batch_size..=max_batch_size).rev() {
        let num_batches = total / batch_size;
        let remainder = total % batch_size;
        if num_batches > 0 {
            configurations.push((num_batches, batch_size, remainder));
        }
    }

    Ok(configurations)
}

/// Finds the most even split possible within a given range of batch counts.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `min_batches` - The minimum number of batches.
/// * `max_batches` - The maximum number of batches.
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// 1. The number of batches.
/// 2. A vector of `NonZeroUsize` representing the size of each batch.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The min_batches is zero.
/// * The max_batches is less than min_batches.
///
/// # Examples
///
/// ```
/// use batch_maestro::optimize_split;
/// use std::num::NonZeroUsize;
///
/// let (num_batches, batch_sizes) = optimize_split(100, 3, 5).unwrap();
/// assert_eq!(num_batches, 4);
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(25).unwrap(); 4]);
/// ```
pub fn optimize_split(total: usize, min_batches: usize, max_batches: usize) -> Result<(usize, Vec<NonZeroUsize>), String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if min_batches == 0 {
        return Err(String::from("Minimum number of batches must be a positive number"));
    }
    if max_batches < min_batches {
        return Err(String::from("Maximum number of batches must be greater than or equal to minimum number of batches"));
    }

    let mut best_num_batches = min_batches;
    let mut min_remainder = total;

    for num_batches in min_batches..=max_batches {
        let remainder = total % num_batches;
        if remainder < min_remainder {
            best_num_batches = num_batches;
            min_remainder = remainder;
        }
        if remainder == 0 {
            break;
        }
    }

    let base_size = total / best_num_batches;
    let mut batch_sizes = vec![NonZeroUsize::new(base_size).unwrap(); best_num_batches];
    for i in 0..min_remainder {
        batch_sizes[i] = NonZeroUsize::new(base_size + 1).unwrap();
    }

    Ok((best_num_batches, batch_sizes))
}

/// Splits a total number into even batches, ensuring each batch meets a minimum size requirement.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `max_batch_size` - The maximum allowed size for each batch.
/// * `min_batch_size` - The minimum required size for each batch.
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// 1. The number of batches.
/// 2. A vector of `NonZeroUsize` representing the size of each batch.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The max_batch_size is zero.
/// * The min_batch_size is greater than max_batch_size.
/// * It's impossible to create batches that meet the minimum size requirement.
///
/// # Examples
///
/// ```
/// use batch_maestro::split_with_min_batch;
/// use std::num::NonZeroUsize;
///
/// let (num_batches, batch_sizes) = split_with_min_batch(100, 30, 20).unwrap();
/// assert_eq!(num_batches, 4);
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(25).unwrap(); 4]);
/// ```
pub fn split_with_min_batch(total: usize, max_batch_size: usize, min_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if max_batch_size == 0 {
        return Err(String::from("Max batch size must be a positive number"));
    }
    if min_batch_size > max_batch_size {
        return Err(String::from("Min batch size must be less than or equal to max batch size"));
    }

    let num_batches = (total + min_batch_size - 1) / min_batch_size;
    let base_size = total / num_batches;
    let remainder = total % num_batches;

    let mut batch_sizes = Vec::with_capacity(num_batches);
    for i in 0..num_batches {
        let size = base_size + if i < remainder { 1 } else { 0 };
        batch_sizes.push(NonZeroUsize::new(size).unwrap());
    }

    Ok((num_batches, batch_sizes))
}


/// Splits a total number into a specified number of batches.
///
/// This function divides the total into the given number of batches,
/// allowing for uneven distribution if necessary.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `num_batches` - The number of batches to split the total into.
///
/// # Returns
///
/// A `Result` containing a vector of `NonZeroUsize` representing the size of each batch.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The number of batches is zero.
///
/// # Examples
///
/// ```
/// use batch_maestro::split_by_count;
/// use std::num::NonZeroUsize;
///
/// let batch_sizes = split_by_count(10, 3).unwrap();
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(4).unwrap(), NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(3).unwrap()]);
/// ```
pub fn split_by_count(total: usize, num_batches: usize) -> Result<Vec<NonZeroUsize>, String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if num_batches == 0 {
        return Err(String::from("Number of batches must be a positive number"));
    }

    let base_size = total / num_batches;
    let remainder = total % num_batches;

    let mut batches = Vec::with_capacity(num_batches);
    for i in 0..num_batches {
        let size = base_size + if i < remainder { 1 } else { 0 };
        batches.push(NonZeroUsize::new(size).ok_or_else(|| String::from("Failed to create NonZeroUsize"))?);
    }

    Ok(batches)
}

/// Splits a total number into even batches, returning the remainder separately.
///
/// This function is similar to `even_split`, but instead of including the remainder
/// in the last batch, it returns it as a separate value.
///
/// # Arguments
///
/// * `total` - The total number to be split.
/// * `max_batch_size` - The maximum allowed size for each batch.
///
/// # Returns
///
/// A `Result` containing a tuple with:
/// 1. The number of batches.
/// 2. A vector of `NonZeroUsize` representing the size of each batch.
/// 3. The remainder.
///
/// # Errors
///
/// Returns an error if:
/// * The total is zero.
/// * The max_batch_size is zero.
///
/// # Examples
///
/// ```
/// use batch_maestro::split_with_remainder;
/// use std::num::NonZeroUsize;
///
/// let (num_batches, batch_sizes, remainder) = split_with_remainder(50, 8).unwrap();
/// assert_eq!(num_batches, 6);
/// assert_eq!(batch_sizes, vec![NonZeroUsize::new(8).unwrap(); 6]);
/// assert_eq!(remainder, 2);
/// ```
pub fn split_with_remainder(total: usize, max_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>, usize), String> {
    if total == 0 {
        return Err(String::from("Total must be a positive number"));
    }
    if max_batch_size == 0 {
        return Err(String::from("Max batch size must be a positive number"));
    }

    let num_batches = total / max_batch_size;
    let remainder = total % max_batch_size;

    if num_batches == 0 {
        Ok((1, vec![NonZeroUsize::new(total).unwrap()], 0))
    } else {
        Ok((
            num_batches,
            vec![NonZeroUsize::new(max_batch_size).unwrap(); num_batches],
            remainder
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_split_basic() {
        assert_eq!(even_split(50, 8), Ok((10, vec![NonZeroUsize::new(5).unwrap(); 10])));
        assert_eq!(even_split(128, 8), Ok((16, vec![NonZeroUsize::new(8).unwrap(); 16])));
        assert_eq!(even_split(46, 8), Ok((2, vec![NonZeroUsize::new(23).unwrap(); 2])));
        assert_eq!(even_split(7, 8), Ok((1, vec![NonZeroUsize::new(7).unwrap()])));
    }

    #[test]
    fn test_even_split_edge_cases() {
        assert_eq!(even_split(1, 1), Ok((1, vec![NonZeroUsize::new(1).unwrap()])));
        assert_eq!(even_split(100, 100), Ok((1, vec![NonZeroUsize::new(100).unwrap()])));
    }

    #[test]
    fn test_even_split_errors() {
        assert!(even_split(0, 8).is_err());
        assert!(even_split(10, 0).is_err());
    }

    #[test]
    fn test_even_split_large_numbers() {
        assert_eq!(even_split(1000000, 1000), Ok((1000, vec![NonZeroUsize::new(1000).unwrap(); 1000])));
    }

    #[test]
    fn test_even_split_prime_numbers() {
        assert_eq!(even_split(17, 8), Ok((1, vec![NonZeroUsize::new(17).unwrap()])));
        assert_eq!(even_split(23, 8), Ok((1, vec![NonZeroUsize::new(23).unwrap()])));
    }

    #[test]
    fn test_split_by_count() {
        assert_eq!(split_by_count(10, 3), Ok(vec![NonZeroUsize::new(4).unwrap(), NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(3).unwrap()]));
        assert_eq!(split_by_count(20, 4), Ok(vec![NonZeroUsize::new(5).unwrap(); 4]));
        assert_eq!(split_by_count(7, 3), Ok(vec![NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(2).unwrap(), NonZeroUsize::new(2).unwrap()]));
    }

    #[test]
    fn test_split_by_count_errors() {
        assert!(split_by_count(0, 5).is_err());
        assert!(split_by_count(10, 0).is_err());
    }

    #[test]
    fn test_split_with_remainder() {
        assert_eq!(split_with_remainder(50, 8), Ok((6, vec![NonZeroUsize::new(8).unwrap(); 6], 2)));
        assert_eq!(split_with_remainder(100, 30), Ok((3, vec![NonZeroUsize::new(30).unwrap(); 3], 10)));
        assert_eq!(split_with_remainder(10, 20), Ok((1, vec![NonZeroUsize::new(10).unwrap()], 0)));
    }

    #[test]
    fn test_split_with_remainder_errors() {
        assert!(split_with_remainder(0, 5).is_err());
        assert!(split_with_remainder(10, 0).is_err());
    }

    #[test]
    fn test_split_weighted() {
        assert_eq!(split_weighted(100, vec![1, 2, 3]), Ok(vec![NonZeroUsize::new(17).unwrap(), NonZeroUsize::new(33).unwrap(), NonZeroUsize::new(50).unwrap()]));
        assert_eq!(split_weighted(10, vec![1, 1]), Ok(vec![NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap()]));
    }

    #[test]
    fn test_split_weighted_errors() {
        assert!(split_weighted(0, vec![1, 2, 3]).is_err());
        assert!(split_weighted(100, vec![]).is_err());
        assert!(split_weighted(100, vec![0, 1, 2]).is_err());
    }

    #[test]
    fn test_split_range() {
        assert_eq!(split_range(100, 20, 40), Ok(vec![(3, 33, 1), (4, 25, 0), (5, 20, 0)]));
        assert_eq!(split_range(10, 2, 5), Ok(vec![(2, 5, 0), (3, 3, 1), (4, 2, 2)]));
    }

    #[test]
    fn test_split_range_errors() {
        assert!(split_range(0, 20, 40).is_err());
        assert!(split_range(100, 0, 40).is_err());
        assert!(split_range(100, 40, 20).is_err());
    }

    #[test]
    fn test_optimize_split() {
        assert_eq!(optimize_split(100, 3, 5), Ok((4, vec![NonZeroUsize::new(25).unwrap(); 4])));
        assert_eq!(optimize_split(10, 2, 4), Ok((2, vec![NonZeroUsize::new(5).unwrap(); 2])));
    }

    #[test]
    fn test_optimize_split_errors() {
        assert!(optimize_split(0, 3, 5).is_err());
        assert!(optimize_split(100, 0, 5).is_err());
        assert!(optimize_split(100, 5, 3).is_err());
    }

    #[test]
    fn test_split_with_min_batch() {
        assert_eq!(split_with_min_batch(100, 30, 20), Ok((4, vec![NonZeroUsize::new(25).unwrap(); 4])));
        assert_eq!(split_with_min_batch(50, 20, 10), Ok((3, vec![NonZeroUsize::new(17).unwrap(), NonZeroUsize::new(17).unwrap(), NonZeroUsize::new(16).unwrap()])));
    }

    #[test]
    fn test_split_with_min_batch_errors() {
        assert!(split_with_min_batch(0, 30, 20).is_err());
        assert!(split_with_min_batch(100, 0, 20).is_err());
        assert!(split_with_min_batch(100, 30, 40).is_err());
        assert!(split_with_min_batch(100, 30, 31).is_err());
    }
}
