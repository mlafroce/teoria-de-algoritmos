pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
	// Sort ordena de mayor a menor
	let mut nums_cpy = numbers.clone();
	let mut min_idx = 0;
	for current_k in 0 .. (k+1) as usize {
		for idx in current_k .. numbers.len() {
			if numbers[idx] < numbers[min_idx] {
				min_idx = idx
			}
		}
		let aux = nums_cpy[min_idx];
		nums_cpy[min_idx] = nums_cpy[current_k];
		nums_cpy[current_k] = aux;
	}
	for (idx, item) in nums_cpy.iter().enumerate() {
		println!("Vector[{}]: {}", idx, item);
	}
	nums_cpy[k as usize]
}