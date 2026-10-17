pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_numbers() {
        let mut v = vec![5, 2, 9, 1, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 5, 9]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert!(v.is_empty());
    }
}
