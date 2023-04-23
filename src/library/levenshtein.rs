use std::io::Result;

// Levenshtein Distance
pub fn lev_distance(a: &str, b: &str) -> Result<u32> {
    let mut matrix: Vec<Vec<u32>> = vec![(0..(a.len()+1) as u32).collect(); b.len()+1];

    //  TODO: make more efficient
    for i in 0..(a.len()+1) {
        matrix[0][i] = i as u32;
    }

    for j in 0..(b.len()+1) {
        matrix[j][0] = j as u32;
    }

    for j in 1..(b.len()+1) {
        for i in 1..(a.len()+1) {
            let mut sub_c = 0;
            if a.chars().nth(i-1).unwrap() != b.chars().nth(j-1).unwrap() {
                sub_c = 1;
            }

            matrix[j][i] = {*[
                matrix[j][i-1]+1, // deletion
                matrix[j-1][i]+1, // insertion
                matrix[j-1][i-1] + sub_c, // subsitution

            ].iter().min().unwrap()};

            if a.chars().nth(i) == b.chars().nth(j-1) && a.chars().nth(i-1) == b.chars().nth(j) {
                matrix[j][i] = {*[
                    matrix[j][i],
                    matrix[j-2][i-2]
                ].iter().min().unwrap()};
            }
        }
    }

    Ok(matrix[b.len()][a.len()])
}