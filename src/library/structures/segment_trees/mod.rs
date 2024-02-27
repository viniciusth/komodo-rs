use std::ops::{Add, AddAssign, Mul};

pub mod lazy;

pub trait LazyNode: Copy {
    type T;
    fn new() -> Self;
    fn add_update(&mut self, value: Self::T);
    fn apply_update(&mut self, l: usize, r: usize) -> Self::T;
    fn merge(&self, other: &Self) -> Self;
    fn value(&self) -> Self::T;
}

#[derive(Copy, Clone)]
pub struct SumNode<T> {
    sum: T,
    lazy: T,
}

impl<Ty> LazyNode for SumNode<Ty>

where
    Ty: Copy + Default + Add<Output = Ty> + AddAssign + Mul<Output = Ty> + TryFrom<usize>,
{
    type T = Ty;

    fn new() -> Self {
        Self {
            sum: Ty::default(),
            lazy: Ty::default(),
        }
    }

    fn add_update(&mut self, value: Ty) {
        self.lazy += value;
    }

    fn apply_update(&mut self, l: usize, r: usize) -> Ty {
        self.sum += self.lazy * Ty::try_from(r - l + 1).ok().unwrap();
        let tmp = self.lazy;
        self.lazy = Ty::default();
        tmp
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            sum: self.sum + other.sum,
            lazy: Ty::default(),
        }
    }

    fn value(&self) -> Ty {
        self.sum
    }
}
