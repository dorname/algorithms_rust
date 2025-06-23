#[cfg(test)]
mod merge_tests {
    use std::collections::HashMap;
    #[test]
    fn test_merge() {
        // let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        // let mut nums2 = vec![2, 5, 6];
        // 合并两个有序数组的算法
        let mut maps = HashMap::new();
        maps.insert(1, 1);
        maps.insert(2, 2);
        maps.insert(3, 3);
        maps.insert(4, 4);
        maps.insert(5, 5);
        maps.insert(6, 6);
        maps.insert(1, 7);
        println!("{:?}", maps);
    }

    #[test]
    fn test() {
        struct Solution {
            nums: Vec<(i32, i32, i32)>,
            limit: usize,
        }

        impl Solution {
            pub fn new(len: usize) -> Self {
                Self {
                    nums: Vec::with_capacity(len),
                    limit: len,
                }
            }

            pub fn put(&mut self, key: i32, value: i32) {
                // 如果key存在，则更新value
                let mut index = None;
                for (i, (k, v, _)) in self.nums.iter_mut().enumerate() {
                    if *k == key {
                        *v = value;
                        index = Some(i);
                        break;
                    }
                }
                match index {
                    Some(x) => {
                        // 如果key存在，调整位置
                        self.nums.swap(0, x);
                    }
                    None => {
                        // 如果key不存在，则插入到栈顶
                        if self.nums.len() == self.limit {
                            self.nums.pop();
                        }
                        self.nums.insert(0, (key, value, 0));
                    }
                }
            }

            pub fn get(&mut self, key: i32) -> i32 {
                let res = self.nums.iter_mut().find_map(|(k, v, c)| {
                    if *k == key {
                        *c += 1;
                        Some(v.clone())
                    } else {
                        *c = 0;
                        None
                    }
                });
                match res {
                    // 根据访问次数排序
                    Some(v) => {
                        self.nums.sort_by_key(|(_, _, c)| -*c);
                        v
                    }
                    None => -1,
                }
            }
        }

        let mut solution = Solution::new(2);
        // solution.put(1, 1);
        // solution.put(2, 2);
        // assert_eq!(solution.get(1), 1);
        // solution.put(3, 3);
        // assert_eq!(solution.get(2), -1);
        // solution.put(4, 4);
        // assert_eq!(solution.get(1), -1);
        // assert_eq!(solution.get(3), 3);
        // assert_eq!(solution.get(4), 4);
        solution.put(2, 1);
        println!("{:?}", solution.nums);
        solution.put(1, 1);
        println!("{:?}", solution.nums);
        solution.put(2, 3);
        println!("{:?}", solution.nums);
        solution.put(4, 1);
        println!("{:?}", solution.nums);
        assert_eq!(solution.get(1), -1);
        assert_eq!(solution.get(2), 3);
    }
}
