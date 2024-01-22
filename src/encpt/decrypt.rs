use std::{error::Error, i32};

use crate::{
    encpt::math::process::rem_from_vec,
    shared::parse::{
        concat_every_three_elements, get_indexes, move_elements, split_string, string_vec2_str,
        Mv_Direction,
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
    concat_every_three_elements(string_vec2_str(&cnct))
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
    println!("{:?}", rd_rm);
    Ok(salt)
}
