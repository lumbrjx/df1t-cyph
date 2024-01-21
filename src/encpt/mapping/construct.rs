// cyph
pub fn num_to_dxxy<'a>(vc: Vec<&'a str>, mpt: [[&'a str; 2]; 10]) -> Result<Vec<&'a str>, &'a str> {
    let mut result: Vec<&str> = vec![];

    for e in &vc {
        for s in mpt {
            if e == &s[0] {
                result.push(s[1]);
            }
        }
    }
    if result.len() != vc.len() {
        Err("No matching characters found")
    } else {
        Ok(result)
    }
}

pub fn construct_str(dxxy: Vec<String>, info: Vec<String>) -> String {
    let mut result = String::from("");

    let min_length = dxxy.len().min(info.len());

    for i in 0..min_length {
        result.push_str(&dxxy[i]);
        result.push_str(&info[i]);
    }

    result
}
