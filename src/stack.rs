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

    // 返回栈顶元素引用
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    //返回栈顶元素可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    //以下是为栈实现的迭代功能
    // 栈改变，成为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //栈不改变，返回不可变迭代器
    fn iter(&self) -> Iter<T> {
        Iter {
            stack: self.data.iter().map(|item| item).collect(),
        }
    }

    // 栈不改变，返回可变迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            stack: self.data.iter_mut().map(|item| item).collect(),
        }
    }
}

pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
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

    #[test]
    fn peek_mut_test() {
        let mut stack = Stack::<String>::new();
        stack.push("hello".to_owned());
        println!("{:?}", stack);
        let peek_mut = stack.peek_mut().unwrap();
        peek_mut.push_str(" world");
        println!("{:?}", stack);
    }

    #[test]
    fn into_iter_test() {
        let mut stack = Stack::<String>::new();
        stack.push("hello".to_owned());
        stack.push("world".to_owned());
        for item in stack.into_iter() {
            println!("{:?}", item);
        }
        // 注意这里，迭代器已经消费掉了栈，所以下面会报错
        // println!("{:?}", stack);
    }
    #[test]
    fn iter_test() {
        let mut stack = Stack::<String>::new();
        stack.push("hello".to_owned());
        stack.push("world".to_owned());
        for item in stack.iter() {
            println!("{:?}", item);
        }
        // 注意这里，迭代器不会消费掉栈，所以下面不会报错
        println!("{:?}", stack);
    }

    #[test]
    fn iter_mut_test() {
        let mut stack = Stack::<String>::new();
        stack.push("hello".to_owned());
        stack.push("world".to_owned());
        for item in stack.iter_mut() {
            item.push_str(" world");
            println!("{:?}", item);
        }
        // 注意这里，迭代器不会消费掉栈，所以下面不会报错
        println!("{:?}", stack);
    }

    // 应用场景测试
    // 括号匹配 (()()((())))
    #[test]
    fn par_check_test() {
        let mut stack = Stack::<char>::new();
        let s = "((()()((()))))".chars();
        // let s = "((()()((())))+".chars();
        // let s = "(()()((()))))".chars();

        let mut balance = true;
        s.for_each(|c| match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop().is_none() {
                    balance = false;
                }
            }
            _ => {}
        });
        if balance && stack.is_empty() {
            println!("括号匹配");
        } else {
            println!("括号不匹配");
        }
    }
}
