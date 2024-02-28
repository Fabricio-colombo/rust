fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];
    println!("Vetor antes da ordenação: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Vetor após a ordenação: {:?}", numbers);
}
