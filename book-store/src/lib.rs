use std::collections::HashMap;

const BOOK_PRICE: u32 = 800;
const DISCOUNT_MULTIPIER: &[u32; 6] = &[0, 100, 2 * 95, 3 * 90, 4 * 80, 5 * 75];

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    books
        .iter()
        .for_each(|&n| *counts.entry(n).or_insert(0) += 1);
    let mut counts: Vec<u32> = counts.into_values().collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    find_lowest_price(counts)
}

fn find_lowest_price(counts: Vec<u32>) -> u32 {
    let mut lowest_price = BOOK_PRICE * counts.iter().sum::<u32>();

    let n = counts.iter().filter(|&&n| n > 0).count() + 1;
    for (i, d) in DISCOUNT_MULTIPIER.iter().enumerate().take(n).skip(2) {
        let mut new_counts = counts.clone();
        for new_count in new_counts.iter_mut().take(i) {
            *new_count -= 1;
        }

        new_counts.sort_unstable_by(|a, b| b.cmp(a));
        lowest_price = lowest_price.min(BOOK_PRICE * d / 100 + find_lowest_price(new_counts));
    }

    lowest_price
}
