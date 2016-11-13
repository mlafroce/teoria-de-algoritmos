
pub fn k_statistic(numbers: &Vec<i32>, k: i32) -> i32 {
  let mut sorted_nums = numbers.clone();
  quick_select(sorted_nums.as_mut_slice(), k as usize)
}

fn quick_select(numbers: &mut [i32], k: usize) -> i32 {
  let mut pivoted_idx = 0;
  let numbers_size = numbers.len();
  let pivot_num = numbers[numbers_size - 1];
  for idx in 0..numbers.len() -1 {
    if numbers[idx] < pivot_num {
      numbers.swap(pivoted_idx, idx);
      pivoted_idx += 1;
    }
  }
  numbers.swap(numbers_size - 1, pivoted_idx);
  if k ==  pivoted_idx {
    numbers[pivoted_idx]
  } else if k < pivoted_idx {
    let (lower, _) = numbers.split_at_mut(pivoted_idx);
    quick_select(lower, k)
  } else {
    let (_, higher) = numbers.split_at_mut(pivoted_idx + 1);
    quick_select(higher, k - pivoted_idx - 1)
  }
}
