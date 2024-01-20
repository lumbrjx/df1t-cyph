use crate::shared::parse::split_string;

pub fn char_to_mtrx(chars: Vec<Vec<String>>) -> Vec<Vec<Vec<i32>>> {
    let final_vec: Vec<Vec<Vec<i32>>> = chars
        .into_iter()
        .map(|e| {
            e.iter()
                .map(|c| {
                    split_string(c.to_string())
                        .iter()
                        .map(|r| r.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    final_vec
}
// from [["145","456","789"],["741","789","123"]]

// to [[[1,4,5],[4,5,6],[7,8,9]],[[7,4,1],[7,8,9],[1,2,3]]]

#[cfg(test)]
mod tests {
    use crate::shared::parse::str2String_vec;

    use super::*;

    #[test]
    fn mtrx_vals() {
        let expected = vec![
            vec![vec![1, 4, 5], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![7, 8, 9], vec![1, 2, 3]],
        ];
        let nstd_vec = vec![vec!["145", "456", "789"], vec!["741", "789", "123"]];
        let res = char_to_mtrx(
            nstd_vec
                .iter()
                .map(|c| str2String_vec(c.to_vec()))
                .collect(),
        );
        assert_eq!(res, expected)
    }
}
