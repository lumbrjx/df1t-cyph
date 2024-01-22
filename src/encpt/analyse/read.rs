use crate::shared::parse::str2_string_vec;

pub fn reader(buffer: String, letter: &str) -> Vec<String> {
    let by_dollar: Vec<&str> = buffer.split(letter).collect();
    str2_string_vec(by_dollar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_buffer() {
        let buffer = "M6;n;c$fHH&fHLfLqqL/fd&fHLfLqHL&LAfHHqHL&LA&fHLfL&LA$mUijoUmUi!Ci(BoB$v7/Vq5fVVv7/K/Hvv/qH&V77vv/5V&77777";
        let res = str2_string_vec(vec![
            "M6;n;c",
            "fHH&fHLfLqqL/fd&fHLfLqHL&LAfHHqHL&LA&fHLfL&LA",
            "mUijoUmUi!Ci(BoB",
            "v7/Vq5fVVv7/K/Hvv/qH&V77vv/5V&77777",
        ]);
        let d = reader(buffer.to_string(), "$");
        assert_eq!(res, d)
    }
}
