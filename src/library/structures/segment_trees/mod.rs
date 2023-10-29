use std::ops::{Add, AddAssign, Mul};

pub mod lazy;

pub trait LazyNode<T>: Copy {
    fn new() -> Self;
    fn add_update(&mut self, value: T);
    fn apply_update(&mut self, l: usize, r: usize) -> T;
    fn merge(&self, other: &Self) -> Self;
    fn value(&self) -> T;
}

#[derive(Copy, Clone)]
pub struct SumNode<T> {
    sum: T,
    lazy: T,
}

pub trait LazyNodeType:
    Default + Copy + AddAssign + Add<Output = Self> + Mul<Output = Self> + TryFrom<usize>
{
}

impl<T> LazyNodeType for T where
    T: Default + Copy + AddAssign + Add<Output = Self> + Mul<Output = Self> + TryFrom<usize>
{
}

impl<T: LazyNodeType> LazyNode<T> for SumNode<T> {
    fn new() -> Self {
        Self {
            sum: T::default(),
            lazy: T::default(),
        }
    }

    fn add_update(&mut self, value: T) {
        self.lazy += value;
    }

    fn apply_update(&mut self, l: usize, r: usize) -> T {
        self.sum += self.lazy * T::try_from(r - l + 1).ok().unwrap();
        let tmp = self.lazy;
        self.lazy = T::default();
        tmp
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            sum: self.sum + other.sum,
            lazy: T::default(),
        }
    }

    fn value(&self) -> T {
        self.sum
    }
}
