mod algorithms;
pub use algorithms::*;


fn main() {
    binary_search();
}

fn binary_search() {
    let list = [1, 5, 9, 12, 20, 21, 23, 30, 50, 69, 77, 101, 420, 1024];
    let input = 69;

    println!("----- Binary search -----");
    binary_search::run(&list, input);
}
