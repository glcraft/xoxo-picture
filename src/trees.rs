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

pub enum Octree {
    Branch([Box<Octree>;pow_2(3)]),
    Leaf(Option<[f32;3]>)
}
impl Octree {
    fn generate(ls_num:&mut [[f32;3]]) -> Octree {
        match ls_num.len() {
            0 => Octree::Leaf(None),
            1 => Octree::Leaf(Some(ls_num[0])),
            _ => Octree::Branch(Self::dispatch(ls_num))
        }
    }
    fn dispatch(ls_num:&mut [[f32;3]]) -> [Box<Octree>;pow_2(3)] {
        let mut tree: [Box<Octree>;pow_2(3)];
        // Self::partition(ls_num)
        
    }
    fn partition(ls_num:&mut [[f32;3]], i:usize) -> (&mut[[f32;3]], &mut[[f32;3]]) {
        let mut min=f32::MAX;
        let mut max=f32::MIN;
        for num in ls_num {
            min = min.min(num[0]);
            max = max.max(num[0]);
        }
        let pivot = (min+max)/2.0;
        let first = ls_num.iter_mut();
        for val in ls_num.iter_mut() {

        }
        (ls_num, ls_num)
    }
}