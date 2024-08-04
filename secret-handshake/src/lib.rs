pub fn actions(n: u8) -> Vec<&'static str> {
    let handshake = (0..4)
        .filter(|i| (n as usize) & (1 << i) != 0)
        .map(|i| ["wink", "double blink", "close your eyes", "jump"][i]);
    match n & 0x10 {
        0 => handshake.collect(),
        _ => handshake.rev().collect(),
    }
}
