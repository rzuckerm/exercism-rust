pub fn raindrops(n: u32) -> String {
    let s: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .map(|(k, sound)| if n % k == 0 { sound } else { "" })
        .collect();
    if s.is_empty() {
        n.to_string()
    } else {
        s
    }
}
