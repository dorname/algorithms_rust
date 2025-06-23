use core::borrow;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

/**
 * AVL 树节点结构体
 */
struct AVLNode {
    val: i32,
    height: i32,
    left: OptionAVLNodeRc,
    right: OptionAVLNodeRc,
}

type OptionAVLNodeRc = Option<Rc<RefCell<AVLNode>>>;

impl AVLNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            height: 0,
            left: None,
            right: None,
        }))
    }
    fn height(node: OptionAVLNodeRc) -> i32 {
        match node {
            Some(n) => n.as_ref().borrow().height,
            None => -1,
        }
    }
    fn update_height(node: OptionAVLNodeRc) {
        if let Some(node) = node {
            let left_height = AVLNode::height(node.as_ref().borrow().left.clone());
            let right_height = AVLNode::height(node.as_ref().borrow().right.clone());
            node.borrow_mut().height = left_height.max(right_height) + 1;
        }
    }

    fn balance_factor(node: OptionAVLNodeRc) -> i32 {
        match node {
            Some(node) => {
                let left_height = AVLNode::height(node.as_ref().borrow().left.clone());
                let right_height = AVLNode::height(node.as_ref().borrow().right.clone());
                left_height - right_height
            }
            None => 0,
        }
    }

    // /**
    //  * 左旋转
    //  */
    // fn left_rotate(mut node: OptionAVLNodeRc) -> OptionAVLNodeRc {
    //     match node {
    //         Some(node) => {
    //             let child = node.as_ref().borrow().left.clone().unwrap();
    //             let grand_child = child.as_ref().borrow().right.clone();
    //         }
    //         None => None,
    //     }
    // }
}

mod tests {
    use super::*;
    #[test]
    fn test_avl_tree() {
        let a = 1;
        let b = 2;
        println!("max:{}", a.max(b));
    }
}
