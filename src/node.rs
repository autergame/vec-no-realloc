pub(crate) struct Node<T> {
    pub list: Vec<T>,
    pub last: usize,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: vec_with_size(capacity),
            last: 0,
            next: None,
        }
    }

    pub fn from_elem(elem: T, size: usize) -> Self
    where
        T: Clone,
    {
        Self {
            list: vec![elem; size],
            last: size,
            next: None,
        }
    }

    pub fn push(&mut self, item: T) {
        // SAFETY:
        // this will only overwrite a
        // unused item in list, needed for pop
        unsafe {
            let ptr = self.list.as_mut_ptr();
            let end = ptr.add(self.last);
            end.write(item);
        }
        self.last += 1;
    }

    pub fn pop(&mut self) -> T {
        // SAFETY:
        // this will never return a ownership to the same
        // index more than once and will be marked as unused
        self.last -= 1;
        unsafe {
            let ptr = self.list.as_ptr();
            let end = ptr.add(self.last);
            end.read()
        }
    }
}

// SAFETY:
// make drop only be applied to valid items in list
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            self.list.set_len(self.last);
        }
    }
}

// SAFETY:
// this vector will only be readed when replaced by valid items
#[allow(clippy::uninit_vec)]
fn vec_with_size<T>(size: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    vec
}
