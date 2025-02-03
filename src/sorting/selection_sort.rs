pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..len - 1 {
        let mut min_idx = i;

        for j in (i+1)..len {
            if arr[j] < arr[min_idx] {
               min_idx = j; 
            }
        }

        arr.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_integers() {
        let mut arr:[i32; 5] = [5, 3, 8, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_selection_sort_floats() {
        let mut arr:[f64; 5] = [5.5, 3.3, 8.8, 4.4, 2.2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2.2, 3.3, 4.4, 5.5, 8.8]);
    }
    
    #[test]
    fn test_selection_sort_chars() {
        let mut arr:[char; 5] = ['e', 'c', 'a', 'd', 'b'];
        selection_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
    }
}