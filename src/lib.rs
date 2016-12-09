pub fn quicksort<T: Ord>(numbers: &mut [T]) {
    let last: usize = numbers.len() - 1;
    q_sort(numbers, 0, last);
}

fn q_sort<T: Ord>(numbers: &mut [T], left: usize, right: usize) {
    let mut l = left;
    let mut r = right;

    let p1_i = left;
    let p2_i = (left + right) / 2;
    let p3_i = right;
    let mut pivot_i = if numbers[p1_i] < numbers[p2_i] {
            if numbers[p3_i] < numbers[p2_i] {
                if numbers[p1_i] < numbers[p3_i] {
                    p3_i
                }else{
                    p1_i
                }
            }else{
                p2_i
            }
        }else{ // numbers[p2_i] <= numbers[p1_i]
            if numbers[p3_i] < numbers[p1_i] {
                if numbers[p2_i] < numbers[p3_i] {
                    p3_i
                }else{
                    p2_i
                }
            }else{
                p1_i
            }
        };

    while l < r {
        while (numbers[pivot_i] < numbers[r]) && (l < r) {
            r = r - 1;
        }
        if l != r {
            numbers.swap(pivot_i, r);
            pivot_i = r;
        }
        while (numbers[l] < numbers[pivot_i]) && (l < r) {
            l = l + 1;
        }
        if l != r {
            numbers.swap(pivot_i, l);
            pivot_i = l;
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
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*; // 外の定義にアクセスするため use
    use rand::{Rng, SeedableRng, StdRng};

    #[test]
    fn test_quicksort_number(){
        let mut numbers = [5, 3, 1, 6, 8, 4, 7, 2];
        quicksort(&mut numbers);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], numbers);
    }

    #[test]
    fn test_quicksort_str(){
        let mut strings = ["ccc", "eee", "aaa", "ddd", "bbb", "ggg", "fff"];
        quicksort(&mut strings);
        assert_eq!(["aaa", "bbb", "ccc", "ddd", "eee", "fff", "ggg"], strings);
    }

    #[test]
    fn test_quicksort_randam_vector(){
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
		let len: usize = 10000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();
        quicksort(&mut v);

        let mut prev = v[0];
        for i in 1..v.len() {
            assert!(prev < v[i]);
            prev = v[i];
        }
    }
}
