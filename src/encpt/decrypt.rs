use std::{error::Error, i32};

use crate::encpt::mapping::mapper::*;
use crate::{
    encpt::{
        mapping::switcher::reference_to_origin,
        math::{matrix::vecs_to_mtrx, process::rem_from_vec},
    },
    shared::parse::{
        concat_every_n_elements, generate_a_string, get_indexes, join_string, move_elements,
        pop_elements_from_vector, rem_zeros, split_string, str2_string_vec, string_vec2_str,
        MvDirection,
    },
};

use super::analyse::read::reader;

pub fn ceaser_unswap(indxs: Vec<String>, n: usize) -> Vec<i32> {
    let swp = get_indexes(
        move_elements(n, MvDirection::RIGHT),
        string_vec2_str(&indxs),
    );
    let cnct: Vec<String> = swp.iter().map(|c| c.to_string()).collect();
    concat_every_n_elements(string_vec2_str(&cnct), 3)
        .iter()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}
pub fn ref_to_bfr_vec(salt: String, mx_as_to_char: Vec<String>) -> Vec<String> {
    let salt_vec: Vec<&str>;

    let binding = split_string(&salt);
    match chr_to_mp(
        string_vec2_str(&binding),
        MpType::SaltMap,
        DirecType::FORWARD,
    ) {
        Ok(t) => salt_vec = t,
        Err(e) => panic!("{}", e),
    };
    let bfr_vec: Vec<String>;
    match reference_to_origin(salt_vec, string_vec2_str(&mx_as_to_char)) {
        Ok(t) => bfr_vec = t,
        Err(e) => panic!("{}", e),
    };
    bfr_vec
}
pub fn df1t_decrypt(buffer: String, salt: String) -> Result<String, Box<dyn Error>> {
    let parsed_buffer = reader(buffer, "$");

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

    // extended version of buffer
    let mx_as_to_char: Vec<String>;
    match chr_to_mxas(string_vec2_str(&restored_mtrx), DirecType::BACKWARD) {
        Ok(t) => mx_as_to_char = concat_every_n_elements(t, 2),
        Err(e) => panic!("{}", e),
    };

    // original length of the buffer before cyph
    let orgnl = ceaser_unswap(split_string(&parsed_buffer[0].clone()), 142);

    if orgnl[0] > salt.len() as i32 {
        // extending the salt
        let virt_str = generate_a_string(orgnl[0] as usize);
        let slt_extd: String;
        match salt_extender(&salt, &virt_str) {
            Ok(t) => slt_extd = t,
            Err(e) => panic!("{}", e),
        };
        // translating the salt to first level map
        let bfr_vec = ref_to_bfr_vec(slt_extd, mx_as_to_char);

        match chr_to_mp(
            string_vec2_str(&bfr_vec),
            MpType::CharMap,
            DirecType::BACKWARD,
        ) {
            Ok(t) => return Ok(join_string(str2_string_vec(t))),
            Err(e) => panic!("{}", e),
        };
        // last parse
    } else if orgnl[0] < salt.len() as i32 {
        // translating the salt to first level map
        let mut bfr_vec = ref_to_bfr_vec(salt.clone(), mx_as_to_char);
        // pop the extended elements
        pop_elements_from_vector(&mut bfr_vec, salt.len() - orgnl[0] as usize);

        match chr_to_mp(
            string_vec2_str(&bfr_vec),
            MpType::CharMap,
            DirecType::BACKWARD,
        ) {
            Ok(t) => return Ok(join_string(str2_string_vec(t))),
            Err(e) => panic!("{}", e),
        };
    } else {
        // translating the salt to first level map
        let bfr_vec = ref_to_bfr_vec(salt.clone(), mx_as_to_char);

        match chr_to_mp(
            string_vec2_str(&bfr_vec),
            MpType::CharMap,
            DirecType::BACKWARD,
        ) {
            Ok(t) => return Ok(join_string(str2_string_vec(t))),
            Err(e) => panic!("{}", e),
        };
    }
}
