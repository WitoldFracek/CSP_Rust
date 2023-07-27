use num_traits::Num;
use crate::CSP;

pub trait Constraint {
    fn is_satisfied(&self) -> bool;
}

pub struct NumSeqUnique<T> where T: Num {
    min: T,
    max: T,
    arr: Vec<T>,
}

impl NumSeqUnique<u8> {
    pub fn new(min: u8, max: u8) -> Self {
        let spread = max - min + 1;
        let arr = vec![0; spread as usize];
        Self {
            min, max, arr
        }
    }
}