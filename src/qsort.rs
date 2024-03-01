pub fn qsort<T: Ord + Copy>(values: &mut [T]) {
    qsort_internal(values, 0, (values.len() as isize) - 1)
}

fn qsort_internal<T: Ord + Copy>(values: &mut [T], start: isize, end: isize) {
    if start >= end {
        return;
    }

    // goofy ah conversions
    let index = partition(values, start as usize, end as usize) as isize;

    qsort_internal(values, start, index - 1);
    qsort_internal(values, index + 1, end);
}

fn partition<T: Ord + Copy>(values: &mut [T], start: usize, end: usize) -> usize {
    let mut pivot_index = start;
    let pivot_value = values[end];

    for i in start..end {
        if values[i] < pivot_value {
            values.swap(i, pivot_index);
            pivot_index += 1;
        }
    }

    values.swap(end, pivot_index);
    pivot_index
}
