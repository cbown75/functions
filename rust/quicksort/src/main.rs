fn main() {

    let mut data: Vec<i32> = vec![100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 2, 6, 9, 19, 32, 31];
    println!("{:?}", data);
    let high = data.len()-1;
    quicksort(&mut data, 0 as usize, high);
    println!("{:?}", data);
}


fn quicksort(numbs: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot = part(numbs, low, high);

        quicksort(numbs, low, pivot - 1);

        quicksort(numbs, pivot + 1, high);
    }


}


fn part(numbs: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = numbs[high];
    let mut l = low;

    for i in low..high {
        if numbs[i] < pivot {
            let sl = numbs[l];
            let si = numbs[i];
            numbs[i] = sl;
            numbs[l] = si;
            l = l + 1;
        }
    }
    let nl = numbs[l];
    let nh = numbs[high];
    numbs[l] = nh;
    numbs[high] = nl;

    return l;
}
