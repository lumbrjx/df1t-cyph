use std::error::Error;
use std::fmt;

use crate::encpt::mapping::mapper::*;
use crate::encpt::mapping::switcher::switch_chars;
use crate::encpt::math::matrix::{calc_n, char_to_mtrx, fill_mtrx_gaps, mtrx_to_vecs};
use crate::encpt::math::process::add_to_vec;
use crate::shared::parse::{
    flatten_vec, get_elements_by_indexes, join_string, move_elements, split_by_n, split_string,
    str2_string_vec, string_vec2_str, MvDirection,
};
// use crate::{chr_to_mp, chr_to_mxas, salt_extender, DirecType, MpType};
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

pub fn is_salt_short<'a>(buffer: &'a str, salt: &'a str) -> bool {
    if buffer.len() > salt.len() {
        return true;
    }
    return false;
}

pub fn flt_subvecs(n: usize, vc: Vec<i32>) -> Vec<String> {
    let flt = flatten_vec(
        add_to_vec(n as i32, vc)
            .iter()
            .map(|&c| c.to_string())
            .collect(),
    );
    flt
}
pub fn ceaser_swap(indxs: Vec<String>, n: usize) -> Vec<&'static str> {
    let swp = get_elements_by_indexes(
        move_elements(n, MvDirection::RIGHT),
        indxs.iter().map(|c| c.parse::<i32>().unwrap()).collect(),
    );
    swp
}

pub fn cyph_info(strct_element: usize, mv: usize) -> Vec<&'static str> {
    let cph = get_elements_by_indexes(
        move_elements(mv, MvDirection::RIGHT),
        flatten_vec(split_string(strct_element.to_string().as_str()))
            .iter()
            .map(|c| c.parse::<i32>().unwrap())
            .collect(),
    );
    cph
}
pub fn df1t_encrypt(buffer: String, salt: String) -> Result<String, Box<dyn Error>> {
    if salt.len() > 16 {
        return Err(Box::new(ValueError {
            message: "Salt can't be more than 16 char".to_string(),
        }));
    };

    // Extend the shotest string
    let extended: String;
    match salt_extender(&salt, &buffer) {
        Ok(res) => extended = res,
        Err(err) => panic!("{}", err),
    }

    // split the buffer and salt into vectors and parse them to mapped version
    let mut binding1: Vec<String> = split_string(&buffer);
    let mut binding2: Vec<String> = split_string(&salt);

    match is_salt_short(&buffer, &salt) {
        true => {
            binding2 = split_string(&extended);
        } // + add the shortest in struct
        false => binding1 = split_string(&extended),
    }

    let buffer_vec: Vec<&str>;
    match chr_to_mp(
        string_vec2_str(&binding1),
        MpType::CharMap,
        DirecType::FORWARD,
    ) {
        Ok(t) => buffer_vec = t,
        Err(e) => panic!("{}", e),
    };
    let salt_vec: Vec<&str>;
    match chr_to_mp(
        string_vec2_str(&binding2),
        MpType::SaltMap,
        DirecType::FORWARD,
    ) {
        Ok(t) => salt_vec = t,
        Err(e) => panic!("{}", e),
    };

    // Salt and buffer mixing
    let mixed: Vec<String>;
    match switch_chars(salt_vec, buffer_vec) {
        Ok(t) => mixed = t,
        Err(e) => panic!("{}", e),
    }

    // map the mixed vec into mx_as vec version
    let binding3 = flatten_vec(mixed);

    let mx_version: Vec<&str>;
    match chr_to_mxas(string_vec2_str(&binding3), DirecType::FORWARD) {
        Ok(t) => mx_version = t,
        Err(e) => panic!("{}", e),
    }
    let mtrx_n = calc_n(mx_version.len());

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
    let grn_a: Vec<String> = flt_subvecs(mtrx_n, grn.to_vec());
    let rd_a: Vec<String> = flt_subvecs(mtrx_n, rd.to_vec());
    let ble_a: Vec<String> = flt_subvecs(mtrx_n, ble.to_vec());

    // ceaser
    let grn_swapped = ceaser_swap(grn_a.clone(), grn_a.len() + 2);
    let rd_swapped = ceaser_swap(rd_a.clone(), rd_a.len());
    let ble_swapped = ceaser_swap(ble_a.clone(), ble_a.len() + rd_a.len() + 1);

    // create a new matrix of the new values
    let parsed_mtrx: Vec<Vec<String>> =
        vec![grn_swapped, vec!["$"], rd_swapped, vec!["$"], ble_swapped]
            .iter()
            .map(|c| flatten_vec(c.iter().map(|&c| c.to_string()).collect()))
            .collect();
    // faltten the matrix
    let flat_mtrx: Vec<String> = parsed_mtrx.into_iter().flatten().collect();
    // fulfill info and encrypt it
    let orgnl = cyph_info(buffer.len(), 142);
    let info_vec = vec![join_string(str2_string_vec(orgnl)), "$".to_string()];

    // join info + encrypted
    Ok(join_string(info_vec) + &join_string(flat_mtrx))
}
