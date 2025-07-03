mod ui;
mod raw_reader;
fn merge(left: &[u32], right: &[u32]) -> Vec<u32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_it = 0;
    let mut right_it = 0;

    while left_it < left.len() && right_it < right.len() {
        if left[left_it] <= right[right_it] {
            merged.push(left[left_it]);
            left_it += 1;
        } else {
            merged.push(right[right_it]);
            right_it += 1;
        }
    }

    //Copy over left over data

    while left_it < left.len() {
        merged.push(left[left_it]);
        left_it += 1;
    }
    
    while right_it < right.len() {
        merged.push(right[right_it]);
        right_it += 1;
    }

    merged
}

fn sort(vec: &[u32]) -> Vec<u32> {
    if vec.len() <= 1 {
        return vec.to_vec();
    }
    let midpoint = vec.len() / 2;
    let left = sort(&vec[0..midpoint]);
    let right = sort(&vec[midpoint..]);

    merge(&left, &right)
}

fn main() {
    let hehe: Vec<u32> = vec![7, 2, 4, 5];
    println!("{:?}", sort(&hehe))
}