use std::cmp::Ordering;
use std::ops::Deref;

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => match key.cmp(value) {
                // 当前值和目标值相对则返回true
                Ordering::Equal => true,
                // 如果大于目标值则向左搜索
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },
                // 如果小于目标值则向右搜索
                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                },
            },
            None => false,
        }
    }

    pub fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(key) => {
                // 小于当前节点的值插入到左边，大于当前节点的值插入到右边
                let target_node = if value < *key {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(ref mut node) => {
                        node.insert(value);
                    }
                    // 如果当前节点没有数据则新增一个节点
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.value = Some(value);
                        *target_node = Some(Box::new(node));
                    }
                }
            }
        }
    }

    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => match key.cmp(value) {
                // key > value
                Ordering::Greater => match &self.left {
                    Some(node) => node.floor(value),
                    None => None,
                },
                // key < value
                Ordering::Less => match &self.right {
                    Some(node) => {
                        let val = node.floor(value);
                        match val {
                            Some(_) => val,
                            None => Some(key),
                        }
                    }
                    None => Some(key),
                },
                Ordering::Equal => Some(key),
            },
            None => None,
        }
    }

    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => match key.cmp(value) {
                // key < value
                Ordering::Less => match &self.right {
                    Some(node) => node.ceil(value),
                    None => None,
                },
                Ordering::Greater => match &self.left {
                    Some(node) => {
                        let val = node.ceil(value);
                        match val {
                            Some(_) => val,
                            None => Some(key),
                        }
                    }
                    None => Some(key),
                },
                Ordering::Equal => Some(key),
            },
            None => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        // 深度优先将所有根节点及其左右的左节点压入stack
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            // 在遍历stack时看看当前的节点是否有右节点，如果有有则将其压入stack
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
