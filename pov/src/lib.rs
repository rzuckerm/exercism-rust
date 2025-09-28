use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Default::default(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self.children.sort_unstable_by(|a, b| a.label.cmp(&b.label));
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        let Some(path) = self.find_path(from) {
            return false;
        };

        for idx in path {
            let mut child = self.children.remove(idx);
            self.children.sort_unstable_by(|a, b| a.label.cmp(&b.label));
            std::mem::swap(self, &mut child);
            self.children.push(child);
        }

        self.children.sort_unstable_by(|a, b| a.label.cmp(&b.label));
        true
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        let from_path = self.find_path(from)?;
        let to_path = self.find_path(to)?;
        let common_length = from_path
            .iter()
            .zip(&to_path)
            .take_while(|(a, b)| a == b)
            .count();
        let from_labels = self.get_labels(&from_path);
        let to_labels = self.get_labels(&to_path);
        let mut path = from_labels[common_length..]
            .iter()
            .rev()
            .copied()
            .collect::<Vec<_>>();
        if common_length == 0 {
            path.push(&self.label);
        }

        path.extend_from_slice(&to_labels[common_length.saturating_sub(1)..]);
        Some(path)
    }

    fn find_path(&self, target: &T) -> Option<Vec<usize>> {
        if self.label == *target {
            return Some(Vec::new());
        }

        for (idx, child) in self.children.iter().enumerate() {
            if let Some(mut path) = child.find_path(target) {
                path.insert(0, idx);
                return Some(path);
            }
        }

        None
    }

    fn get_labels<'a>(&'a self, path: &[usize]) -> Vec<&'a T> {
        let mut node = self;
        path.iter()
            .map(|&idx| {
                node = &node.children[idx];
                &node.label
            })
            .collect()
    }
}
