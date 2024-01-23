use std::{error::Error, i32};

use crate::{
    encpt::math::{matrix::vecs_to_mtrx, process::rem_from_vec},
    mxas_to_chars,
    shared::parse::{
        concat_every_n_elements, get_indexes, move_elements, rem_zeros, split_string,
        string_vec2_str, Mv_Direction,
    },
};

use super::analyse::read::reader;

#[derive(Debug)]
struct EncptInfo {
    original_length: usize,
    salt_short: usize, // 0 : buffer, 1 : salt
    mtrx_n: usize,
}

pub fn ceaser_unswap(indxs: Vec<String>, n: usize) -> Vec<i32> {
    let swp = get_indexes(
        move_elements(n, Mv_Direction::RIGHT),
        string_vec2_str(&indxs),
    );
    let cnct: Vec<String> = swp.iter().map(|c| c.to_string()).collect();
    concat_every_n_elements(string_vec2_str(&cnct), 3)
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}
pub fn df1t_decrypt(buffer: String, salt: String) -> Result<String, Box<dyn Error>> {
    let parsed_buffer = reader(buffer, "$");
    let info = &parsed_buffer[0];
    let parsed_info = reader(info.to_string(), ";");
    // split each vector
    let grn = split_string(&parsed_buffer[1]);
    let rd = split_string(&parsed_buffer[2]);
    let ble = split_string(&parsed_buffer[3]);
    // reverse the ceaser swapping
    let grn_un = ceaser_unswap(grn.clone(), grn.len() + 2);
    let rd_un = ceaser_unswap(rd.clone(), rd.len());
    let ble_un = ceaser_unswap(ble.clone(), ble.len() + rd.len() + 1);

    let rdlen = rd_un.len();
    let grn_rm = rem_from_vec(rdlen as i32, grn_un);
    let rd_rm = rem_from_vec(rdlen as i32, rd_un);
    let ble_rm = rem_from_vec(rdlen as i32, ble_un);

    // restore the matrix from the rgb vectors and remove the Null elements
    let restored_mtrx: Vec<String> = rem_zeros(vecs_to_mtrx(vec![grn_rm, rd_rm, ble_rm]))
        .iter()
        .map(|c| c.to_string())
        .collect();

    let mx_as_to_char: Vec<String>;
    match mxas_to_chars(string_vec2_str(&restored_mtrx)) {
        Ok(t) => mx_as_to_char = concat_every_n_elements(t, 2),
        Err(e) => panic!("{}", e),
    };
    println!("{:?}", mx_as_to_char);
    Ok(salt)
}

// mx version ["652", "165", "314", "671", "113", "422", "103", "923", "314", "194", "113", "389", "314", "422", "652", "923", "113", "194", "103", "422", "652", "389"]
//             [652, 165, 314, 671, 113, 422, 103, 923, 314, 194, 113, 389, 314, 422, 652, 923, 113, 194, 103, 422, 652, 389]

// green [165, 314, 671, 113, 923, 314, 194, 422, 652, 422]
// red [652, 103, 314, 103, 0]
// blue [422, 113, 389, 923, 113, 194, 652, 389, 0, 0]

// green dcp : [165, 314, 671, 113, 923, 314, 194, 422, 652, 422]
// red dcp : [652, 103, 314, 103, 0]
// blue dcp : [422, 113, 389, 923, 113, 194, 652, 389, 0]

// the original matrix [[652, 165, 314, 671, 113], [422, 103, 923, 314, 194], [113, 389, 314, 422, 652], [923, 113, 194, 103, 422], [652, 389, 101, 101, 101]]
//                      [652, 165, 314, 671, 113, 422, 103, 923, 314, 194,      113, 389, 314, 422, 652, 923, 113, 194, 103, 422, 652, 389, 101, 101, 101]
