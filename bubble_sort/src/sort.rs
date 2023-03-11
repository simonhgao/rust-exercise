// advanced bubble sort function with template and PartialOrd
pub fn advanced_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// basic bubble sort function with i32
pub fn basic_bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 1..(len - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}