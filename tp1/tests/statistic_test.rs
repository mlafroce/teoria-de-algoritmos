extern crate tp1;

use tp1::statistic;

#[test]
fn test_brute_force_min(){
    let my_vec = vec![7, 1, 3, 5, 2, 4, 6];
    let brute_k = statistic::brute_force::k_statistic(&my_vec, 0);
    assert_eq!(brute_k, 1)
}

#[test]
fn test_brute_force_max(){
    let my_vec = vec![7, 1, 3, 5, 2, 4, 6];
    let brute_k = statistic::brute_force::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(brute_k, 7)
}