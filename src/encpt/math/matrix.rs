use std::f64;
pub fn calc_n(chunk_len: usize) -> usize {
    let mut n = (chunk_len as f64).sqrt().round() as usize;
    let dif = n * n;
    if chunk_len > dif {
        n = n + 1
    }
    n
}

pub fn char_to_mtrx(chars: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    let final_vec: Vec<Vec<i32>> = chars
        .into_iter()
        .map(|e| e.iter().map(|c| c.parse::<i32>().unwrap()).collect())
        .collect();
    final_vec
}

// from [["145","456","789"],["741","789","123"]]

// to [[[1,4,5],[4,5,6],[7,8,9]],[[7,4,1],[7,8,9],[1,2,3]]]
pub fn fill_mtrx_gaps(n: usize, orgnl_mtrx: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let filled_mtrx: Vec<Vec<i32>> = orgnl_mtrx
        .iter()
        .map(|c| {
            let mut row = c.clone(); // Create a new row based on the original row
            if row.len() < n {
                // If the row is shorter than n, extend it with zeros
                row.extend(std::iter::repeat(0).take(n - row.len()));
            }
            row // Return the modified row
        })
        .collect();

    filled_mtrx
}

pub fn mtrx_to_vecs(mtrx: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut blue: Vec<i32> = vec![];
    let mut green: Vec<i32> = vec![];
    let mut red: Vec<i32> = vec![];
    let mut res: Vec<Vec<i32>> = vec![];

    for (i, element) in mtrx.iter().enumerate() {
        for (j, sub) in element.iter().enumerate() {
            if i > j {
                blue.push(*sub);
            } else if j > i {
                green.push(*sub);
            } else {
                red.push(*sub);
            }
        }
    }
    res.push(green);
    res.push(red);
    res.push(blue);

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::parse::str2_string_vec;

    #[test]
    fn mtrx_vals() {
        let expected = vec![vec![145, 456, 789], vec![741, 789, 123]];
        let nstd_vec = vec![vec!["145", "456", "789"], vec!["741", "789", "123"]];
        let res = char_to_mtrx(
            nstd_vec
                .iter()
                .map(|c| str2_string_vec(c.to_vec()))
                .collect(),
        );
        assert_eq!(res, expected)
    }

    #[test]
    fn calc_n_test1() {
        let e = 28;
        let res = calc_n(e);
        assert_eq!(res, 6);
    }

    #[test]
    fn calc_n_test2() {
        let e = 25;
        let res = calc_n(e);
        assert_eq!(res, 5);
    }
}
