pub fn split_string(input: String) -> Vec<String> {
    let result: Vec<String> = input.chars().map(|c| c.to_string()).collect();
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

pub fn str2String_vec(nsrd: Vec<&str>) -> Vec<String> {
    let nsrd_as_strings: Vec<String> = nsrd.iter().map(|&s| s.to_string()).collect();
    nsrd_as_strings
}

pub fn split_by_five(chunk: Vec<String>) -> Vec<Vec<String>> {
    let split_vectors: Vec<Vec<String>> = chunk.chunks(5).map(|chunk| chunk.to_vec()).collect();
    split_vectors
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
        let splitted = split_string(string.to_string());

        assert_eq!(result, splitted);
    }

    #[test]
    fn split_string_then_restor_it() {
        let string = "super secret 8 t8 --y strin_g=";
        let splitted = split_string(string.to_string());
        let joined = join_string(splitted);
        assert_eq!(joined, string);
    }

    #[test]
    fn flatten_test() {
        let nsrd = vec!["Av", "bQ", "TG"];
        let res = flatten_vec(str2String_vec(nsrd));
        assert_eq!(res, vec!["A", "v", "b", "Q", "T", "G"])
    }
}
