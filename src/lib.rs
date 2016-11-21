pub fn quicksort<T: Ord>(numbers: &mut [T]) {
    let last: usize = numbers.len() - 1;
    q_sort(numbers, 0, last);
}

fn q_sort<T: Ord>(numbers: &mut [T], left: usize, right: usize) {
    let mut pivot_i = left;
    let mut l = left;
    let mut r = right;
    while l < r {
        while (numbers[pivot_i] <= numbers[r]) && (l < r) {
            r = r - 1;
        }
        if l != r {
            numbers.swap(pivot_i, r);
            pivot_i = r;
            l = l + 1;
        }
        while (numbers[l] <= numbers[pivot_i]) && (l < r) {
            l = l + 1;
        }
        if l != r {
            numbers.swap(pivot_i, l);
            pivot_i = l;
            r = r - 1;
        }
    }
    if left < l {
        q_sort(numbers, left, l - 1);
    }
    if right > l {
        q_sort(numbers, l + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 外の定義にアクセスするため use

    #[test]
    fn test_quicksort(){
        let mut numbers = [5, 3, 1, 6, 8, 4, 7, 2];
        quicksort(&mut numbers);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], numbers);
    }
}
