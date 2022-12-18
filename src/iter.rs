use crate::VecNoRealloc;

pub struct Iter<'a, T> {
    pub(crate) vnr: &'a VecNoRealloc<T>,
    pub(crate) index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(vnr: &'a VecNoRealloc<T>) -> Self {
        Self { vnr, index: 0 }
    }
}

impl<'a, T> IntoIterator for &'a VecNoRealloc<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut search = self.index;
        let mut current = &self.vnr.head;

        while let Some(node) = current {
            if search < self.vnr.bucket_size {
                if search >= node.last {
                    break;
                }
                self.index += 1;
                return Some(&node.list[search]);
            }
            search -= self.vnr.bucket_size;

            current = &node.next;
        }

        None
    }
}

pub struct IterMut<'a, T> {
    pub(crate) vnr: &'a mut VecNoRealloc<T>,
    pub(crate) index: usize,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(vnr: &'a mut VecNoRealloc<T>) -> Self {
        Self { vnr, index: 0 }
    }
}

impl<'a, T> IntoIterator for &'a mut VecNoRealloc<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut search = self.index;
        let mut current = &mut self.vnr.head;

        while let Some(node) = current {
            if search < self.vnr.bucket_size {
                if search >= node.last {
                    break;
                }
                self.index += 1;
                // SAFETY:
                // this will never return a mutable
                // reference to the same index more than once
                return Some(unsafe {
                    let ptr = &mut node.list[search];
                    &mut *(ptr as *mut _)
                });
            }
            search -= self.vnr.bucket_size;

            current = &mut node.next;
        }

        None
    }
}
