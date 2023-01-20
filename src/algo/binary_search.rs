use std::cmp::Ordering;

///
///
/// # Arguments
///
/// * `arr`: Исходный массив
/// * `x`: Искомое число
///
/// returns: bool
///
/// # Examples
///
/// ```
/// let arr = [1, 3, 4, 6, 10, 11];
/// let is_success_10 = binary_search(arr, 10); // true
///
/// let is_success_2 = binary_search(arr, 2); // fale
/// ```
fn binary_search(arr: &[i32], x: i32) -> bool {
    if arr.is_empty() { return false; }
    let mid_v = arr.len() / 2;
    let sub_slice = match arr[mid_v].cmp(&x) {
        Ordering::Less => &arr[mid_v + 1..],
        Ordering::Equal => return true,
        Ordering::Greater => &arr[..mid_v],
    };
    binary_search(sub_slice, x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_true() {
        let arr = [1, 3, 4, 5, 6, 7];
        assert!(binary_search(arr, 4) == true);

        assert!(binary_search(arr, 7) == true);
    }

    #[test]
    fn test_search_false() {
        let arr = [1, 3, 4, 5, 6, 7];
        assert!(binary_search(arr, 2) == false);

        assert!(binary_search(arr, 9) == false);
    }
}