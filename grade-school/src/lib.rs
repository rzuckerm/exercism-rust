use std::collections::{BTreeMap, BTreeSet};

pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.0.values().any(|s| s.contains(student)) {
            self.0.entry(grade).or_default().insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0
            .get(&grade)
            .map_or(Vec::new(), |s| s.iter().cloned().collect())
    }
}
