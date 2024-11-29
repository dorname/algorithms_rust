use std::collections::btree_map::{Keys, Values};

/// 定义B+树的节点类型
#[derive(Debug)]
enum NodeType {
    // 叶子节点
    Leaf,
    // 内部节点
    Internal,
}

// 定义B+树节点的结构体
#[derive(Debug)]
pub struct BPTreeNode<K, V> {
    // 节点类型
    node_type: NodeType,
    // 索引值数组
    keys: Vec<K>,
    // 值数组
    values: Vec<V>,
    // 子节点数组 使用Box
    childrens: Vec<Box<BPTreeNode<K, V>>>,
    // 链接节点
    next: Option<Box<BPTreeNode<K, V>>>,
}

impl<K, V> BPTreeNode<K, V> {
    fn new(node_type: NodeType, keys: Vec<K>, values: vec![]) -> Self {
        BPTreeNode {
            node_type: NodeType::Leaf,
            keys: Vec::new(),
            values: Vec::new(),
            childrens: Vec::new(),
            next: None,
        }
    }
}

// B+树结构体
#[derive(Debug)]
pub struct BPTree<K, V> {
    // 根节点
    root: Option<Box<BPTreeNode<K, V>>>,
    // 树深度--阶数
    limit: usize,
}

impl<K, V> BPTree<K, V> {
    fn new(limit: usize) -> Self {
        BPTree { root: None, limit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_test() {
        let tree: BPTree<u16, u16> = BPTree::new(3);
        println!("{:?}", tree);
    }
}
