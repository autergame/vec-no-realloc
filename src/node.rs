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
            list: vec![T::default(); size],
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
}

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("list", &self.list)
            .field("last", &self.last)
            .field("next", &self.next)
            .finish()
    }
}
