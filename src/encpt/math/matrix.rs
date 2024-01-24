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

pub fn fill_mtrx_gaps(n: usize, orgnl_mtrx: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let filled_mtrx: Vec<Vec<i32>> = orgnl_mtrx
        .iter()
        .map(|c| {
            let mut row = c.clone(); // Create a new row based on the original row
            if row.len() < n {
                // If the row is shorter than n, extend it with zeros
                row.extend(std::iter::repeat(101).take(n - row.len()));
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

    let res: Vec<Vec<i32>> = vec![green, red, blue];

    res
}

pub fn create_virt_mtrx(n: usize) -> Vec<Vec<i32>> {
    let mut nested_vector: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let inner_vector: Vec<i32> = vec![0; n];

        nested_vector.push(inner_vector);
    }

    nested_vector
}

pub fn vecs_to_mtrx(mtrx: Vec<Vec<i32>>) -> Vec<i32> {
    let mut green = mtrx[0].clone();
    let mut red = mtrx[1].clone();
    let mut blue = mtrx[2].clone();

    let virt_mtrx: Vec<Vec<i32>> = create_virt_mtrx(red.len());
    // if n = 5 :
    // [[0, 0, 0, 0, 0],
    //  [0, 0, 0, 0, 0],
    //  [0, 0, 0, 0, 0],
    //  [0, 0, 0, 0, 0],
    //  [0, 0, 0, 0, 0]]

    let mut res: Vec<i32> = vec![];
    for (i, element) in virt_mtrx.iter().enumerate() {
        for (j, _sub) in element.iter().enumerate() {
            if i < j && !green.is_empty() {
                res.push(green.remove(0));
            } else if j < i && !blue.is_empty() {
                res.push(blue.remove(0));
            } else if !red.is_empty() {
                res.push(red.remove(0));
            }
        }
    }
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

    #[test]
    fn test_rvrs_vec_to_mtrx() {
        let mtrx = vec![
            vec![165, 314, 671, 113, 923, 314, 194, 422, 652, 422],
            vec![652, 103, 314, 103, 0],
            vec![422, 113, 389, 923, 113, 194, 652, 389, 0, 0],
        ];

        let expected = vec![
            652, 165, 314, 671, 113, 422, 103, 923, 314, 194, 113, 389, 314, 422, 652, 923, 113,
            194, 103, 422, 652, 389, 0, 0, 0,
        ];

        let res = vecs_to_mtrx(mtrx.clone());
        // vecs_to_mtrx(mtrx);
        assert_eq!(res, expected);
    }

    #[test]

    fn test_pop() {
        fn poppp() {
            let mut my_vector = vec![165, 314, 671, 113, 923, 314, 194, 422, 652, 422];

            while !my_vector.is_empty() {
                let removed_element = my_vector.remove(0);
            }
        }
        poppp()
    }
}
