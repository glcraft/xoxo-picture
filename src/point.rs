use std::ops::{Add, Mul, Div, Sub};
use num_traits::Zero;

struct Point<T>{
    value: [T;3]
}

impl<T> Point<T>
where 
    T: Zero
{
    fn new() -> Point<T> {
        Point {
            value: [num_traits::zero();3]
        }
    }
}
impl<T> From<[T;3]> for Point<T> 
{
    fn from(v: [T;3]) -> Self {
        Point {
            value: v
        }
    }
}
impl<T> From<T> for Point<T> {
    fn from(v: T) -> Self {
        Point {
            value: [v;3]
        }
    }
}

macro_rules! impl_op_point {
    ($op_name:tt, $op_function:ident, $op:tt) => {
        impl<T> $op_name<T> for Point<T>
        where 
            T: $op_name<Output = T>
        {
            type Output = Self;
            fn $op_function(self, other: T) -> Self {
                Point::from([self.value[0] $op other,self.value[1] $op other,self.value[2] $op other])
            }
        }
        impl<T> $op_name<[T;3]> for Point<T>
        where 
            T: $op_name<Output = T>
        {
            type Output = Self;
            fn $op_function(self, other: [T;3]) -> Self {
                Point::from([self.value[0] $op other[0],self.value[1] $op other[1],self.value[2] $op other[2]])
            }
        }
        impl<T> $op_name<Point<T>> for Point<T>
        where 
            T: $op_name<Output = T>
        {
            type Output = Self;
            fn $op_function(self, other: Point<T>) -> Self {
                Point::from([self.value[0] $op other.value[0],self.value[1] $op other.value[1],self.value[2] $op other.value[2]])
            }
        }
    };
}
impl_op_point!(Add, add, +);
impl_op_point!(Sub, sub, -);
impl_op_point!(Mul, mul, *);
impl_op_point!(Div, div, /);