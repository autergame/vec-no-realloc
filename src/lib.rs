mod fmt;
mod index;
mod iter;
mod node;

pub struct VecNoRealloc<T> {
    pub(crate) bucket_size: usize,
    pub(crate) head: Option<Box<node::Node<T>>>,
}

impl<T> VecNoRealloc<T> {
    pub fn new() -> Self {
        Self {
            bucket_size: 10,
            head: None,
        }
    }
    pub fn with_capacity(size: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            bucket_size: size,
            head: SomeBox!(node::Node::with_capacity(size)),
        }
    }
    pub fn from_elem(elem: T, size: usize) -> Self
    where
        T: Clone,
    {
        Self {
            bucket_size: size,
            head: SomeBox!(node::Node::from_elem(elem, size)),
        }
    }
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Default + Clone,
    {
        let mut vnr = Self {
            bucket_size: 10,
            head: None,
        };

        for chunk in slice.chunks(vnr.bucket_size) {
            for i in 0..chunk.len() {
                vnr.push(chunk[i].clone());
            }
        }

        vnr
    }
    pub(crate) fn iterate<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a node::Node<T>),
    {
        let mut current = &self.head;

        while let Some(node) = current {
            f(node);

            current = &node.next;
        }
    }
    pub fn len(&self) -> usize {
        let mut count = 0;

        self.iterate(|node| {
            count += node.last;
        });

        count
    }
    pub fn capacity(&self) -> usize {
        let mut count = 0;

        self.iterate(|_| {
            count += self.bucket_size;
        });

        count
    }
    pub fn push(&mut self, item: T)
    where
        T: Default + Clone,
    {
        let mut current = &mut self.head;

        while let Some(node) = current {
            if node.last < self.bucket_size {
                // SAFETY:
                // this will only overwrite a
                // unused item in list, needed for pop
                unsafe {
                    let ptr = node.list.as_mut_ptr();
                    let end = ptr.add(node.last);
                    end.write(item);
                }
                node.last += 1;
                return;
            }
            current = &mut node.next;
        }

        let mut new = node::Node::<T>::with_capacity(self.bucket_size);
        new.list[0] = item;
        new.last = 1;

        *current = SomeBox!(new);
    }
    pub fn pop_del(&mut self, remove: bool) -> Option<T> {
        let mut current = &mut self.head;

        if let Some(node) = current {
            if node.last == 0 {
                if remove {
                    *current = None;
                }
                return None;
            }
        }

        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.last != 0 {
                    current = &mut node.next;
                    continue;
                }
                if remove {
                    node.next = None;
                }
            }
            node.last -= 1;
            // SAFETY:
            // this will never return a ownership to the same
            // index more than once and will be marked as unused
            return Some(unsafe {
                let ptr = node.list.as_ptr();
                let end = ptr.add(node.last);
                end.read()
            });
        }

        None
    }
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.pop_del(false)
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut search = index;
        let mut current = &self.head;

        while let Some(node) = current {
            if search < self.bucket_size {
                if search >= node.last {
                    break;
                }
                return Some(&node.list[search]);
            }
            search -= self.bucket_size;

            current = &node.next;
        }

        None
    }
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vector = Vec::with_capacity(self.len());

        self.iterate(|node| {
            for i in 0..node.last {
                vector.push(node.list[i].clone());
            }
        });

        vector
    }
    pub fn to_vec_ref(&self) -> Vec<&T> {
        let mut vector = Vec::with_capacity(self.len());

        self.iterate(|node| {
            for i in 0..node.last {
                vector.push(&node.list[i]);
            }
        });

        vector
    }
    pub fn iter(&self) -> iter::Iter<'_, T> {
        iter::Iter::new(self)
    }
    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        iter::IterMut::new(self)
    }
}

#[macro_export]
macro_rules! SomeBox {
    ($x:expr) => {
        Some(Box::new($x))
    };
}

#[macro_export]
macro_rules! vnr [
    () => {
		VecNoRealloc::new()
	};
    ($elem:expr; $n:expr) => {
		VecNoRealloc::from_elem($elem, $n)
	};
	($ ($x:expr) , *) => {
		VecNoRealloc::<_>::from_slice(&[$ ($x) , *])
    };
];
