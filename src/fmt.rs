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
        write!(f, "[")?;

        self.iterate(|node| {
            if node.last > 0 {
                for i in 0..node.last {
                    write!(f, "{}", node.list[i]).unwrap();
                    if i < node.last - 1 {
                        write!(f, ", ").unwrap();
                    }
                }
            }
            if let Some(next) = &node.next {
                if next.last != 0 {
                    write!(f, ", ").unwrap();
                }
            }
        });

        write!(f, "]")
    }
}
