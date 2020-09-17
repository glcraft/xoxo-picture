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
enum PartitionTree<T> {
    Partition(Box<PartitionTree<T>>, Box<PartitionTree<T>>),
    Value(T, T)
}
pub enum Octree {
    Branch([Box<Octree>;pow_2(3)], [f32;3]),
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
        let parts = Self::partition(ls_num,0);
        
    }
    fn partition(ls_num:&mut [[f32;3]], i:usize) -> PartitionTree<&mut[[f32;3]]> {
        let mut min: [f32;3]=[f32::MAX;3];
        let mut max: [f32;3]=[f32::MIN;3];
        for num in ls_num {
            for idim in 0..3 {
                min[idim] = min[idim].min(num[idim]);
                max[idim] = max[idim].max(num[idim]);
            }
        }
        let pivot: [f32;3] = [0.;3];
        for idim in 0..3 {
            pivot[idim]=(min[idim]+max[idim])/2.0;
        }
        Self::partition_n(ls_num, pivot, i)
    }
    fn partition_n(ls_num:&mut [[f32;3]], pivot: [f32;3], i:usize) -> PartitionTree<&mut[[f32;3]]>{
        let mut first = 0;
        let mut val = ls_num.iter_mut();
        val.next();
        while let Some(val) = val.next() {
            if val[i]<pivot[i] {
                std::mem::swap(val, &mut ls_num[first]);
                first+=1;
            }
        }
        let v=(&mut ls_num[0..first], &mut ls_num[first+1..0]);
        if i!=3-1 {
            PartitionTree::Partition(Box::new(Self::partition_n(v.0, pivot,i+1)), Box::new(Self::partition_n(v.1, pivot,i+1)))
        }
        else {
            PartitionTree::Value(v.0,v.1)
        }
    }
}