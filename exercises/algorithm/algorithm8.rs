/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/
// I AM  DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

enum StackQueueIndex {
    Stack1,
    Stack2,
}
pub struct myStack<T> {
    //TODO
    index: StackQueueIndex,
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            //TODO
            index: StackQueueIndex::Stack1,
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        match self.index {
            StackQueueIndex::Stack1 => {
                self.q1.enqueue(elem);
                println!("q1:{:?}", self.q1);
            }
            StackQueueIndex::Stack2 => {
                self.q2.enqueue(elem);
                println!("s2:{:?}", self.q2);
            }
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.index {
            StackQueueIndex::Stack1 => {
                while self.q1.size() > 1 {
                    let elem = self.q1.dequeue().unwrap();
                    self.q2.enqueue(elem);
                }
                if self.q1.size() == 1 {
                    self.index = StackQueueIndex::Stack2;
                    return self.q1.dequeue();
                } else {
                    return Err("Stack is empty");
                }
            }
            StackQueueIndex::Stack2 => {
                while self.q2.size() > 1 {
                    let elem = self.q2.dequeue().unwrap();
                    self.q1.enqueue(elem);
                }
                if self.q2.size() == 1 {
                    self.index = StackQueueIndex::Stack1;
                    return self.q2.dequeue();
                } else {
                    return Err("Stack is empty");
                }
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.index {
            StackQueueIndex::Stack1 => {
                if self.q1.size() == 0 {
                    true
                } else {
                    false
                }
            }
            StackQueueIndex::Stack2 => {
                if self.q2.size() == 0 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
