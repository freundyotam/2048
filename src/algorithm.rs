

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




pub fn slide_left(data: &[i32]) -> (Vec<i32>, i32, bool) {
    let reversed: Vec<i32> = data.iter().rev().copied().collect();
    let (mut new_row, score, is_moving) = slide_right(&reversed);
    new_row.reverse();
    (new_row, score, is_moving)
}




pub fn transpose<const N: usize>(data: &mut [i32]) {
    for i in 0..N {
        for j in i..N {
            data.swap(i + N * j, j + N * i);
        }
    }
}

pub fn transpose_3d<const N: usize>(data: &mut [i32]) {
    for i in 0..N {
        for j in i..N {
            for k in 0..N {
                // Swap elements for transposing along the first two dimensions (i, j)
                let index1 = i + N * (j + N * k);
                let index2 = j + N * (i + N * k);
                data.swap(index1, index2);
            }
        }
    }
}
