use std::fmt::Error;

use crate::maps::first_lvl::*;

// uncompleted
pub fn mapper_lvl1(vc: Vec<&str>) -> Result<Vec<(&'static str, i32)>, &'static str> {
    let mut result: Vec<_> = vec![];
    let _ = vc.iter().map(|input_char| {
        for e in CHAR_MAP {
            print!("{:?}", e);
            if input_char == &e.0 {
                result.push(e.1[0])
            }
        }
    });

    if result.is_empty() {
        Err("No matching characters found")
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_one_char() {
        let res = mapper_lvl1(vec!["A", "B", "C"]);
        assert_eq!(res, Ok(vec![("Av", 67), ("bQ", 14), ("TG", 24)]))
    }
}
