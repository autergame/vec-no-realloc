#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
mod fmt {
    use crate::node::Node;
    use crate::VecNoRealloc;
    impl<T> std::fmt::Debug for Node<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let list: Vec<&T> = self.list.iter().take(self.last).collect();
            let ds = &mut f.debug_struct("Node");
            ds.field("list", &list);
            ds.field("last", &self.last);
            ds.field(
                "next",
                match &self.next {
                    Some(next) => next,
                    None => &None::<()>,
                },
            );
            ds.finish()
        }
    }
    impl<T> std::fmt::Debug for VecNoRealloc<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let ds = &mut f.debug_struct("VecNoRealloc");
            ds.field("bucket_size", &self.bucket_size);
            ds.field(
                "head",
                match &self.head {
                    Some(head) => head,
                    None => &None::<()>,
                },
            );
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
            self.get(index).expect("index out of bounds")
        }
    }
    impl<T> std::ops::IndexMut<usize> for VecNoRealloc<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            self.get_mut(index).expect("index out of bounds")
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
                    self.index += 1;
                    Some(unsafe { &mut *(item as *mut T) })
                }
                None => None,
            }
        }
    }
}
mod node {
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
                list: ::alloc::vec::from_elem(elem, size),
                last: size,
                next: None,
            }
        }
        pub fn push(&mut self, item: T) {
            unsafe {
                let ptr = self.list.as_mut_ptr();
                let end = ptr.add(self.last);
                end.write(item);
            }
            self.last += 1;
        }
        pub fn pop(&mut self) -> T {
            self.last -= 1;
            unsafe {
                let ptr = self.list.as_ptr();
                let end = ptr.add(self.last);
                end.read()
            }
        }
    }
    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            unsafe {
                self.list.set_len(self.last);
            }
        }
    }
    #[allow(clippy::uninit_vec)]
    fn vec_with_size<T>(size: usize) -> Vec<T> {
        let mut vec = Vec::with_capacity(size);
        unsafe {
            vec.set_len(size);
        }
        vec
    }
}
#[macro_use]
mod macros {}
pub struct VecNoRealloc<T> {
    bucket_size: usize,
    head: Option<Box<node::Node<T>>>,
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
        let mut vnr = Self::new();
        for chunk in slice.chunks(vnr.bucket_size) {
            for item in chunk {
                vnr.push(item.clone());
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
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn reserve(&mut self, additional: usize) {
        let capacity = self.capacity();
        let to_add = self.len() + additional;
        if to_add > capacity {
            let diff = to_add - capacity;
            let count = (diff as f32 / self.bucket_size as f32).ceil() as usize;
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
    pub fn pop_del(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        if let Some(node) = current {
            if node.last == 0 {
                self.head = None;
                return None;
            }
        }
        while let Some(node) = current {
            if let Some(next) = &node.next {
                if next.last != 0 {
                    current = &mut node.next;
                    continue;
                }
                node.next = None;
            }
            return Some(node.pop());
        }
        None
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
            return Some(node.pop());
        }
        None
    }
    pub fn first(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            if head.last != 0 {
                return Some(&head.list[0]);
            }
        }
        None
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        if let Some(head) = &mut self.head {
            if head.last != 0 {
                return Some(&mut head.list[0]);
            }
        }
        None
    }
    pub fn last(&self) -> Option<&T> {
        self.get(self.len() - 1)
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len() - 1)
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
impl<T> Default for VecNoRealloc<T> {
    fn default() -> Self {
        Self::new()
    }
}
