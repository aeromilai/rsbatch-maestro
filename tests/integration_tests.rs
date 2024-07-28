// tests/integration_tests.rs

use batch_maestro::even_split;
use std::num::NonZeroUsize;

#[test]
fn test_various_scenarios() {
    // Test with different total values and max batch sizes
    let scenarios = [
        (100, 10, 10, vec![10; 10]),   // Evenly divisible
        (50, 8, 10, vec![5; 10]),      // Not evenly divisible by max_batch_size
        (17, 5, 17, vec![1; 17]),      // Prime number
        (1000, 7, 200, vec![5; 200]),  // Large number
        (12, 12, 1, vec![12]),         // max_batch_size equals total
        (9, 10, 1, vec![9]),           // max_batch_size greater than total
    ];

    for (total, max_batch_size, expected_num_batches, expected_batch_sizes) in scenarios {
        let result = even_split(total, max_batch_size).unwrap();
        assert_eq!(result.0, expected_num_batches);
        assert_eq!(result.1.len(), expected_num_batches);
        assert_eq!(result.1.iter().map(|&x| x.get()).collect::<Vec<_>>(), expected_batch_sizes);
    }
}

#[test]
fn test_error_conditions() {
    assert!(even_split(0, 5).is_err());  // Total is zero
    assert!(even_split(10, 0).is_err()); // Max batch size is zero
}

#[test]
fn test_large_numbers() {
    let result = even_split(1_000_000, 1000).unwrap();
    assert_eq!(result.0, 1000);
    assert_eq!(result.1, vec![NonZeroUsize::new(1000).unwrap(); 1000]);
}

#[test]
fn test_prime_numbers() {
    let prime_numbers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for &prime in &prime_numbers {
        let result = even_split(prime, 8).unwrap();
        let sum: usize = result.1.iter().map(|&x| x.get()).sum();
        assert_eq!(sum, prime);
        assert!(result.1.iter().all(|&x| x.get() <= 8));
        if prime <= 8 {
            assert_eq!(result.0, 1);
            assert_eq!(result.1, vec![NonZeroUsize::new(prime).unwrap()]);
        } else {
            assert!(result.0 > 1);
        }
    }
}

#[test]
fn test_odd_numbers() {
    let odd_numbers = [3, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    for &odd in &odd_numbers {
        let result = even_split(odd, 8).unwrap();
        let sum: usize = result.1.iter().map(|&x| x.get()).sum();
        assert_eq!(sum, odd);
        assert!(result.1.iter().all(|&x| x.get() <= 8));
        if odd <= 8 {
            assert_eq!(result.1.len(), 1);  // Should return a single batch for odd numbers <= max_batch_size
        } else {
            assert!(result.1.len() > 1);  // Should return multiple batches for odd numbers > max_batch_size
        }
    }
}
