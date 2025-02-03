pub fn binary_search<T: PartialEq + PartialOrd>(arr: &mut[T], key: &T) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = len - 1;


    while low <= high {
        let mid: usize = low + ((high - low) / 2);

        if arr[mid] == *key {
            return Some(mid);
        } else if arr[mid] < *key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&mut arr, &3), Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&mut arr, &6), None);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let mut arr: [i32; 0] = [];
        assert_eq!(binary_search(&mut arr, &1), None);
    }

    #[test]
    fn test_binary_search_single_element_found() {
        let mut arr = [1];
        assert_eq!(binary_search(&mut arr, &1), Some(0));
    }

    #[test]
    fn test_binary_search_single_element_not_found() {
        let mut arr = [1];
        assert_eq!(binary_search(&mut arr, &2), None);
    }

    #[test]
    fn test_binary_search_first_element() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&mut arr, &1), Some(0));
    }

    #[test]
    fn test_binary_search_last_element() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&mut arr, &5), Some(4));
    }
}
