mod evm_stack;
mod link_list;
mod merge;
mod stack;
mod tree;
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_hash_map() {
        // let mut map = HashMap::with_capacity(2);
        // map.insert("key1", "value1");
        // map.insert("key2", "value2");
        // map.insert("key1", "value3");
        // println!("{:?}", map);
        struct LRUCache {
            // 全局的缓存索引
            // key (value,prev,next)
            caches: HashMap<i32, (i32, i32, i32)>,
            // 大小控制
            capacity: usize,
            // 时间序号
            head_key: i32,
            // 访问顺序
            tail_key: i32,
        }

        /**
         * `&self` means the method takes an immutable reference.
         * If you need a mutable reference, change it to `&mut self` instead.
         */
        impl LRUCache {
            fn new(capacity: i32) -> Self {
                Self {
                    caches: HashMap::<i32, (i32, i32, i32)>::new(),
                    capacity: capacity as usize,
                    head_key: -1,
                    tail_key: -1,
                }
            }

            fn get(&mut self, key: i32) -> i32 {
                if key.eq(&self.tail_key) {
                    return match self.caches.get(&self.tail_key) {
                        Some(&(value, _, _)) => value,
                        None => -1,
                    };
                }
                if let Some(&(value, prev_key, next_key)) = self.caches.get(&key) {
                    if let Some(&(val, _, nxt_key)) = self.caches.get(&next_key) {
                        if key.eq(&self.head_key) {
                            self.caches.insert(next_key, (val, -1, nxt_key));
                            self.head_key = next_key;
                        } else {
                            self.caches.insert(next_key, (val, prev_key, nxt_key));
                            if key.ne(&self.tail_key) {
                                if let Some(&(pv, pp, _)) = self.caches.get(&prev_key) {
                                    self.caches.insert(prev_key, (pv, pp, next_key));
                                }
                            }
                        }
                    }
                    // 如果数据存在则频次累加，然后调整前置指针，指向尾部节点
                    self.caches.insert(key, (value, self.tail_key, -1));

                    // 调整原本的尾部节点,节点的后置指针指向key
                    if let Some(&(val_tail, prev_key, _)) = self.caches.get(&self.tail_key) {
                        self.caches.insert(self.tail_key, (val_tail, prev_key, key));
                    }
                    self.tail_key = key;
                    value
                } else {
                    -1
                }
            }

            fn put(&mut self, key: i32, value: i32) {
                //1、存储为空
                if self.caches.is_empty() {
                    // 直接插入
                    self.caches.insert(key, (value, -1, -1));
                    self.head_key = key;
                    self.tail_key = key;
                    return;
                }

                //2、key == tail
                if self.tail_key.eq(&key) {
                    if let Some(&(_, p, n)) = self.caches.get(&key) {
                        self.caches.insert(key, (value, p, n));
                        return;
                    }
                }
                //2、如果存储不为空且空间溢出了
                if (!self.caches.contains_key(&key)) && (self.caches.len() + 1 > self.capacity) {
                    if let Some(&(_, _, next)) = self.caches.get(&self.head_key) {
                        // head.next.prev = -1
                        if let Some(&(sec_val, _, sec_next)) = self.caches.get(&next) {
                            self.caches.insert(next, (sec_val, -1, sec_next));
                        }
                        // remove head
                        self.caches.remove(&self.head_key);
                        self.head_key = next;
                    }
                    // key.prev = tail,key.next = -1
                    self.caches.insert(key, (value, self.tail_key, -1));
                    // tail.next = key
                    if let Some(&(tail_val, tail_prev, _)) = self.caches.get(&self.tail_key) {
                        self.caches
                            .insert(self.tail_key, (tail_val, tail_prev, key));
                    }
                    self.tail_key = key;
                    return;
                } else {
                    if let Some(&(_, _, nxt_key)) = self.caches.get(&key) {
                        if let Some(&(nv, np, nn)) = self.caches.get(&nxt_key) {
                            // key==head
                            if key.eq(&self.head_key) {
                                //key.next.prev = -1
                                self.caches.insert(nxt_key, (nv, -1, nn));
                                self.head_key = nxt_key;
                            } else {
                                self.caches.insert(nxt_key, (nv, np, nn));
                            }
                        }
                    }
                    // key.prev = tail,key.next = -1
                    self.caches.insert(key, (value, self.tail_key, -1));
                    // tail.next = key
                    if let Some(&(tail_val, tail_prev, _)) = self.caches.get(&self.tail_key) {
                        self.caches
                            .insert(self.tail_key, (tail_val, tail_prev, key));
                    }
                    // tail = key
                    self.tail_key = key;
                }
            }
        }

        let mut lru = LRUCache::new(3);
        lru.put(1, 1);
        lru.put(2, 2);
        lru.put(3, 3);
        lru.put(4, 4);
        println!("{:?}", lru.caches);
        lru.get(4);
        println!("{:?}", lru.caches);
        lru.get(3);
        println!("{:?}", lru.caches);
        lru.get(2);
        println!("{:?}", lru.caches);
        lru.get(1);
        println!("{:?}", lru.caches);
        lru.put(5, 5);
        lru.get(1);
        lru.get(2);
        lru.get(3);
        lru.get(4);
        lru.get(5);
        println!("{:?}", lru.caches);
    }
}
