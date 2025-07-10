type Node<T> = Option<Box<LinkListNode<T>>>;

// 定义节点的数据结构
#[derive(Debug)]
struct LinkListNode<T> {
    // 存储任意类型的数据
    data: T,
    // 指向下一个节点的 Option<Box<Node<T>>>，
    // 使用 Box 进行堆内存分配，Option 来表示节点可能为空。
    next: Node<T>,
}

// 定义链表的数据结构
#[derive(Debug)]
struct LinkList<T> {
    // 链表的头节点
    head: Node<T>,
    // 链表长度
    lenght: usize,
}
impl<T> LinkList<T> {
    // 创建一个空的链表
    fn new() -> Self {
        LinkList {
            head: None,
            lenght: 0,
        }
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
    fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take();
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

    // 弹出链表尾部节点
    fn pop_back(&mut self) -> Option<T> {
        let len = self.len();
        let mut current = &mut self.head;
        for _ in 0..len - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }
        let tail = current.take();
        match tail {
            Some(node) => Some(node.data),
            None => None,
        }
    }

    // 插入节点
    fn insert(&mut self, data: T, index: usize) {
        // 1、判断index 是否超过链表长度了
        if index > self.lenght {
            panic!("Overflow: index > LinkList lenght");
        }

        let new_node = Box::new(LinkListNode { data, next: None });

        // 如果是插入到头部
        if index == 0 {
            let mut new_node = new_node;
            new_node.next = self.head.take();
            self.head = Some(new_node);
            self.lenght += 1;
            return;
        }

        // 找到插入位置的前一个节点
        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        // 执行插入操作
        if let Some(node) = current {
            let mut new_node = new_node;
            new_node.next = node.next.take();
            node.next = Some(new_node);
            self.lenght += 1;
        }
    }

    // 删除
    fn remove_at(&mut self, index: usize) {
        if index == 0 {
            self.pop_front();
            return;
        }

        // 找到删除位置的前一个节点
        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        //执行删除操作
        if let Some(node) = current {
            let mut remove_node = node.next.take();
            if let Some(mut re_node) = remove_node {
                node.next = re_node.next.take();
            }
            self.lenght -= 1;
        }
    }

    // 添加一个辅助方法来获取链表长度
    fn len(&self) -> usize {
        self.lenght
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
        list.pop_front();
        println!("{:?}", list);
    }

    #[test]
    fn insert_test() {
        let mut list = LinkList::new();

        // 测试在空链表中插入
        list.insert(1, 0);
        assert_eq!(list.len(), 1);

        // 测试在头部插入
        list.insert(2, 0);
        assert_eq!(list.len(), 2);

        // 测试在中间插入
        list.insert(3, 1);
        assert_eq!(list.len(), 3);

        // 测试在尾部插入
        list.insert(4, 3);
        assert_eq!(list.len(), 4);

        // 验证插入后的弹出顺序
        assert_eq!(list.pop_front(), Some(2)); // 头部
        assert_eq!(list.pop_front(), Some(3)); // 第二个
        assert_eq!(list.pop_front(), Some(1)); // 第三个
        assert_eq!(list.pop_front(), Some(4)); // 尾部
        assert_eq!(list.pop_front(), None); // 空链表
    }

    #[test]
    fn remove_at_test() {
        let mut list = LinkList::new();

        // 测试在空链表中插入
        list.insert(1, 0);
        assert_eq!(list.len(), 1);

        // 测试在头部插入
        list.insert(2, 0);
        assert_eq!(list.len(), 2);

        // 测试在中间插入
        list.insert(3, 1);
        assert_eq!(list.len(), 3);

        // 测试在尾部插入
        list.insert(4, 3);
        assert_eq!(list.len(), 4);

        println!("{:?}", list);
        list.remove_at(0);
        println!("{:?}", list);
        assert_eq!(list.pop_back(), Some(1))
    }
}
