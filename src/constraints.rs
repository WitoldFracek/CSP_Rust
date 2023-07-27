use num_traits::Num;
use crate::CSP;

pub trait Constraint<P: CSP> {
    fn is_satisfied(&self, problem: &P) -> bool;
}