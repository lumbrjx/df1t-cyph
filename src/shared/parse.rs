pub fn split_string(input: &str) -> Vec<String> {
    let result: Vec<String> = input.to_string().chars().map(|c| c.to_string()).collect();
    result
}

pub fn join_string(chars: Vec<String>) -> String {
    chars.join("")
}

pub fn flatten_vec(nstd: Vec<String>) -> Vec<String> {
    let flatten: Vec<String> = nstd
        .iter()
        .flat_map(|s| s.chars().map(|c| c.to_string()))
        .collect();

    flatten
}

pub fn str2_string_vec(nsrd: Vec<&str>) -> Vec<String> {
    let nsrd_as_strings: Vec<String> = nsrd.iter().map(|&s| s.to_string()).collect();
    nsrd_as_strings
}

pub fn string_vec2_str(nsrd: &[String]) -> Vec<&str> {
    nsrd.iter().map(|s| s.as_str()).collect()
}

pub fn split_by_n(n: usize, chunk: Vec<String>) -> Vec<Vec<String>> {
    let mut split_vectors: Vec<Vec<String>> = chunk.chunks(n).map(|chunk| chunk.to_vec()).collect();
    if split_vectors.len() < n {
        split_vectors.push(vec![]);
    }
    split_vectors
}

pub fn rem_zeros(chunk: Vec<i32>) -> (Vec<i32>, usize) {
    let count_zeros = chunk.iter().filter(|&&c| c == 0).count();
    let filtered_chunk: Vec<i32> = chunk.into_iter().filter(|&c| c != 0).collect();
    (filtered_chunk, count_zeros)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_a_vec() {
        let vec: Vec<String> = vec!["s", "q", "d", "f", " ", "4", "5", "h", " ", "d", "f"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        let result = "sqdf 45h df";
        let joined = join_string(vec);

        assert_eq!(result, joined)
    }

    #[test]
    fn parse_a_string() {
        let string = "sqdf 45h df";
        let result = vec!["s", "q", "d", "f", " ", "4", "5", "h", " ", "d", "f"];
        let splitted = split_string(string);

        assert_eq!(result, splitted);
    }

    #[test]
    fn split_string_then_restor_it() {
        let string = "super secret 8 t8 --y strin_g=";
        let splitted = split_string(string);
        let joined = join_string(splitted);
        assert_eq!(joined, string);
    }

    #[test]
    fn flatten_test() {
        let nsrd = vec!["Av", "bQ", "TG"];
        let res = flatten_vec(str2_string_vec(nsrd));
        assert_eq!(res, vec!["A", "v", "b", "Q", "T", "G"])
    }
    #[test]
    fn split_by_6_28_elemnets() {
        let ele = vec![
            "785", "535", "789", "987", "123", "789", "785", "535", "789", "987", "123", "787",
            "785", "535", "789", "987", "123", "456", "543", "528", "693", "285", "147", "556",
            "753", "456", "456", "564",
        ];
        let exp = vec![
            vec!["785", "535", "789", "987", "123", "789"],
            vec!["785", "535", "789", "987", "123", "787"],
            vec!["785", "535", "789", "987", "123", "456"],
            vec!["543", "528", "693", "285", "147", "556"],
            vec!["753", "456", "456", "564"],
            vec![],
        ];
        let res = split_by_n(6, str2_string_vec(ele));
        assert_eq!(res, exp)
    }
}
