
fn main() {

    let mut data: Vec<i32> = vec![100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 2, 6, 9, 19, 32, 31];
    println!("Bubble Sort");
    println!("{:?}", data);
    bubble_sort(&mut data);
    println!("{:?}", data);
}

fn bubble_sort(numbs: &mut Vec<i32>) {
    let l = numbs.len();
    let mut left: i32;
    let mut right: i32;
    for i in 0..l {
        for r in 0..(l - i - 1) {
            left = numbs[r];
            right = numbs[r + 1];
            if left > right {
                numbs[r] = right;
                numbs[r + 1] = left;
            }
        }
    }
}
