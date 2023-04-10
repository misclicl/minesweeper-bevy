use std::collections::*;

pub struct Queue<T> {
    pub items: VecDeque<T>,
}

impl<T> Queue<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

impl<T, U> From<U> for Queue<T> 
where U: IntoIterator<Item = T>
{
    fn from(iter: U) -> Self {
        Queue {
            items:VecDeque::from_iter(iter)
        }
    }
}
