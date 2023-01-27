use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Default)]
pub struct ListNode {
    pub val: i64,
    pub next: Link,
}

impl ListNode {
    pub fn new(val: i64) -> Self {
        ListNode { next: None, val }
    }

    pub fn with_next(mut self, next: ListNode) -> Self {
        self.next = next.into();
        self
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list: Vec<i64> = Vec::default();
        list.push(self.val);

        let mut cur = &self.next;
        while let Some(node) = cur {
            list.push(node.val);
            cur = &node.next;
        }

        f.write_str("LinkedList")?;
        list.fmt(f)
    }
}

pub type Link = Option<Rc<ListNode>>;

impl From<ListNode> for Link {
    fn from(node: ListNode) -> Self {
        Some(Rc::new(node))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LinkWrapper(Link);

impl LinkWrapper {
    pub fn unwrap(self) -> Link {
        self.0
    }
}

impl From<ListNode> for LinkWrapper {
    fn from(node: ListNode) -> Self {
        Self(Some(Rc::new(node)))
    }
}

impl From<Option<ListNode>> for LinkWrapper {
    fn from(node: Option<ListNode>) -> Self {
        Self(node.and_then(|x| Some(Rc::new(x))))
    }
}

impl From<Link> for LinkWrapper {
    fn from(node: Link) -> Self {
        Self(node)
    }
}

impl Deref for LinkWrapper {
    type Target = Link;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<i64>> for LinkWrapper {
    fn from(elems: Vec<i64>) -> Self {
        let list = elems
            .iter()
            .rfold(None, |acc, &x| ListNode { val: x, next: acc }.into());

        list.into()
    }
}

#[macro_export]
macro_rules! linked_list {
    ($($x:expr),*) => {
        LinkWrapper::from(vec![$($x),*]).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_test() {
        assert_eq!(linked_list![], None);
        assert_eq!(
            linked_list![1, 2, 3, 4, 5],
            ListNode::new(1)
                .with_next(ListNode::new(2).with_next(
                    ListNode::new(3).with_next(ListNode::new(4).with_next(ListNode::new(5)))
                ))
                .into()
        );
    }
}
