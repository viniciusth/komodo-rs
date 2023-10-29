use std::ops::{Add, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait PointType:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Default
    + PartialOrd
    + Mul<Output = Self>
{
}

impl<T> PointType for T where
    T: Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Neg<Output = Self>
        + Default
        + PartialOrd
        + Mul<Output = Self>
{
}

impl<T> Point<T>
where
    T: PointType,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U, F: Fn(T) -> U>(&self, f: F) -> Point<U> {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }

    fn abs(x: T) -> T {
        if x < T::default() {
            -x
        } else {
            x
        }
    }

    pub fn manhattan(&self, other: &Self) -> T {
        Self::abs(self.x - other.x) + Self::abs(self.y - other.y)
    }

    pub fn rotate_90(&self) -> Self {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn signed_area(&self, a: &Self, b: &Self) -> T {
        (*a - *self).cross(&(*b - *self))
    }

    pub fn dist2(&self, other: &Self) -> T {
        (*self - *other).dot(&(*self - *other))
    }

    pub fn orientation(&self, a: &Self, b: &Self) -> Orientation {
        let area = self.signed_area(a, b);
        if area > T::default() {
            Orientation::Clockwise
        } else if area < T::default() {
            Orientation::CounterClockwise
        } else {
            Orientation::Collinear
        }
    }
}

pub enum Orientation {
    Clockwise,
    CounterClockwise,
    Collinear,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn partial_cmp<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(&b).unwrap()
}
