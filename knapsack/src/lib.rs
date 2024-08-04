pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut m = vec![0; max_weight as usize + 1];
    for item in items {
        for j in (item.weight as usize..=max_weight as usize).rev() {
            m[j] = m[j].max(m[j - item.weight as usize] + item.value);
        }
    }

    m[max_weight as usize]
}
