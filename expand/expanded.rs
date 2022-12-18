#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
mod fmt {
    use crate::VecNoRealloc;
    impl<T> std::fmt::Debug for VecNoRealloc<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("VecNoRealloc")
                .field("bucket_size", &self.bucket_size)
                .field("head", &self.head)
                .finish()
        }
    }
    impl<T> std::fmt::Display for VecNoRealloc<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["["], &[]))?;
            self.iterate(|node| {
                if node.last > 0 {
                    for i in 0..node.last {
                        f.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&node.list[i])],
                                ),
                            )
                            .unwrap();
                        if i < node.last - 1 {
                            f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]))
                                .unwrap();
                        }
                    }
                }
                if let Some(next) = &node.next {
                    if next.last != 0 {
                        f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]))
                            .unwrap();
                    }
                }
            });
            f.write_fmt(::core::fmt::Arguments::new_v1(&["]"], &[]))
        }
    }
}
mod index {
    use crate::VecNoRealloc;
    impl<T> std::ops::Index<usize> for VecNoRealloc<T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            let mut search = index;
            let mut current = &self.head;
            while let Some(node) = current {
                if search < self.bucket_size {
                    if search > node.last {
                        break;
                    }
                    return &node.list[search];
                }
                search -= self.bucket_size;
                current = &node.next;
            }
            { ::std::rt::begin_panic("index out of bounds") }
        }
    }
    impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            let mut search = index;
            let mut current = &mut self.head;
            while let Some(node) = current {
                if search < self.bucket_size {
                    if search > node.last {
                        break;
                    }
                    return &mut node.list[search];
                }
                search -= self.bucket_size;
                current = &mut node.next;
            }
            { ::std::rt::begin_panic("index out of bounds") }
        }
    }
}
mod iter {
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
}
mod node {
    pub(crate) struct Node<T> {
        pub(crate) list: Vec<T>,
        pub(crate) last: usize,
        pub(crate) next: Option<Box<Node<T>>>,
    }
    impl<T> Node<T> {
        pub(crate) fn with_capacity(size: usize) -> Self
        where
            T: Default + Clone,
        {
            Self {
                list: ::alloc::vec::from_elem(T::default(), size),
                last: 0,
                next: None,
            }
        }
        pub(crate) fn from_elem(elem: T, size: usize) -> Self
        where
            T: Clone,
        {
            Self {
                list: ::alloc::vec::from_elem(elem, size),
                last: size,
                next: None,
            }
        }
    }
    impl<T> std::fmt::Debug for Node<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("VNRNode")
                .field("list", &self.list)
                .field("last", &self.last)
                .field("next", &self.next)
                .finish()
        }
    }
}
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
            head: Some(Box::new(node::Node::with_capacity(size))),
        }
    }
    pub fn from_elem(elem: T, size: usize) -> Self
    where
        T: Clone,
    {
        Self {
            bucket_size: size,
            head: Some(Box::new(node::Node::from_elem(elem, size))),
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
        *current = Some(Box::new(new));
    }
    pub fn pop(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        if let Some(node) = current {
            if node.last == 0 {
                return None;
            }
        }
        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.last != 0 {
                    current = &mut node.next;
                    continue;
                }
            }
            node.last -= 1;
            return Some(unsafe {
                let ptr = node.list.as_ptr();
                let end = ptr.add(node.last);
                end.read()
            });
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
