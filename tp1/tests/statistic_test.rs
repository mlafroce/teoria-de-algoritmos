extern crate tp1;

use tp1::statistic;

#[test]
fn test_brute_force_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let brute_k = statistic::brute_force::k_statistic(&my_vec, 0);
    assert_eq!(brute_k, -5)
}

#[test]
fn test_brute_force_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let brute_k = statistic::brute_force::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(brute_k, 10)
}

#[test]
fn test_order_select_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let brute_k = statistic::order_select::k_statistic(&my_vec, 0);
    assert_eq!(brute_k, -5)
}

#[test]
fn test_order_select_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let brute_k = statistic::order_select::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(brute_k, 10)
}
