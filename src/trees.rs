// macro_rules! make_tree {
//     ($name:ident, $n:expr) => {
//         pub enum $name<T> {
//             Branch([Box<$name<T>>;$n]),
//             Leaf(Option<T>)
//         }
//     };
// }

// make_tree!(Octree<T>, 3);
// make_tree!(Quadtree, 2);
use std::collections::LinkedList;
use array_init::array_init;
use std::ops::{Add, Div, Index};

pub trait MinMax {
    fn min_per_value(&self, other: &Self) -> Self;
    fn max_per_value(&self, other: &Self) -> Self;
    fn low() -> Self;
    fn high() -> Self;
    fn average(self, other: Self) -> Self;
}


const fn pow_2(i: usize) -> usize{
    if i>1 {
        2*pow_2(i-1)
    } else if i==1 {
        2
    } else {
        1
    }
}


pub enum Octree<T> {
    Branch([Box<Octree<T>>;pow_2(3)], T),
    Leaf(T),
    Empty
}
impl<T, U> Octree<T> 
where
    T: Ord + Clone + From<u32> + MinMax + Index<usize, Output=U>,
    U: PartialOrd
{
    pub fn generate(ls_num:&mut [T]) -> Octree<T> {
        match ls_num.len() {
            0 => Octree::Empty,
            1 => Octree::Leaf(ls_num[0].clone()),
            _ => Self::dispatch(ls_num)
        }
    }
    fn insert(&mut self, value: T) // à terminer
    {
        match self {
            Octree::Branch(oct, pivot) => {

            },
            Octree::Leaf(other)=> {
                let mut t = [value, other.clone()];
                *self = Self::dispatch(&mut t);
            }
            Octree::Empty => {
                *self = Octree::Leaf(value);
            }
        }
    }
}


impl<T, U> Octree<T> 
where
    T: Ord + Clone + From<u32> + MinMax + Index<usize, Output=U>,
    U: PartialOrd
{
    
    fn dispatch(ls_num:&mut [T]) -> Octree<T> {
        
        let pivot = Self::get_pivot(ls_num);
        let mut parts = Self::partition(ls_num, &pivot, 0);
        
        let tree=array_init(|_: usize|{Box::new(Self::generate(&mut ls_num[parts.pop_front().unwrap()]))});
        Self::Branch(tree, pivot)
    }
    fn get_pivot(ls_num:&[T]) -> T {
        let mut min: T=T::low();
        let mut max: T=T::high();
        for num in ls_num.into_iter() {
            min = min.min_per_value(num);
            max = max.max_per_value(num);
        }
        min.average(max)
    }
    fn partition<'a>(ls_num:&'a mut [T], pivot: &T, axis:usize) -> LinkedList<std::ops::Range<usize>> {
        let ls = Self::partition_n(ls_num,pivot,axis);
        let mut res = LinkedList::new();
        if axis<=3-1 {
            res.append(&mut Self::partition(&mut ls_num[ls.0], pivot, axis+1));
            res.append(&mut Self::partition(&mut ls_num[ls.1], pivot, axis+1));
        }
        else {
            res.push_back(ls.0);
            res.push_back(ls.1);
        }
        res
    }
    fn partition_n<'a>(ls_num:&'a mut [T], pivot: &T, axis: usize) -> (std::ops::Range<usize>,std::ops::Range<usize>) {
        let mut first = 0;
        for i in 1..ls_num.len() {
            if ls_num[i][axis]<pivot[axis] {
                ls_num.swap(first, i);
                first+=1;
            }
        }
        (0..first, first..ls_num.len())
    }
    
}