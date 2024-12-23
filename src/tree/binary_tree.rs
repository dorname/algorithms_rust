use std::cell::RefCell;
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
}
