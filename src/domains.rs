use std::ops::AddAssign;
use num_traits::Num;

pub trait Domain {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn reset(&mut self);
    fn value(&self) -> Self::Item;
    fn has_next(&self) -> bool;
    fn empty_value() -> Self::Item;
    fn is_fixed(&self) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct RangeDomain<T> {
    current: T,
    min: T,
    max: T,
}

impl <T: Num + Copy> RangeDomain<T> {
    pub fn new(min: T, max: T) -> Self {
        Self {
            current: min,
            min,
            max,
        }
    }
}

impl <T: Num + Copy + PartialOrd> Domain for RangeDomain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        let ret = self.current;
        self.current = self.current + Self::Item::one();
        Some(ret)
    }

    fn reset(&mut self) {
        self.current = self.min;
    }

    fn value(&self) -> Self::Item {
        self.current
    }

    fn has_next(&self) -> bool {
        self.current < self.max
    }

    fn empty_value() -> Self::Item {
        Self::Item::zero()
    }

    fn is_fixed(&self) -> bool {
        self.min == self.max
    }
}