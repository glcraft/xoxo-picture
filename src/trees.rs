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
use std::ops::DerefMut;
use std::ops::Index;

pub trait MinMax {
    fn min_per_value(&self, other: &Self) -> Self;
    fn max_per_value(&self, other: &Self) -> Self;
    fn low() -> Self;
    fn high() -> Self;
    fn average(ls: &[Self]) -> Self where Self: Sized;
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
    Leaf(Vec<T>),
    Empty
}
impl<T, U> Octree<T> 
where
    T: Ord + Clone + MinMax + Index<usize, Output=U>,
    U: PartialOrd
{
    pub fn generate(ls_num:&mut [T]) -> Octree<T> {
        let mut _test = ls_num.len();
        match ls_num.len() {
            0 => Octree::Empty,
            1 => Octree::Leaf(vec!(ls_num[0].clone())),
            _ => Self::dispatch(ls_num)
        }
    }
    fn insert(&mut self, value: T) // à terminer
    {
        match self {
            Octree::Branch(_oct, _pivot) => {

            },
            Octree::Leaf(other)=> {
                let mut new_values:Vec<T> = vec![value];
                new_values.append(other);
                
                *self = Self::dispatch(&mut new_values);
            }
            Octree::Empty => {
                *self = Octree::Leaf(vec![value]);
            }
        }
    }
    pub fn get(&self, search: &T) -> &Octree<T> {
        match self {
            Octree::Branch(oct, pivot) => {
                let mut id=0;
                for i in 0..3 {
                    id |= ((search[i]>pivot[i]) as usize) << i;
                }
                let mut result = oct[id].get(search);
                while let Octree::Empty = result{
                    id+=1;
                    result = oct[id%8].get(search);
                }
                result
            },
            Octree::Leaf(_) | Octree::Empty=> self
        }
    }
}


impl<T, U> Octree<T> 
where
    T: Ord + Clone + MinMax + Index<usize, Output=U>,
    U: PartialOrd
{
    
    fn dispatch(ls_num:&mut [T]) -> Octree<T> {
        if {
            let first = &ls_num[0];
            let mut ok = false;
            for i in 1..ls_num.len() {
                if ls_num[i] != *first {
                    ok=true; 
                    break;
                }
            }
            ok
        } {
            let _test = ls_num.len();
            let pivot = Self::get_pivot(ls_num);
            let mut parts = Self::partition(ls_num, &pivot, 0, 0);
            let uncompressed = [
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap(),
                parts.pop_front().unwrap()
            ];
            let test = 0;
            let tree = [
                Box::new(Self::generate(&mut ls_num[uncompressed[0].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[1].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[2].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[3].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[4].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[5].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[6].clone()])),
                Box::new(Self::generate(&mut ls_num[uncompressed[7].clone()])),
            ];
            Self::Branch(tree, pivot)
        }
        else {
            Self::Leaf(Vec::from(ls_num))
        }

    }
    #[inline]
    fn get_pivot(ls_num:&[T]) -> T {
        T::average(ls_num)
    }
    fn partition<'a>(ls_num:&'a mut [T], pivot: &T, axis:usize, offset: usize) -> LinkedList<std::ops::Range<usize>> {
        let ls = Self::partition_n(ls_num,pivot,axis);
        let mut res = LinkedList::new();
        if axis<(3-1) {
            let start = (ls.0.start, ls.1.start);
            res.append(&mut Self::partition(&mut ls_num[ls.0], pivot, axis+1, offset+start.0));
            res.append(&mut Self::partition(&mut ls_num[ls.1], pivot, axis+1, offset+start.1));
        }
        else {
            res.push_back(ls.0.start+offset..ls.0.end+offset);
            res.push_back(ls.1.start+offset..ls.1.end+offset);
        }
        res
    }
    fn partition_n<'a>(ls_num:&'a mut [T], pivot: &T, axis: usize) -> (std::ops::Range<usize>,std::ops::Range<usize>) {
        // https://en.cppreference.com/w/cpp/algorithm/partition
        if ls_num.is_empty() {
            return (0..0,0..0);
        }
        let mut first = ls_num.len();
    
        for i in 0..ls_num.len() {
            if ls_num[i][axis]>pivot[axis] {
                first = i;
                break;
            }
        }
    
        if first == ls_num.len() {
            return (0..first,first..first);
        }
        for i in first+1..ls_num.len() {
            if ls_num[i][axis]<=pivot[axis] {
                ls_num.swap(first, i);
                first+=1;
            }
        }
        (0..first, first..ls_num.len())
    }
    
}