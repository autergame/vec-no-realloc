use crate::VecNoRealloc;

impl<T> std::ops::Index<usize> for VecNoRealloc<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(item) = self.get(index) {
            item
        } else {
            panic!("index out of bounds")
        }
    }
}

impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if let Some(item) = self.get_mut(index) {
            item
        } else {
            panic!("index out of bounds")
        }
    }
}
