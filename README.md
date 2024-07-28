# RSBatch Maestro

A Rust crate for flexible batch splitting and management with various strategies.

## Features

- Even and uneven splitting of totals into batches
- Splitting by count, with remainder, or based on weights
- Range-based splitting and optimization
- Minimum batch size enforcement
- Batch merging and rebalancing
- Handles edge cases and provides meaningful errors

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rsbatch-maestro = "0.2.0"
```

## Usage

```rust
use rsbatch_maestro::even_split;

fn main() {
    match even_split(128, 8) {
        Ok((num_batches, batch_sizes)) => {
            println!("Number of batches: {}", num_batches);
            println!("Batch sizes: {:?}", batch_sizes);
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

## API

The crate provides the following functions:

```rust
pub fn even_split(total: usize, max_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String>
pub fn uneven_split(total: usize, max_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String>
pub fn split_by_count(total: usize, num_batches: usize) -> Result<Vec<NonZeroUsize>, String>
pub fn split_with_remainder(total: usize, batch_size: usize) -> Result<(Vec<NonZeroUsize>, usize), String>
pub fn split_weighted(total: usize, weights: Vec<usize>) -> Result<Vec<NonZeroUsize>, String>
pub fn split_range(total: usize, min_batch_size: usize, max_batch_size: usize) -> Result<Vec<(usize, usize)>, String>
pub fn optimize_split(total: usize, min_batches: usize, max_batches: usize) -> Result<(usize, Vec<NonZeroUsize>), String>
pub fn split_with_min_batch(total: usize, max_batch_size: usize, min_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String>
pub fn split_to_nearest(total: usize, target_batch_size: usize) -> Result<(usize, Vec<NonZeroUsize>), String>
pub fn merge_batches(batches: Vec<NonZeroUsize>, merge_count: usize) -> Result<Vec<NonZeroUsize>, String>
pub fn rebalance_batches(batches: Vec<NonZeroUsize>) -> Vec<NonZeroUsize>
```

For detailed documentation on each function, please refer to the [API documentation](https://docs.rs/rsbatch-maestro).

## Examples

```rust
use rsbatch_maestro::*;
use std::num::NonZeroUsize;

// Even split
assert_eq!(even_split(50, 8), Ok((10, vec![NonZeroUsize::new(5).unwrap(); 10])));

// Uneven split
assert_eq!(uneven_split(50, 8), Ok((7, vec![NonZeroUsize::new(8).unwrap(); 6].into_iter().chain(vec![NonZeroUsize::new(2).unwrap()]).collect())));

// Split by count
assert_eq!(split_by_count(50, 8), Ok(vec![NonZeroUsize::new(7).unwrap(); 2].into_iter().chain(vec![NonZeroUsize::new(6).unwrap(); 6]).collect()));

// Split with remainder
assert_eq!(split_with_remainder(50, 8), Ok((vec![NonZeroUsize::new(8).unwrap(); 6], 2)));

// Split weighted
assert_eq!(split_weighted(100, vec![1, 2, 3, 4]), Ok(vec![NonZeroUsize::new(10).unwrap(), NonZeroUsize::new(20).unwrap(), NonZeroUsize::new(30).unwrap(), NonZeroUsize::new(40).unwrap()]));

// More examples for other functions can be found in the API documentation
```

## Use Cases

Batch Maestro is versatile and can be applied to various scenarios:

1. **Task Distribution**: Distribute tasks among workers in parallel processing systems.
2. **Load Balancing**: Distribute network traffic or database queries across multiple servers.
3. **Resource Allocation**: Allocate resources (e.g., memory, disk space) across different processes or users.
4. **Inventory Management**: Divide inventory items across different storage locations or shipments.
5. **Time Management**: Split a total time duration into segments for schedules or timetables.
6. **Financial Applications**: Divide sums of money for budgeting or investment purposes.
7. **Data Partitioning**: Partition large datasets for distributed processing or storage.
8. **Batch Processing**: Create batches for any kind of batch processing operation.
9. **UI/UX Design**: Distribute elements in user interfaces, such as grid layouts or menu items.
10. **Educational Applications**: Create groups of students for group projects or activities.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Please make sure to update tests as appropriate.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/aeromilai/rsbatch-maestro/tags).

## Authors

* **aeromilai** - *Initial work* - [aeromilai](https://github.com/aeromilai)

See also the list of [contributors](https://github.com/aeromilai/rsbatch-maestro/contributors) who participated in this project.

## Acknowledgements

- Inspired by the need for efficient and flexible resource distribution in various domains.
- Thanks to the Rust community for providing excellent tools and libraries.

## Changelog

For details on our releases, see the [Changelog](CHANGELOG.md).          
