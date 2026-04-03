/*
	heap
	This question requires you to implement a binary heap function
*/
// 确实不会

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;

        // 1. 将新值添加到 items 的末尾（即 items[self.count]）
        if self.items.len() > self.count {
            self.items[self.count] = value;
        } else {
            self.items.push(value);
        }

        // 2. 向上浮动 (Sift-Up)
        let mut current_idx = self.count;

        // 只要当前节点不是根节点 (idx > 1)
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);

            // 检查当前节点和父节点是否违反堆属性
            // (即在 MinHeap 中，子节点比父节点小；在 MaxHeap 中，子节点比父节点大)
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                // 违反属性，交换
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                // 属性已满足，停止上浮
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 1. 检查是否有右子节点
        if right_idx > self.count {
            // 只有左子节点（或没有子节点，但 children_present 已经保证了至少有左子节点）
            left_idx
        } else {
            // 2. 左右子节点都存在，使用比较器判断哪个更符合堆属性
            // (self.comparator)(a, b) 为 true，则 a 是我们想要的 (e.g., MinHeap 中 a 较小)
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.is_empty() {
            return None;
        }

        // 1. 交换根节点 (index 1) 和最后一个元素 (index self.count)
        self.items.swap(1, self.count);

        // 2. 弹出并返回旧的根节点（现在在末尾）
        self.count -= 1;
        // 因为 self.items[0] 是默认值，所以我们pop掉最后一个元素是安全的
        let extracted_value = self.items.pop().unwrap_or_default();

        // 3. 向下沉降 (Sift-Down)
        let mut current_idx = 1;

        while self.children_present(current_idx) {
            // 找到最符合堆属性的子节点索引
            let target_child_idx = self.smallest_child_idx(current_idx);

            // 检查当前节点是否违反堆属性与目标子节点进行比较
            // 如果子节点比当前节点更符合堆属性 (e.g., MinHeap 中子节点更小)
            if (self.comparator)(&self.items[target_child_idx], &self.items[current_idx]) {
                // 违反属性，交换
                self.items.swap(current_idx, target_child_idx);
                current_idx = target_child_idx;
            } else {
                // 属性已满足，停止下沉
                break;
            }
        }
        Some(extracted_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}