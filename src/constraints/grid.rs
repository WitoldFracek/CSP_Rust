use std::collections::HashSet;
use std::hash::Hash;
use num_traits::Num;
use crate::{Constraint, CSP, Grid};

trait Unique {
    fn unique<I, T: Hash + Eq>(iter: I) -> bool where I: Iterator<Item=Option<T>> {
        let mut set = HashSet::new();
        for elem in iter {
            match elem {
                None => continue,
                Some(value) if set.contains(&value) => return false,
                Some(value) => set.insert(value),
            };
        }
        return true;
    }
}

pub struct RowUnique {

}

impl RowUnique {
    pub fn new() -> Self {
        Self { }
    }
}

impl Unique for RowUnique { }

impl <T: Copy + Hash + Eq> Constraint<Grid<T>> for RowUnique {
    fn is_satisfied(&self, problem: &Grid<T>) -> bool {
        problem.domains
            .iter()
            .all(|row| RowUnique::unique(row
                .iter()
                .map(|x| x.value())))
    }
}

pub struct ColUnique {

}

impl ColUnique {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unique for ColUnique { }

impl <T: Copy + Hash + Eq> Constraint<Grid<T>> for ColUnique {
    fn is_satisfied(&self, problem: &Grid<T>) -> bool {
        (0..problem.size)
            .all(|i| ColUnique::unique(problem.domains
                .iter()
                .map(|row| row[i].value())))
    }
}