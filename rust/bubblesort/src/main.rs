
fn main() {

    let mut data: Vec<i32> = vec![100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 2, 6, 9, 19, 32, 31];
    println!("{:?}", data);
    bubblesort(&mut data);
    println!("{:?}", data);
}

fn bubblesort(numbs: &mut [i32]) {
    let l: u32 = numbs.len() as u32;
    let mut left: i32;
    let mut right: i32;
    for i in 0..l {
        for r in 0..(l as i32 - i as i32 - 1) {
            left = numbs[r as usize];
            right = numbs[(r as u32 + 1) as usize];
            if left > right {
                numbs[r as usize] = right;
                numbs[(r as u32 + 1) as usize] = left;
            }
        }
    }
}
