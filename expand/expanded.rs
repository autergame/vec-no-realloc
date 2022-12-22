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
            let ds = &mut f.debug_struct("VecNoRealloc");
            ds.field("bucket_size", &self.bucket_size);
            if let Some(head) = &self.head {
                ds.field("head", head);
            } else {
                ds.field("head", &self.head);
            }
            ds.finish()
        }
    }
    impl<T> std::fmt::Display for VecNoRealloc<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(&["["], &[]))?;
            let mut current = &self.head;
            while let Some(node) = current {
                if node.last > 0 {
                    for i in 0..node.last {
                        f.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&node.list[i])],
                            ),
                        )?;
                        if i < node.last - 1 {
                            f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]))?;
                        }
                    }
                }
                if let Some(next) = &node.next {
                    if next.last != 0 {
                        f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]))?;
                    }
                }
                current = &node.next;
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(&["]"], &[]))
        }
    }
}
mod index {
    use crate::VecNoRealloc;
    impl<T> std::ops::Index<usize> for VecNoRealloc<T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            if let Some(item) = self.get(index) {
                item
            } else {
                { ::std::rt::begin_panic("index out of bounds") }
            }
        }
    }
    impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            if let Some(item) = self.get_mut(index) {
                item
            } else {
                { ::std::rt::begin_panic("index out of bounds") }
            }
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
            let item = self.vnr.get(self.index);
            self.index += 1;
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
            if let Some(item) = self.vnr.get_mut(self.index) {
                self.index += 1;
                Some(unsafe { &mut *(item as *mut T) })
            } else {
                None
            }
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
                list: ::alloc::vec::from_elem(elem, size),
                last: size,
                next: None,
            }
        }
        pub(crate) fn push(&mut self, item: T) {
            unsafe {
                let ptr = self.list.as_mut_ptr();
                let end = ptr.add(self.last);
                end.write(item);
            }
            self.last += 1;
        }
        pub(crate) fn pop(&mut self) -> T {
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
    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            unsafe {
                self.list.set_len(self.last);
            }
        }
    }
    fn vec_with_size<T>(size: usize) -> Vec<T> {
        let mut vec = Vec::with_capacity(size);
        unsafe {
            vec.set_len(size);
        }
        vec
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
    pub fn with_capacity(size: usize) -> Self {
        let bucket_size = size.max(2);
        Self {
            bucket_size,
            head: Some(Box::new(node::Node::with_capacity(bucket_size))),
        }
    }
    pub fn from_elem(elem: T, size: usize) -> Self
    where
        T: Clone,
    {
        let bucket_size = size.max(2);
        Self {
            bucket_size,
            head: Some(Box::new(node::Node::from_elem(elem, bucket_size))),
        }
    }
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
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
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += node.last;
            current = &node.next;
        }
        count
    }
    pub fn capacity(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += self.bucket_size;
            current = &node.next;
        }
        count
    }
    pub fn reserve(&mut self, additional: usize) {
        let capacity = self.capacity();
        let to_add = self.len() + additional;
        if to_add > capacity {
            let count = ((to_add - capacity) as f32 / self.bucket_size as f32).ceil()
                as usize;
            let mut current = &mut self.head;
            for _ in 0..count {
                while let Some(node) = current {
                    current = &mut node.next;
                }
                *current = Some(
                    Box::new(node::Node::<T>::with_capacity(self.bucket_size)),
                );
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn push(&mut self, item: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.last < self.bucket_size {
                node.push(item);
                return;
            }
            current = &mut node.next;
        }
        let mut new = node::Node::<T>::with_capacity(self.bucket_size);
        new.list[0] = item;
        new.last = 1;
        *current = Some(Box::new(new));
    }
    pub fn pop_del(&mut self, remove: bool) -> Option<T> {
        let mut current = &mut self.head;
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
            if node.last == 0 {
                if remove {
                    self.head = None;
                }
                return None;
            }
            return Some(node.pop());
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
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut search = index;
        let mut current = &mut self.head;
        while let Some(node) = current {
            if search < self.bucket_size {
                if search >= node.last {
                    break;
                }
                return Some(&mut node.list[search]);
            }
            search -= self.bucket_size;
            current = &mut node.next;
        }
        None
    }
    pub fn clear_del(&mut self) {
        self.head = None;
    }
    pub fn clear(&mut self) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            node.last = 0;
            current = &mut node.next;
        }
    }
    pub fn append(&mut self, other: &mut Self) {
        for _ in 0..other.len() {
            self.push(other.pop().unwrap());
        }
    }
    pub fn append_clone(&mut self, other: &Self)
    where
        T: Clone,
    {
        for item in other {
            self.push(item.clone());
        }
    }
    pub fn append_vec(&mut self, vector: Vec<T>) {
        for item in vector {
            self.push(item);
        }
    }
    pub fn append_vec_clone(&mut self, vector: &Vec<T>)
    where
        T: Clone,
    {
        for item in vector {
            self.push(item.clone());
        }
    }
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vector = Vec::with_capacity(self.len());
        let mut current = &self.head;
        while let Some(node) = current {
            for i in 0..node.last {
                vector.push(node.list[i].clone());
            }
            current = &node.next;
        }
        vector
    }
    pub fn to_vec_ref(&self) -> Vec<&T> {
        let mut vector = Vec::with_capacity(self.len());
        let mut current = &self.head;
        while let Some(node) = current {
            for i in 0..node.last {
                vector.push(&node.list[i]);
            }
            current = &node.next;
        }
        vector
    }
    pub fn iter(&self) -> iter::Iter<'_, T> {
        iter::Iter::new(self)
    }
    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        iter::IterMut::new(self)
    }
}
