use rand::Rng;

fn coder(key: &str, s: &str, f: impl Fn(u8, u8) -> u8) -> Option<String> {
    (!key.is_empty() && key.chars().all(|k| k.is_ascii_lowercase())).then_some(
        key.chars()
            .cycle()
            .zip(s.chars())
            .map(|(k, c)| (f((c as u8 & 0x1f) - 1, (k as u8 & 0x1f) - 1) % 26 + b'a') as char)
            .collect(),
    )
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    coder(key, s, |c, k| c + k)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    coder(key, s, |c, k| c + 26 - k)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..100).map(|_| rng.gen_range('a'..='z')).collect();
    (key.clone(), encode(&key, s).unwrap())
}
