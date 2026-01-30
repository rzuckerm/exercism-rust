use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Rc<RefCell<HashSet<String>>>,
}
pub struct Robot {
    name: String,
    used_names: Rc<RefCell<HashSet<String>>>,
}

fn get_robot_name(rng: &mut impl Rng, used_names: &Rc<RefCell<HashSet<String>>>) -> String {
    loop {
        let a = rng.random_range(b'A'..=b'Z') as char;
        let b = rng.random_range(b'A'..=b'Z') as char;
        let n = rng.random_range(0..=999);
        let name = format!("{}{}{:03}", a, b, n);
        if used_names.borrow_mut().insert(name.clone()) {
            return name;
        }
    }
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory {
            used_names: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        Robot {
            name: get_robot_name(rng, &self.used_names),
            used_names: Rc::clone(&self.used_names),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.used_names.borrow_mut().remove(&self.name);
        self.name = get_robot_name(rng, &self.used_names);
    }
}
