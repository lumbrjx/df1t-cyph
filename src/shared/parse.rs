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

pub fn rem_zeros(chunk: Vec<i32>) -> Vec<i32> {
    let filtered_chunk: Vec<i32> = chunk.into_iter().filter(|&c| c != 101).collect();
    filtered_chunk
}

pub fn concat_every_n_elements(input_vec: Vec<&str>, n: usize) -> Vec<String> {
    let mut result_vec = Vec::new();

    // Iterate over the input vector in chunks of three
    for chunk in input_vec.chunks(n) {
        // Concatenate the elements in each chunk
        let concatenated = chunk.join("");
        result_vec.push(concatenated);
    }

    result_vec
}

pub fn generate_a_string(n: usize) -> String {
    // Create a string with 'A' repeated n times
    let result: String = std::iter::repeat('A').take(n).collect();
    result
}

pub fn pop_elements_from_vector(vector: &mut Vec<String>, n: usize) {
    // Ensure not to pop more elements than the vector has
    let num_elements_to_pop = n.min(vector.len());

    // Calculate the starting index for elements to pop
    let start_index = vector.len() - num_elements_to_pop;

    // Truncate the vector to remove the elements
    vector.truncate(start_index);
}
#[derive(PartialEq)]
pub enum MvDirection {
    LEFT,
    RIGHT,
}
pub fn move_elements(n: usize, direction: MvDirection) -> Vec<&'static str> {
    let vec = vec![
        "e", "M", "y", "P", "c", "6", "I", "-", "8", "u", "F", "@", "b", "T", "w", ".", "J", "O",
        "z", "p", ":", "W", "7", "v", "V", "K", "5", "H", "q", "f", "&", "/", "A", "{", "Z", "d",
        "L", "9", "N", "R", "*", "h", "0", "D", "G", "]", "s", "3", "S", "r", "2", "Q", "}", "g",
        "X", "x", "k", "<", "B", "(", "C", "j", "!", "U", "m", "a", "i", "o", "4", "l", "1", "E",
        "t", "n",
    ];
    let len = vec.len();
    if len == 0 {
        return vec;
    }
    if n == 0 {
        return vec;
    }

    let n = n % len; // Ensure n is within the range of vector length

    // Create a new vector with the elements moved to the right
    let mut new_vec = vec![Default::default(); len];
    if direction == MvDirection::RIGHT {
        for i in 0..len {
            new_vec[(i + n) % len] = vec[i].clone();
        }
    }
    if direction == MvDirection::LEFT {
        for i in 0..len {
            new_vec[i] = vec[(i + len - n) % len].clone();
        }
    }

    new_vec.into_iter().take(10).collect()
}

pub fn get_elements_by_indexes(original: Vec<&str>, indexes: Vec<i32>) -> Vec<&str> {
    indexes
        .iter()
        .filter_map(|&i| original.get(i as usize).cloned())
        .collect()
}

pub fn get_indexes(map: Vec<&str>, second: Vec<&str>) -> Vec<usize> {
    // Check if both vectors have the same length

    // Create a mapping of elements in the second vector to their indexes in the first vector
    let mut index_map = std::collections::HashMap::new();
    for (index, &element) in map.iter().enumerate() {
        index_map.insert(element, index);
    }

    // Use the mapping to get the indexes of elements in the second vector
    let indexes: Vec<usize> = second
        .iter()
        .flat_map(|&element| index_map.get(&element))
        .cloned()
        .collect();

    // Check if all elements in the second vector were found in the first vector
    indexes
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
