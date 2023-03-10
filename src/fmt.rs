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
        write!(f, "[")?;

        let mut current = &self.head;

        while let Some(node) = current {
            if node.last > 0 {
                for i in 0..node.last {
                    write!(f, "{}", node.list[i])?;
                    if i < node.last - 1 {
                        write!(f, ", ")?;
                    }
                }
            }
            if let Some(next) = &node.next {
                if next.last != 0 {
                    write!(f, ", ")?;
                }
            }
            current = &node.next;
        }

        write!(f, "]")
    }
}
