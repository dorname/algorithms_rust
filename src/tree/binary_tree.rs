use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
/**
 * 二叉树节点结构体
 */
struct TreeNode {
    val: i32,                             // 节点值
    left: Option<Rc<RefCell<TreeNode>>>,  // 左子树
    right: Option<Rc<RefCell<TreeNode>>>, // 右子树
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeNode")
            .field("leftNode", &self.left)
            .field("rightNode", &self.right)
            .field("val", &self.val)
            .finish()
    }
}

impl TreeNode {
    /**
     * 构造方法
     */
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }

    /**
     * 插入节点
     */
    pub fn insert(&mut self, left: Rc<RefCell<TreeNode>>, right: Rc<RefCell<TreeNode>>) {
        self.left = Some(left);
        self.right = Some(right);
    }
}
/**
 * 层序遍历
 */
pub fn level_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut result = Vec::new();
    while let Some(node) = queue.pop_front() {
        result.push(node.borrow().val);
        if let Some(left) = node.borrow().left.clone() {
            // 左子树入队
            queue.push_back(left);
        }
        if let Some(right) = node.borrow().right.clone() {
            // 右子树入队
            queue.push_back(right);
        }
    }
    result
}

/**
 * 前序优先遍历
 */
fn pre_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        // 根->左->右
        if let Some(node) = root {
            let node = node.borrow();
            res.push(node.val);
            dfs(node.left.as_ref(), res);
            dfs(node.right.as_ref(), res);
        }
    }
    dfs(root, &mut result);
    result
}

/**
 * 中序遍历
 */
fn in_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        // 左->根->右
        if let Some(node) = root {
            let node = node.borrow();
            dfs(node.left.as_ref(), res);
            res.push(node.val);
            dfs(node.right.as_ref(), res);
        }
    }
    dfs(root, &mut result);
    result
}

/**
 * 后序遍历
 */
fn post_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        // 左->右->根
        if let Some(node) = root {
            let node = node.borrow();
            dfs(node.left.as_ref(), res);
            dfs(node.right.as_ref(), res);
            res.push(node.val);
        }
    }
    dfs(root, &mut result);
    result
}
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let root = TreeNode::new(1);
        let n1: Rc<RefCell<TreeNode>> = TreeNode::new(2);
        let n2 = TreeNode::new(3);
        let n3 = TreeNode::new(4);
        let n4 = TreeNode::new(5);

        // root.borrow_mut().left = Some(n1);
        // root.borrow_mut().right = Some(n2.clone());
        root.borrow_mut().insert(n1, n2.clone());
        println!("{:?}", root);
        n2.borrow_mut().insert(n3, n4);
        // n2.borrow_mut().left = Some(n3);
        // n2.borrow_mut().right = Some(n4);
        println!("{:?}", root);
    }

    #[test]
    fn level_order_test() {
        let root = TreeNode::new(1);
        let n1: Rc<RefCell<TreeNode>> = TreeNode::new(2);
        let n2 = TreeNode::new(3);
        let n3 = TreeNode::new(4);
        let n4 = TreeNode::new(5);
        root.borrow_mut().insert(n1, n2.clone());
        n2.borrow_mut().insert(n3, n4);
        assert_eq!(level_order(&root), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn pre_order_test() {
        let root = TreeNode::new(1);
        let n1: Rc<RefCell<TreeNode>> = TreeNode::new(2);
        let n2 = TreeNode::new(3);
        let n3 = TreeNode::new(4);
        let n4 = TreeNode::new(5);
        root.borrow_mut().insert(n1, n2.clone());
        n2.borrow_mut().insert(n3, n4);
        assert_eq!(pre_order(Some(&root)), vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn in_order_test() {
        let root = TreeNode::new(1);
        let n1: Rc<RefCell<TreeNode>> = TreeNode::new(2);
        let n2 = TreeNode::new(3);
        let n3 = TreeNode::new(4);
        let n4 = TreeNode::new(5);
        root.borrow_mut().insert(n1, n2.clone());
        n2.borrow_mut().insert(n3, n4);
        assert_eq!(in_order(Some(&root)), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn post_order_test() {
        let root: Rc<RefCell<TreeNode>> = TreeNode::new(1);
        let n1: Rc<RefCell<TreeNode>> = TreeNode::new(2);
        let n2 = TreeNode::new(3);
        let n3 = TreeNode::new(4);
        let n4 = TreeNode::new(5);
        root.borrow_mut().insert(n1, n2.clone());
        n2.borrow_mut().insert(n3, n4);
        assert_eq!(post_order(Some(&root)), vec![2, 4, 5, 3, 1]);
    }
}
