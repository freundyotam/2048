// #[cfg(test)]
// mod slide_test {
//     use super::slide_left;
//     use super::slide_right;
//     #[test]
//     fn test_slide_right_with_one_element() {
//         assert_eq!((vec![0, 0, 0, 1], 0), slide_right(&[0, 1, 0, 0]));
//     }
//     #[test]
//     fn test_slide_left_with_one_element() {
//         assert_eq!((vec![1, 0, 0, 0], 0), slide_left(&[0, 1, 0, 0]));
//     }
//     #[test]
//     fn test_slide_right_with_two_different_elements() {
//         assert_eq!((vec![0, 0, 1, 2], 0), slide_right(&[1, 0, 2, 0]));
//     }
//     #[test]
//     fn test_slide_left_with_two_different_elements() {
//         assert_eq!((vec![1, 2, 0, 0], 0), slide_left(&[1, 0, 2, 0]));
//     }
//     #[test]
//     fn test_slide_right_with_two_same_elements() {
//         assert_eq!((vec![0, 0, 0, 2], 4), slide_right(&[1, 0, 1, 0]));
//     }
//     #[test]
//     fn test_slide_left_with_two_same_elements() {
//         assert_eq!((vec![2, 0, 0, 0], 4), slide_left(&[1, 0, 1, 0]));
//     }
//     #[test]
//     fn test_slide_right_with_three_same_elements() {
//         assert_eq!((vec![0, 0, 1, 2], 4), slide_right(&[1, 0, 1, 1]));
//     }
//     #[test]
//     fn test_slide_left_with_three_same_elements() {
//         assert_eq!((vec![2, 1, 0, 0], 4), slide_left(&[1, 0, 1, 1]));
//     }
//     #[test]
//     fn test_slide_right_with_three_different_elements() {
//         assert_eq!((vec![0, 0, 2, 2], 4), slide_right(&[1, 0, 1, 2]));
//         assert_eq!((vec![0, 2, 1, 2], 0), slide_right(&[2, 0, 1, 2]));
//         assert_eq!((vec![0, 0, 2, 2], 4), slide_right(&[0, 1, 1, 2]));
//     }
//     #[test]
//     fn test_slide_left_with_three_different_elements() {
//         assert_eq!((vec![2, 2, 0, 0], 4), slide_left(&[1, 0, 1, 2]));
//         assert_eq!((vec![2, 1, 2, 0], 0), slide_left(&[2, 0, 1, 2]));
//         assert_eq!((vec![2, 2, 0, 0], 4), slide_left(&[0, 1, 1, 2]));
//     }
//     #[test]
//     fn test_slide_right_with_four_same_elements() {
//         assert_eq!((vec![0, 0, 2, 2], 8), slide_right(&[1, 1, 1, 1]));
//     }
//     #[test]
//     fn test_slide_left_with_four_same_elements() {
//         assert_eq!((vec![2, 2, 0, 0], 8), slide_left(&[1, 1, 1, 1]));
//     }
//     #[test]
//     fn test_slide_right_with_four_different_elements() {
//         assert_eq!((vec![1, 2, 1, 2], 0), slide_right(&[1, 2, 1, 2]));
//     }
//     #[test]
//     fn test_slide_left_with_four_different_elements() {
//         assert_eq!((vec![1, 2, 1, 2], 0), slide_left(&[1, 2, 1, 2]));
//     }
// }


pub fn merge_backward(slice: &mut [i32]) -> i32 {
    if slice.len() != 2 {
        return 0;
    }
    
    if slice[0] == slice[1] && slice[0] != 0 {
        slice[1] *= 2;  // Merge into the right tile
        slice[0] = 0;   // Clear the left tile
        return slice[1]; // Return the score gained from merging
    }

    0 // No merge happened
}



// fn merge_backward(slice: &mut [i32]) -> i32 {
//     if slice[0] == slice[1] && slice[1] != 0 {
//         slice[0] = 0;
//         slice[1] += 1;
//         2i32.pow(slice[1] as u32)
//     } else {
//         0
//     }
// }

fn stable_partition<T, I, F>(slice: I, pred: F) -> Vec<T>
where
    T: Copy,
    I: IntoIterator<Item = T>,
    for<'r> F: Fn(&'r T) -> bool,
{
    let (mut left, right): (Vec<T>, Vec<T>) = slice.into_iter().partition(pred);
    left.extend(right.iter());
    left
}


pub fn slide_right(data: &[i32]) -> (Vec<i32>, i32, bool) {
    let mut row: Vec<i32> = data.iter().copied().filter(|&x| x != 0).collect(); // Remove zeros
    let mut score = 0;
    let mut is_moving = false;

    let mut i = row.len();
    
    while i > 1 {
        if row[i - 1] == row[i - 2] { // If adjacent tiles are equal, merge them
            row[i - 1] += 1;
            score += row[i - 1]; // Add to score
            row[i - 2] = 0; // Clear merged tile
            i -= 1; // Skip the next tile to prevent double merging
        }
        i -= 1;
    }

    let mut result: Vec<i32> = row.into_iter().filter(|&x| x != 0).collect(); // Remove extra zeros
    while result.len() < data.len() {
        is_moving = true;
        result.insert(0, 0); // Fill empty spaces with zeros on the left
    }

    (result, score, is_moving)
}


// pub fn slide_right(data: &[i32]) -> (Vec<i32>, i32) {
//     let mut ret = stable_partition(data.iter().copied(), |x| *x == 0);
//     let mut index = data.len();
//     let mut score = 0;
//     while index > 1 {
//         score += merge_backward(&mut ret[index - 2..index]);
//         index -= 1;
//     }
//     (stable_partition(ret.iter().copied(), |x| *x == 0), score)
// }


pub fn slide_left(data: &[i32]) -> (Vec<i32>, i32, bool) {
    let reversed: Vec<i32> = data.iter().rev().copied().collect();
    let (mut new_row, score, is_moving) = slide_right(&reversed);
    new_row.reverse();
    (new_row, score, is_moving)
}


// pub fn slide_left(data: &[i32]) -> (Vec<i32>, i32) {
//     let ret = data.iter().rev().copied().collect::<Vec<_>>();
//     let (data, score) = slide_right(&ret);
//     (data.iter().rev().copied().collect::<Vec<_>>(), score)
// }


pub fn transpose<const N: usize>(data: &mut [i32]) {
    for i in 0..N {
        for j in i..N {
            data.swap(i + N * j, j + N * i);
        }
    }
}
