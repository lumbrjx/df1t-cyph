use std::error::Error;

pub fn switch_chars<'a>(
    salt_vec: Vec<&'a str>,
    password_vec: Vec<&'a str>,
) -> Result<Vec<String>, Box<dyn Error>> {
    if salt_vec.len() != password_vec.len() {
        return Err("Error: not enough args for comparison".into());
    }
    let mut result_vector: Vec<String> = Vec::new();
    // Iterate through each pair of elements in the vectors
    for (elem1, elem2) in salt_vec.iter().zip(password_vec.iter()) {
        // Get the first characters of each element
        let salt_char1 = elem1.chars().next();
        let salt_char2 = elem1.chars().last();
        // Get the second characters of each element
        let password_char1 = elem2.chars().next();
        let password_char2 = elem2.chars().last();
        // after comparison elements
        let mut char1: String = String::from("");
        let mut char2: String = String::from("");
        // Compare the characters
        match (salt_char1, salt_char2, password_char1, password_char2) {
            (Some(c1), Some(c2), Some(c3), Some(c4)) => {
                println!(" chars {} , {}, {} , {} ", c1, c2, c3, c4);
                if c1 != c3 {
                    char1 = c1.to_string();
                }
                if c2 != c4 {
                    char2 = c4.to_string();
                }
                if c1 == c3 {
                    char1 = c1.to_string()
                }
                if c2 == c4 {
                    char2 = c2.to_string()
                }
                if c1 == c3 && c2 == c4 {
                    char1 = elem1.chars().rev().collect();
                    char2 = "".to_string();
                }
            }
            _ => result_vector = vec![],
        };
        // Add the modified element to the result vector
        result_vector.push(char1 + &char2);
    }
    Ok(result_vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switcher_equal_elements() {
        match switch_chars(vec!["AD", "FF", "LH"], vec!["AD", "FF", "LH"]) {
            Ok(res) => {
                let expected = vec!["DA", "FF", "HL"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }

    #[test]
    fn switcher_different_first_chars() {
        match switch_chars(vec!["fD", "FF", "LH"], vec!["AD", "gF", "mH"]) {
            Ok(res) => {
                let expected = vec!["fD", "FF", "LH"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }

    #[test]
    fn switcher_different_second_chars() {
        match switch_chars(vec!["AD", "AF", "AH"], vec!["Ar", "Am", "Al"]) {
            Ok(res) => {
                let expected = vec!["Ar", "Am", "Al"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }

    #[test]
    fn switcher_both_different() {
        match switch_chars(vec!["ly", "hF", "iH"], vec!["dr", "Am", "ol"]) {
            Ok(res) => {
                let expected = vec!["lr", "hm", "il"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }

    #[test]
    fn switcher_mixed_collection() {
        match switch_chars(vec!["yR", "JF", "iH"], vec!["yR", "Jm", "oH"]) {
            Ok(res) => {
                let expected = vec!["Ry", "Jm", "iH"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }

    #[test]
    #[should_panic]
    fn switcher_unequal_args() {
        match switch_chars(vec!["yR", "JF"], vec!["yR", "Jm", "oH"]) {
            Ok(res) => {
                let expected = vec!["Ry", "hm", "iH"];
                assert_eq!(res, expected)
            }
            Err(err) => panic!("Unexpected error: {}", err),
        };
    }
}
