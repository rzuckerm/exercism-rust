use std::cmp::min;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let workers_needed = min(input.len(), worker_count);
    match workers_needed {
        0 | 1 => get_frequency_count(input),
        _ => thread::scope(|s| {
            let threads = input
                .chunks((input.len() + workers_needed - 1) / workers_needed)
                .map(|chunk| s.spawn(|| get_frequency_count(chunk)));
            let mut count = HashMap::new();
            for t in threads {
                for (&k, v) in t.join().unwrap().iter() {
                    *count.entry(k).or_insert(0) += v;
                }        
            }

            count
        }),
    }
}

fn get_frequency_count(input: &[&str]) -> HashMap<char, usize> {
    let mut count = HashMap::new();
    for &input_line in input {
        for c in input_line.chars() {
            if c.is_alphabetic() {
                *count.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            }
        }
    }

    count
}
