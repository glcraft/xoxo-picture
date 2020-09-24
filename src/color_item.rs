use std::ops::{Index, IndexMut};
use crate::trees;

#[derive(Eq)]
pub struct ColorItem {
    pub color: [u8; 3],
    pub file: String
}
impl ColorItem {
    pub fn new(color: [u8; 3], file: String) -> Self {
        ColorItem{color, file}
    }
}
impl Ord for ColorItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.color[0]<other.color[0] {
            std::cmp::Ordering::Less
        } else if self.color[0]>other.color[0] {
            std::cmp::Ordering::Greater
        } else {
            if self.color[1]<other.color[1] {
                std::cmp::Ordering::Less
            } else if self.color[1]>other.color[1] {
                std::cmp::Ordering::Greater
            } else {
                if self.color[2]<other.color[2] {
                    std::cmp::Ordering::Less
                } else if self.color[2]>other.color[2] {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    }
}
impl PartialEq for ColorItem {
    fn eq(&self, other: &Self) -> bool {
        self.color[0] == other.color[0] && self.color[1] == other.color[1] && self.color[2] == other.color[2]
    }
}
impl PartialOrd for ColorItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Index<usize> for ColorItem {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.color[idx]
    }
}
impl IndexMut<usize> for ColorItem {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.color[idx]
    }
}
impl trees::MinMax for ColorItem {
    fn min_per_value(&self, other: &Self) -> Self {
        let mut result = Self::high();
        for icol in 0..3 {
            result.color[icol] = self.color[icol].min(other.color[icol]);
        }
        result
    }
    fn max_per_value(&self, other: &Self) -> Self{
        let mut result = Self::low();
        for icol in 0..3 {
            result.color[icol] = self.color[icol].max(other.color[icol]);
        }
        result
    }
    #[inline]
    fn low() -> Self {
        ColorItem {
            color: [u8::MIN; 3],
            file: String::new()
        }
    }
    #[inline]
    fn high() -> Self {
        ColorItem {
            color: [u8::MAX; 3],
            file: String::new()
        }
    }
    fn average(self, other: Self) -> Self {
        ColorItem {
            color: [
                (self.color[0] as u16+other.color[0] as u16 / 2) as u8,
                (self.color[1] as u16+other.color[1] as u16 / 2) as u8,
                (self.color[2] as u16+other.color[2] as u16 / 2) as u8
            ],
            file: String::new()
        }
    }
}
