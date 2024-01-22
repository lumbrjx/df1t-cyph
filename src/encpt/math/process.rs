pub fn add_to_vec(a: i32, chunk: Vec<i32>) -> Vec<i32> {
    chunk
        .iter()
        .map(|&c| if c == 0 { c } else { c + a })
        .collect()
}
pub fn rem_from_vec(a: i32, chunk: Vec<i32>) -> Vec<i32> {
    chunk
        .iter()
        .map(|&c| if c == 0 { c } else { c - a })
        .collect()
}
