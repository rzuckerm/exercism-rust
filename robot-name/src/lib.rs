use std::sync::atomic::{AtomicUsize, Ordering};

static SERIAL_NUM: AtomicUsize = AtomicUsize::new(0);

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(get_robot_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = get_robot_name();
    }
}

fn get_robot_name() -> String {
    let n = SERIAL_NUM.fetch_add(1, Ordering::SeqCst);
    let a = (b'A' + ((n / 26000) % 26) as u8) as char;
    let b = (b'A' + ((n / 1000) % 26) as u8) as char;
    format!("{}{}{:03}", a, b, n % 1000)
}
