#[cfg(test)]
mod qsort_tests {
    use dsa::qsort::*;

    #[test]
    fn test_qsort() {
        let mut array = vec![5, 2, 1, 4, 3];
        qsort(&mut array);
        assert_eq!(vec![1, 2, 3, 4, 5], array);
    }

    #[test]
    fn test_qsort_edge_cases() {
        let mut array = vec![1, 2];
        qsort(&mut array);
        assert_eq!(vec![1, 2], array);

        let mut array = vec![1];
        qsort(&mut array);
        assert_eq!(vec![1], array);

        let mut array: Vec<i32> = vec![];
        let empty_vec: Vec<i32> = vec![];
        qsort(&mut array);
        assert_eq!(empty_vec, array);
    }
}
