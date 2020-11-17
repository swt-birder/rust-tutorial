use std::collections::HashMap;

fn main() {
    let mut numbers = vec![3, 3, 5, 1, 4, 2, 6, 2, 3, 10];
    let mut sum = 0;
    let mut max_v = 0;
    let mut max_k = 0;
    let median: i32;

    // mean
    for x in numbers.iter() {
        sum = sum + x;
    }
    let mean = sum / numbers.len() as i32;
    println!("mean: {}", mean);

    // median
    numbers.sort();
    let middle = numbers.len() / 2;
    median = numbers[middle];
    println!("mediann: {}", median);

    // mode
    let mut scores = HashMap::new();
    for x in numbers.iter() {
        let count = scores.entry(x).or_insert(0);
        println!("{}", x);
        *count += 1;
    }
    for (key, value) in scores {
        println!("{}: {}", key, value);
        if value > max_v {
            max_v = value;
            max_k = *key;
        }
    }
    println!("Mode {}", max_k);
}
