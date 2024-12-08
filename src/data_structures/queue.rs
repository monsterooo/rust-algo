use std::collections::LinkedList;

/**
定义：
[队列的定义](https://zh.wikipedia.org/wiki/%E9%98%9F%E5%88%97)
先进先出（FIFO），跟我们去买东西排队一样
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.elements.back()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn drain(&mut self) {
        self.elements.clear();
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
        let mut queue: Queue<usize> = Queue::default();

        assert!(queue.is_empty());
        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
    }
}
