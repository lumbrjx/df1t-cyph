pub fn split_string(input: String) -> Vec<String> {
    let result: Vec<String> = input.chars().map(|c| c.to_string()).collect();
    result
}

pub fn join_string(chars: Vec<String>) -> String {
    chars.join("")
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
}
