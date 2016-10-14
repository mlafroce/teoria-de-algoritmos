use statistic::brute_force;
use statistic::order_select;
use statistic::k_select;
use statistic::k_heapsort;
use statistic::heap_select;
use statistic::quick_select;

#[test]
fn test_brute_force_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = brute_force::k_statistic(&my_vec, 0);
    assert_eq!(statistic, -5)
}

#[test]
fn test_brute_force_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = brute_force::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}

#[test]
fn test_order_select_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = order_select::k_statistic(&my_vec, 0);
    assert_eq!(statistic, -5)
}

#[test]
fn test_order_select_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = order_select::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}

#[test]
fn test_k_select_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = k_select::k_statistic(&my_vec, 0);
    assert_eq!(statistic, -5)
}

#[test]
fn test_k_select_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = k_select::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}

#[test]
fn test_k_heapsort_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = k_heapsort::k_statistic(&my_vec, 0);
    assert_eq!(statistic, -5)
}

#[test]
fn test_k_heapsort_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = k_heapsort::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}

#[test]
fn test_heap_select_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = heap_select::k_statistic(&my_vec, 0);
    assert_eq!(statistic, -5)
}

#[test]
fn test_heap_select_mid(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = heap_select::k_statistic(&my_vec, 3);
    assert_eq!(statistic, 2)
}

#[test]
fn test_heap_select_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = heap_select::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}

#[test]
fn test_quick_select_min(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = quick_select::k_statistic(&my_vec, 0 as i32);
    assert_eq!(statistic, -5)
}

#[test]
fn test_quick_select_mid(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = quick_select::k_statistic(&my_vec, 3);
    assert_eq!(statistic, 2)
}

#[test]
fn test_quick_select_max(){
    let my_vec = vec![-2, 10, 1, 3, -5, 2, 4, 6];
    let statistic = quick_select::k_statistic(&my_vec, (my_vec.len() - 1) as i32);
    assert_eq!(statistic, 10)
}
