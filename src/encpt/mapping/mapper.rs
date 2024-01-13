use crate::maps::chars::*;
use crate::maps::salt::*;

#[derive(PartialEq)]
pub enum MpType {
    CharMap,
    SaltMap,
}

pub fn chr_to_mp(vc: Vec<&str>, mpt: MpType) -> Result<Vec<&str>, &str> {
    let mut result: Vec<&str> = vec![];
    let mpp: [[&str; 3]; 85];
    match mpt {
        MpType::CharMap => mpp = CHAR_MAP,
        MpType::SaltMap => mpp = SALT_MAP,
    }
    for e in &vc {
        for s in mpp {
            if e == &s[0] {
                result.push(s[1]);
                println!("{:?}", s[1])
            }
        }
    }
    if result.len() != vc.len() {
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
        let res = chr_to_mp(vec!["A", "B", "C"], MpType::CharMap);
        assert_eq!(res, Ok(vec!["Av", "bQ", "TG"]))
    }
}
