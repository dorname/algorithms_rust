/// 定义B+树的节点类型
enum NodeType {
    // 叶子节点
    Leaf,
    // 内部节点
    Internal,
}

// 定义B+树节点的结构体
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

// B+树结构体
pub struct BPTree<K, V> {
    // 根节点
    root: Option<Box<BPTreeNode<K, V>>>,
    // 树深度--阶数
    level: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init_test() {
        // let mut tree = BPTree::new(3);
    }
}
