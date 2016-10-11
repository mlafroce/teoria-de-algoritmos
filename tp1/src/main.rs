mod statistic;

fn main() {
    let my_vec = vec![7, 1, 3, 5, 2, 4, 6];
    let brute_k = statistic::brute_force::k_statistic(&my_vec, 0);
    println!("My statistic is {0}", brute_k);
}