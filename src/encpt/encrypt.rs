use std::error::Error;
use std::fmt;

use crate::encpt::mapping::switcher::switch_chars;
use crate::encpt::math::matrix::{calc_n, char_to_mtrx, fill_mtrx_gaps, mtrx_to_vecs};
use crate::encpt::math::process::add_to_vec;
use crate::shared::parse::{
    flatten_vec, split_by_n, split_string, str2_string_vec, string_vec2_str,
};
use crate::{chr_to_mp, chr_to_mxas, salt_extender, MpType};

#[derive(Debug)]
struct ValueError {
    message: String,
}
impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EncrytionError: {}", self.message)
    }
}

impl Error for ValueError {}

struct EncptInfo {
    original_length: usize,
    salt_short: i8, // 0 : buffer, 1 : salt
    mtrx_n: usize,
    rd_len: usize,
}
pub fn is_salt_short<'a>(buffer: &'a str, salt: &'a str) -> bool {
    if buffer.len() > salt.len() {
        return true;
    }
    return false;
}

pub fn df1t_encrypt(buffer: String, salt: String) -> Result<String, Box<dyn Error>> {
    if salt.len() > 16 {
        return Err(Box::new(ValueError {
            message: "Salt can't be more than 16 char".to_string(),
        }));
    };

    // add original length
    //
    // .....
    //

    // Extend the shotest string
    let extended: String;
    match salt_extender(&salt, &buffer) {
        Ok(res) => extended = res,
        Err(err) => panic!("{}", err),
    }
    println!("im extended {}", extended);

    // split the buffer and salt into vectors and parse them to mapped version
    let mut binding1: Vec<String> = split_string(&buffer);
    let mut binding2: Vec<String> = split_string(&salt);

    match is_salt_short(&buffer, &salt) {
        true => binding2 = split_string(&extended), // + add the shortest in struct
        false => binding1 = split_string(&extended),
    }

    let buffer_vec = chr_to_mp(string_vec2_str(&binding1), MpType::CharMap);
    let salt_vec = chr_to_mp(string_vec2_str(&binding2), MpType::SaltMap);
    println!("im buffer {:?}", &buffer_vec);
    println!("im salt {:?}", &salt_vec);

    // Salt and buffer mixing
    let mixed: Vec<String>;
    match switch_chars(salt_vec, buffer_vec) {
        Ok(t) => mixed = t,
        Err(e) => panic!("{}", e),
    }
    println!("im mixed {:?}", &mixed);

    // map the mixed vec into mx_as vec version
    let binding3 = flatten_vec(mixed);
    let mx_version: Vec<&str>;
    match chr_to_mxas(string_vec2_str(&binding3)) {
        Ok(t) => mx_version = t,
        Err(e) => panic!("{}", e),
    }
    let mtrx_n = calc_n(mx_version.len());
    // add mtrx_n to struct

    // split the chunk into unstructered matrix
    let splitted_empty = split_by_n(mtrx_n, str2_string_vec(mx_version));
    // structure the matrix by filling the gaps with 0's
    let splitted_filled = fill_mtrx_gaps(mtrx_n, char_to_mtrx(splitted_empty));
    // get the green, red, blue vecs from the matrix
    let vecs_from_mtrx = mtrx_to_vecs(splitted_filled);
    let grn = &vecs_from_mtrx[0];
    let rd = &vecs_from_mtrx[1];
    let ble = &vecs_from_mtrx[2];
    // add mtrx_n to green and blue and the red length to red
    let grn_a = add_to_vec(mtrx_n as i32, grn.to_vec());
    let rd_a = add_to_vec(rd.len() as i32, rd.to_vec());
    let ble_a = add_to_vec(mtrx_n as i32, ble.to_vec());
    // create a new matrix of the new values
    let parsed_mtrx: Vec<Vec<String>> = vec![grn_a, rd_a, ble_a]
        .iter()
        .map(|c| flatten_vec(c.iter().map(|&c| c.to_string()).collect()))
        .collect();
    // faltten the matrix
    let flat_mtrx: Vec<String> = parsed_mtrx.into_iter().flatten().collect();
    println!("to string {:?}", flat_mtrx);
    // println!("blue {:?}", ble_a);
    Ok("done".to_string())
}
