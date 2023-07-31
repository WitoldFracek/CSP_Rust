use std::collections::HashSet;
use std::hash::Hash;
use crate::{Constraint, Grid};

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

pub struct SudokuSquare {

}

impl SudokuSquare {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unique for SudokuSquare { }

impl <T: Copy + Hash + Eq> Constraint<Grid<T>> for SudokuSquare {
    fn is_satisfied(&self, problem: &Grid<T>) -> bool {
        let n = (problem.size as f64).sqrt() as usize;
        let square_it = |i, j| problem.domains[i..i+n]
            .iter()
            .flat_map(move |row| &row[j..j+n])
            .map(|d| d.value());
        (0..n).all(|i| (0..n).all(|j| Self::unique(square_it(i * n, j * n))))
    }
}

trait BinaryEquilibrium {
    fn in_equilibrium<I>(iter: I) -> bool where I: Iterator<Item=Option<i8>> {
        let mut counter = [0_i8; 3];
        let mut sum = 0;
        iter.for_each(|x| {
            sum += 1;
            match x {
                None => counter[2] += 1,
                Some(value) => counter[value as usize] += 1,
            }
        });
        (counter[0] - counter[1]).abs() <= counter[2] + (sum % 2)
    }
}

pub struct RowBinaryEquilibrium {

}

impl RowBinaryEquilibrium {
    pub fn new() -> Self {
        Self { }
    }
}

impl BinaryEquilibrium for RowBinaryEquilibrium { }

impl Constraint<Grid<i8>> for RowBinaryEquilibrium {
    fn is_satisfied(&self, problem: &Grid<i8>) -> bool {
        problem.domains
            .iter()
            .all(|row| Self::in_equilibrium(row
                .iter()
                .map(|x| x.value())))
    }
}

pub struct ColBinaryEquilibrium {

}

impl ColBinaryEquilibrium {
    pub fn new() -> Self {
        Self { }
    }
}

impl BinaryEquilibrium for ColBinaryEquilibrium { }

impl Constraint<Grid<i8>> for ColBinaryEquilibrium {
    fn is_satisfied(&self, problem: &Grid<i8>) -> bool {
        (0..problem.size)
            .all(|i| Self::in_equilibrium(problem.domains
                .iter()
                .map(|row| row[i].value())))
    }
}

