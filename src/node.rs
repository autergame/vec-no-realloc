pub(crate) struct Node<T> {
    pub(crate) list: Vec<T>,
    pub(crate) last: usize,
    pub(crate) next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            list: vec_with_size(capacity),
            last: 0,
            next: None,
        }
    }

    pub(crate) fn from_elem(elem: T, size: usize) -> Self
    where
        T: Clone,
    {
        Self {
            list: vec![elem; size],
            last: size,
            next: None,
        }
    }

    pub(crate) fn push(&mut self, item: T) {
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

    pub(crate) fn pop(&mut self) -> T {
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

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list: Vec<&T> = self.list.iter().take(self.last).collect();

        let ds = &mut f.debug_struct("Node");

        ds.field("list", &list);
        ds.field("last", &self.last);

        if let Some(next) = &self.next {
            ds.field("next", next);
        } else {
            ds.field("next", &self.next);
        }

        ds.finish()
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
// this vector can only be readed when replaced by valid items
fn vec_with_size<T>(size: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    vec
}
