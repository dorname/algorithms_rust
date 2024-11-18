#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
            size: 0,
        }
    }
    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let mut stack = Stack::<&str>::new();
        stack.data.push("hello");
        println!("{:?}", stack);
    }

    #[test]
    fn push_test() {
        let mut stack = Stack::<&str>::new();
        stack.push("hello");
        println!("{:?}", stack);
    }

    #[test]
    fn pop_test() {
        let mut stack = Stack::<&str>::new();
        stack.push("hello");
        println!("{:?}", stack);
        println!("{:?}", stack.pop());
        println!("{:?}", stack);
    }

    #[test]
    fn peek_test() {
        let mut stack = Stack::<String>::new();
        stack.push("hello".to_owned());
        println!("{:?}", stack);
        println!("{:?}", stack.peek());
    }
}
