fn main() {
    assert_eq!(merge_sort(&vec![1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(merge_sort(&vec![3, 2, 1]), vec![1, 2, 3]);
    assert_eq!(merge_sort(&vec![1]), vec![1]);
}

fn merge_sort(v: &Vec<i32>) -> Vec<i32> {
    if v.len() < 2 {
        return v.to_vec();
    }

    let middle = v.len() / 2;

    let left = merge_sort(&v[0..middle].to_vec());
    let right = merge_sort(&v[middle..v.len()].to_vec());

    merge(&left, &right)
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
            continue;
        }

        result.push(right[j]);
        j += 1;
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}
