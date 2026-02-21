use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.grades.values().any(|s| s.contains(student)) {
            return;
        }

        self.grades
            .entry(grade)
            .or_default()
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }
}
