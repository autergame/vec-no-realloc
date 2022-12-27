use crate::VecNoRealloc;

impl<T> std::ops::Index<usize> for VecNoRealloc<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
		self.get(index).expect("index out of bounds")
    }
}

impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
		self.get_mut(index).expect("index out of bounds")
    }
}
