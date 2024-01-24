use std::error::Error;
use std::fmt;

use crate::maps::chars::*;
use crate::maps::salt::*;

#[derive(PartialEq)]
pub enum MpType {
    CharMap,
    SaltMap,
}

#[derive(PartialEq)]
pub enum DirecType {
    FORWARD,
    BACKWARD,
}
// map strings to vectors
pub fn chr_to_mp(vc: Vec<&str>, mpt: MpType, direc: DirecType) -> Result<Vec<&str>, &str> {
    let mut result: Vec<&str> = vec![];
    let mpp: [[&str; 3]; 62];
    let mut fs: usize = 0;
    let mut sc: usize = 1;
    if direc == DirecType::BACKWARD {
        fs = 1;
        sc = 0
    }
    match mpt {
        MpType::CharMap => mpp = CHAR_MAP,
        MpType::SaltMap => mpp = SALT_MAP,
    }
    for e in &vc {
        for s in mpp {
            if e == &s[fs] {
                result.push(s[sc]);
            }
        }
    }
    if result.len() != vc.len() {
        Err("CharError: unrecognized char")
    } else {
        Ok(result)
    }
}

pub fn chr_to_mxas(vc: Vec<&str>, direc: DirecType) -> Result<Vec<&str>, &str> {
    let mut result: Vec<&str> = vec![];
    let mut fs: usize = 0;
    let mut sc: usize = 2;
    if direc == DirecType::BACKWARD {
        fs = 2;
        sc = 0
    }
    for e in &vc {
        for s in CHAR_MAP {
            if e == &s[fs] {
                result.push(s[sc]);
            }
        }
    }
    if result.len() != vc.len() {
        Err("CharError: unrecognized char")
    } else {
        Ok(result)
    }
}

// extend salt based on string length
#[derive(Debug)]
struct EmptyValueError;

impl fmt::Display for EmptyValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyValueError: can't accept empty values")
    }
}

impl Error for EmptyValueError {}

struct ExtValue {
    longer: String,
    shorter: String,
}
impl ExtValue {
    fn ext_data(&self) -> String {
        let extend_size = &self.longer.len() - &self.shorter.len();
        let mut chunk: String = String::from(&self.shorter);
        while extend_size > chunk.len() {
            chunk = chunk + &self.shorter;
        }
        let trimmed = &chunk[..extend_size];
        let concated = self.shorter.clone() + trimmed;
        concated
    }
}

pub fn salt_extender(salt: &str, password: &str) -> Result<String, Box<dyn Error>> {
    if salt.is_empty() || password.is_empty() {
        return Err(Box::new(EmptyValueError));
    }

    if salt.len() > password.len() {
        let res = ExtValue {
            longer: salt.to_string(),
            shorter: password.to_string(),
        };
        return Ok(res.ext_data());
    }

    if salt.len() < password.len() {
        let res = ExtValue {
            longer: password.to_string(),
            shorter: salt.to_string(),
        };
        return Ok(res.ext_data());
    }

    Ok(password.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_char() {
        let res = chr_to_mp(vec!["A", "B", "C"], MpType::CharMap, DirecType::FORWARD);
        assert_eq!(res, Ok(vec!["Aj", "bQ", "TG"]))
    }
    #[test]
    fn salt_extender_longer() {
        let longer = String::from("abc");
        let shorter = String::from("dsdfsqdfsqdff");
        match salt_extender(&longer, &shorter) {
            Ok(result) => {
                let expected = String::from("abcabcabcabca");
                assert_eq!(result, expected);
            }
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }

    #[test]
    fn salt_extender_shorter() {
        let longer = String::from("abc");
        let shorter = String::from("dsdfsqdfsqdff");
        match salt_extender(&shorter, &longer) {
            Ok(result) => {
                let expected = String::from("abcabcabcabca");
                assert_eq!(result, expected);
            }
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }
    #[test]
    fn salt_extender_even() {
        let longer = String::from("dsdfsqdfsqdff");
        let shorter = String::from("dsdfsqdfsqdff");
        match salt_extender(&shorter, &longer) {
            Ok(result) => {
                let expected = String::from("dsdfsqdfsqdff");
                assert_eq!(result, expected);
            }
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }

    #[test]
    #[should_panic]
    fn salt_extender_empty() {
        let longer = String::from("");
        let shorter = String::from("");
        match salt_extender(&shorter, &longer) {
            Ok(result) => {
                let expected = String::from("dsdfsqdfsqdff");
                assert_eq!(result, expected);
            }
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }

    #[test]
    fn chr_to_mxas_test() {
        let charvc = vec!["A", "v", "b", "Q", "T", "G"];
        let res = chr_to_mxas(charvc, DirecType::FORWARD);
        assert_eq!(res, Ok(vec!["671", "258", "421", "652", "790", "487"]))
    }
}
