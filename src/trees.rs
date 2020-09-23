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
use std::ops::{Add, Div};

pub trait MinMax {
    fn min(&self, other: Self) -> Self;
    fn max(&self, other: Self) -> Self;
    fn low() -> Self;
    fn high() -> Self;
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
    Branch([Box<Octree<T>>;pow_2(3)], [T;3]),
    Leaf([T;3]),
    Empty
}
impl<T> Octree<T> 
where
    T: Add<Output=T> + Div<Output=T> + Ord + Copy + From<u32> + MinMax
{
    pub fn generate(ls_num:&mut [[T;3]]) -> Octree<T> {
        match ls_num.len() {
            0 => Octree::Empty,
            1 => Octree::Leaf(ls_num[0]),
            _ => Self::dispatch(ls_num)
        }
    }
    fn insert(&mut self, value: [T;3]) // à terminer
    {
        match self {
            Octree::Branch(oct, pivot) => {

            },
            Octree::Leaf(other)=> {
                let mut t = [value, *other];
                *self = Self::dispatch(&mut t);
            }
            Octree::Empty => {
                *self = Octree::Leaf(value);
            }
        }
    }
}


impl<T> Octree<T> 
where
    T: Add<Output=T> + Div<Output=T> + Ord + Copy + From<u32> + MinMax
{
    
    fn dispatch(ls_num:&mut [[T;3]]) -> Octree<T> {
        
        let pivot = Self::get_pivot(ls_num);
        let mut parts = Self::partition(ls_num, &pivot, 0);
        
        let tree=array_init(|_: usize|{Box::new(Self::generate(&mut ls_num[parts.pop_front().unwrap()]))});
        Self::Branch(tree, pivot)
    }
    fn get_pivot(ls_num:&[[T;3]]) -> [T;3] {
        let mut min: [T;3]=[T::low();3];
        let mut max: [T;3]=[T::high();3];
        for num in ls_num.into_iter() {
            for idim in 0..3 {
                min[idim] = min[idim].min(num[idim]);
                max[idim] = max[idim].max(num[idim]);
            }
        }
        let two: T = T::from(2);
        [(min[0]+max[0])/two,(min[1]+max[1])/two,(min[2]+max[2])/two]
    }
    fn partition<'a>(ls_num:&'a mut [[T;3]], pivot: &[T;3], axis:usize) -> LinkedList<std::ops::Range<usize>> {
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
    fn partition_n<'a>(ls_num:&'a mut [[T;3]], pivot: &[T;3], axis: usize) -> (std::ops::Range<usize>,std::ops::Range<usize>) {
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