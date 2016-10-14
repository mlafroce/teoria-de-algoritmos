use std::collections::BinaryHeap;

pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
	// Sort ordena de mayor a menor
	let mut heap = BinaryHeap::new();
	for number in numbers {
		println!("HeapLen: {}", heap.len());
		if (heap.len() as i32) < k + 1 || *(heap.peek().unwrap()) > number {
			heap.push(number);
		}
		if heap.len() as i32 == k + 2 {
			heap.pop();
		}
	}
	match heap.pop() {
		Some(x) => *x,
		None => 0,
	}
}
