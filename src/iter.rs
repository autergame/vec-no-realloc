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
        let item = self.vnr.get(self.index);
        if item.is_some() {
            self.index += 1;
        }
        item
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
        match self.vnr.get_mut(self.index) {
            Some(item) => {
                // SAFETY:
                // this will never return a mutable
                // reference to the same index more than once
                self.index += 1;
                Some(unsafe { &mut *(item as *mut T) })
            }
            None => None,
        }
    }
}
