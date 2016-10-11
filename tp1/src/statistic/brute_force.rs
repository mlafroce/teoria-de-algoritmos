pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
	let mut found = false;
	let mut statistic = 0;
	while !found {
		found = validate_statistic(numbers, numbers[statistic], k);
		statistic += 1;
	}
	statistic -= 1;
	numbers[statistic]
}

fn validate_statistic(numbers: &Vec<i32>, candidate: i32, k: i32) -> bool {
	let mut number_of_minors = 0;
	for &number in numbers {
		if number < candidate {
			number_of_minors += 1;
		}
	}
	number_of_minors == k
}