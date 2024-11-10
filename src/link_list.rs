// 定义节点的数据结构
#[derive(Debug)]
struct LinkListNode<T> {
    // 存储任意类型的数据
    data: T,
    // 指向下一个节点的 Option<Box<Node<T>>>，使用 Box 进行堆内存分配，Option 来表示节点可能为空。
    next: Option<Box<LinkListNode<T>>>,
}

// 定义链表的数据结构
#[derive(Debug)]
struct LinkList<T> {
    // 链表的头节点
    head: Option<Box<LinkListNode<T>>>,
}
impl<T> LinkList<T> {
    // 创建一个空的链表
    fn new() -> Self {
        LinkList { head: None }
    }

    // 在链表头部插入一个节点
    fn push_head(&mut self, data: T) {
        // 创建一个新的节点，并将数据赋值给它
        let new_node = Box::new(LinkListNode {
            data,
            next: self.head.take(),
        });

        // 将新节点设置为链表的头节点
        self.head = Some(new_node);
    }

    // 链表尾部插入节点,因为链表不是数组，所以需要先定位到最后一个节点，然后再插入。
    fn push(&mut self, data: T) {
        let new_node = Box::new(LinkListNode { data, next: None });

        // Create a mutable reference to head
        // 创建一个可变引用指向头节点
        let mut tail_ref = &mut self.head;

        // Traverse to the end of the list
        // 遍历链表，直到找到最后一个节点
        while let Some(ref mut tail) = tail_ref {
            tail_ref = &mut tail.next;
        }

        // Set the new node as the next of the last node
        // 将新节点设置为最后一个节点的下一个节点
        *tail_ref = Some(new_node);
    }

    // 弹出节点
    fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take();
        match head {
            Some(node) => {
                // Extract the data from the node
                // 从节点中提取数据
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod link_list_tests {
    use super::*;
    #[test]
    fn push_head_test() {
        let mut list = LinkList::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        println!("{:?}", list);
    }

    #[test]
    fn push_test() {
        let mut list = LinkList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.pop();
        println!("{:?}", list);
    }
}
