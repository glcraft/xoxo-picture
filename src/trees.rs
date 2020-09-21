// macro_rules! make_tree {
//     ($name:ident, $n:expr) => {
//         pub enum $name<T> {
//             Branch([Box<$name<T>>;$n]),
//             Leaf(Option<T>)
//         }
//     };
// }

// make_tree!(Octree, 3);
// make_tree!(Quadtree, 2);
use std::collections::LinkedList;
use array_init::array_init;

pub trait TreeItem<T> {
    fn get_item(&self, id: usize) -> &T;
}


const fn pow_2(i: usize) -> usize{
    if i>1 {
        2*pow_2(i-1)
    } else {
        2
    }
}

impl<T> TreeItem<T> for &[T;3] {
    fn get_item(&self, id: usize) -> &T
    {
        &self[id]
    }
}
enum PartitionTree<T> {
    Partition(Box<PartitionTree<T>>, Box<PartitionTree<T>>),
    Value(T, T)
}
pub enum Octree {
    Branch([Box<Octree>;pow_2(3)], [f32;3]),
    Leaf([f32;3]),
    Empty
}
impl Octree {
    fn generate(ls_num:&mut [[f32;3]]) -> Octree {
        match ls_num.len() {
            0 => Octree::Empty,
            1 => Octree::Leaf(ls_num[0]),
            _ => Self::dispatch(ls_num)
        }
    }
    fn dispatch(ls_num:&mut [[f32;3]]) -> Octree {
        
        let pivot = Self::get_pivot(ls_num);
        let mut parts = Self::partition(ls_num, &pivot, 0);
        
        let tree=array_init(|i: usize|{Box::new(Self::generate(&mut ls_num[parts.pop_front().unwrap()]))});
        Self::Branch(tree, pivot)
    }
    fn get_pivot(ls_num:&[[f32;3]]) -> [f32;3] {
        let mut min: [f32;3]=[f32::MAX;3];
        let mut max: [f32;3]=[f32::MIN;3];
        for num in ls_num {
            for idim in 0..3 {
                min[idim] = min[idim].min(num[idim]);
                max[idim] = max[idim].max(num[idim]);
            }
        }
        [(min[0]+max[0])/2.0,(min[1]+max[1])/2.0,(min[2]+max[2])/2.0]
    }
    fn partition<'a>(ls_num:&'a mut [[f32;3]], pivot: &[f32;3], axis:usize) -> LinkedList<std::ops::Range<usize>> {
        let ls = Self::partition_n(ls_num,pivot,axis);
        let mut res = LinkedList::new();
        if axis<=2 {
            res.append(&mut Self::partition(&mut ls_num[ls.0], pivot, axis+1));
            res.append(&mut Self::partition(&mut ls_num[ls.1], pivot, axis+1));
        }
        else {
            res.push_back(ls.0);
            res.push_back(ls.1);
        }
        res
    }
    fn partition_n<'a>(ls_num:&'a mut [[f32;3]], pivot: &[f32;3], axis: usize) -> (std::ops::Range<usize>,std::ops::Range<usize>) {
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