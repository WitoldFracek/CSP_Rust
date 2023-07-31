pub mod grid;

use crate::CSP;

pub trait Constraint<P: CSP> {
    fn is_satisfied(&self, problem: &P) -> bool;
}