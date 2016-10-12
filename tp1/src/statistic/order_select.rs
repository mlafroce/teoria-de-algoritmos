pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
	// Sort ordena de mayor a menor
	let mut sorted_nums = numbers.clone();
	sorted_nums.sort();
	sorted_nums[k as usize]
}