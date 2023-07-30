use std::ops::AddAssign;
use num_traits::Num;

pub trait Domain {
    type Item;
    fn next(&mut self);
    fn reset(&mut self);
    fn value(&self) -> Option<Self::Item>;
    fn has_next(&self) -> bool;
    fn value_belongs(&self, value: Self::Item) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct FixedDomain<T> {
    value: T
}

impl <T: Num + Copy> FixedDomain<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl <T: Num + Copy> Domain for FixedDomain<T> {
    type Item = T;

    fn next(&mut self) { }

    fn reset(&mut self) { }

    fn value(&self) -> Option<Self::Item> {
        Some(self.value)
    }

    fn has_next(&self) -> bool {
        false
    }

    fn value_belongs(&self, value: Self::Item) -> bool {
        value == self.value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RangeDomain<T> {
    current: Option<T>,
    min: T,
    max: T,
}

impl <T: Num + Copy> RangeDomain<T> {
    pub fn new(min: T, max: T) -> Self {
        Self {
            current: None,
            min,
            max,
        }
    }
}

impl <T: Num + Copy + PartialOrd> Domain for RangeDomain<T> {
    type Item = T;

    fn next(&mut self) {
        if !self.has_next() {
            return;
        }
        self.current = match self.current {
            None => Some(self.min),
            Some(value) => Some(value + Self::Item::one())
        };
    }

    fn reset(&mut self) {
        self.current = None;
    }

    fn value(&self) -> Option<Self::Item> {
        self.current
    }

    fn has_next(&self) -> bool {
        match self.current {
            None => true,
            Some(value) if value < self.max => true,
            _ => false,
        }
    }

    fn value_belongs(&self, value: Self::Item) -> bool {
        value >= self.min && value <= self.max
    }
}