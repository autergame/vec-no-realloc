use crate::VecNoRealloc;

impl<T> std::ops::Index<usize> for VecNoRealloc<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut search = index;
        let mut current = &self.head;

        while let Some(node) = current {
            if search < self.bucket_size {
                if search >= node.last {
                    break;
                }
                return &node.list[search];
            }
            search -= self.bucket_size;

            current = &node.next;
        }

        panic!("index out of bounds")
    }
}

impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let mut search = index;
        let mut current = &mut self.head;

        while let Some(node) = current {
            if search < self.bucket_size {
                if search >= node.last {
                    break;
                }
                return &mut node.list[search];
            }
            search -= self.bucket_size;

            current = &mut node.next;
        }

        panic!("index out of bounds")
    }
}
