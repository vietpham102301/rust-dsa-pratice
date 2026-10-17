use dsa::algorithms::sorting::bubble_sort;

fn main() {
    let mut v = vec![5, 2, 9, 1, 3];
    println!("Trước: {:?}", v);
    bubble_sort(&mut v);
    println!("Sau:   {:?}", v);
}