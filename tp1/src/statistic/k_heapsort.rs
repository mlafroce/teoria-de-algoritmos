use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
	// Sort ordena de mayor a menor
	let mut min_heap = BinaryHeap::new();
	for number in numbers {
		min_heap.push(HeapminData{data: *number});
	}
	for _ in 0 .. k as usize {
		min_heap.pop();
	}
	match min_heap.pop() {
		Some(x) => x.data,
		None => 0,
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapminData {
	data: i32,
}

//impl<T> Ord for HeapminData<T> {
impl Ord for HeapminData {
	fn cmp(&self, other: &HeapminData) -> Ordering {
		other.data.cmp(&self.data)
	}
}

//impl<T> PartialOrd for HeapminData<T> {
impl PartialOrd for HeapminData {
	fn partial_cmp(&self, other: &HeapminData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}