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

const fn pow_2(i: usize) -> usize{
    if i>1 {
        2*pow_2(i-1)
    } else {
        2
    }
}
pub enum Octree<T> {
    Branch([Box<Octree<T>>;pow_2(3)]),
    Leaf(Option<T>)
}
impl<T> Octree<T> {
    fn generate(ls_num:&mut [T]) -> Octree<T> {
        match ls_num.len() {
            0 => Octree::Leaf(None),
            1 => Octree::Leaf(Some(ls_num[0])),
            _ => Octree::Branch(dispach_tree(ls_num))
        }
    }
    fn dispach(ls_num:&[T]) -> [Box<Octree<T>>;pow_2(3)] {
        ls_num.partition_at_index(index: usize, mut f: F)
    }
    fn partition(ls_num:&[T]){
        
    }
}

pub fn gen_octree<T>(ls_num:&[T]) -> Octree<T> {
    
}

fn dispach_tree<T>(ls_num:&[T]) -> [Box<Octree<T>>;pow_2(3)] {

}