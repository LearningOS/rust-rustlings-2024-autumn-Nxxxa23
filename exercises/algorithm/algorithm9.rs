/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
    index: usize
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
            index: 1
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) where T: Clone
    {
        //TODO

        self.items.push(value);
        self.count += 1;

        if (self.count < 2)
        {
            return;
        }

        let mut p: usize = self.count;
        let mut parentidx: usize;

        while (p != 1)
        {
            parentidx = self.parent_idx(p);
            if (!(self.comparator)(&self.items[parentidx], &self.items[p]))
            {
                self.index = 1;
                let temp: T = self.items[parentidx].clone();
                self.items[parentidx] = self.items[p].clone();
                self.items[p] = temp;
            }
            p = parentidx;
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

    fn smallest_child_idx(&self, idx: usize) -> usize where T: Clone + Ord
    {
        //TODO
        if (self.count == 0)
        {
            return 0;
        }

        if (self.comparator == (|a, b| a < b))
        {
            return 1;
        }

        let mut minvalue: T = self.items[1].clone();
        let mut index: usize = 1;
        for i in 1..=self.count
        {
            if (self.items[i] < minvalue)
            {
                minvalue = self.items[i].clone();
                index = i;
            }
        }

        return index;
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
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        //TODO
        if (self.is_empty())
        {
            return None;
        }

        self.index += 1;

        if (self.index > self.count)
        {
            self.index = 1;
            return Some(self.items[self.count].clone());
        }
        Some(self.items[self.index - 1].clone())
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