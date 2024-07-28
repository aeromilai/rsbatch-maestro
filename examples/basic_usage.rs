// examples/basic_usage.rs

use rseven_splitter::even_split;

fn main() {
    let total_items = 100;
    let max_batch_size = 8;

    match even_split(total_items, max_batch_size) {
        Ok((num_batches, batch_sizes)) => {
            println!("Total items: {}", total_items);
            println!("Maximum batch size: {}", max_batch_size);
            println!("Number of batches: {}", num_batches);
            println!("Batch sizes: {:?}", batch_sizes);

            // Demonstrate how to use the results
            for (i, batch_size) in batch_sizes.iter().enumerate() {
                println!("Batch {}: Processing {} items", i + 1, batch_size);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Another example with prime number
    let prime_total = 17;
    match even_split(prime_total, max_batch_size) {
        Ok((num_batches, batch_sizes)) => {
            println!("\nTotal items (prime number): {}", prime_total);
            println!("Number of batches: {}", num_batches);
            println!("Batch sizes: {:?}", batch_sizes);
        }
        Err(e) => println!("Error: {}", e),
    }
}
