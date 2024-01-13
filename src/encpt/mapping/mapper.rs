use crate::maps::first_lvl::*;

// uncompleted
pub fn mapper_lvl1(vc: Vec<&str>) -> Result<Vec<[&str; 3]>, &str> {
    let mut result: Vec<[&str; 3]> = vec![];
    for e in vc {
        for s in CHAR_MAP {
            if e == s[0] {
                result.push(s);
                println!("{:?}", s)
            }
        }
    }
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
    fn try_char() {
        let res = mapper_lvl1(vec!["A", "B", "C"]);
        assert_eq!(
            res,
            Ok(vec![
                ["A", "Av", "671"],
                ["B", "bQ", "142"],
                ["C", "TG", "243"]
            ])
        )
    }
}
