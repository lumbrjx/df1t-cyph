use crate::encpt::{decrypt::df1t_decrypt, encrypt::df1t_encrypt, mapping::mapper::*};
pub mod maps {
    pub mod chars;
    pub mod salt;
}
mod shared {
    pub mod parse;
}
mod encpt {
    pub mod decrypt;
    pub mod encrypt;
    pub mod analyse {
        pub mod read;
    }
    pub mod math {
        pub mod matrix;
        pub mod process;
    }
    pub mod mapping {
        pub mod mapper;
        pub mod switcher;
    }
}

fn main() {
    // let _ = chr_to_mp(vec!["A", "B", "C"], MpType::CharMap);
    // match salt_extender(String::from("abc"), String::from("dsdfsqdfsqdff")) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(err) => eprintln!("Error: {}", err),
    // }
    // // DA fR FC
    // match switch_chars(vec!["AD", "fR", "Fm"], vec!["AD", "BR", "FC"]) {
    //     Ok(res) => println!("nested vec {:?}", res),
    //     Err(err) => println!("hmmmm {}", err),
    // };
    // let charvc = vec!["A", "v", "b", "Q", "T", "G"];
    // let res = chr_to_mxas(charvc);
    // println!("{:?}", res)

    // let vecs = vec![
    //     vec!["785", "535", "789", "987", "123"],
    //     vec!["543", "528", "693", "285", "147"],
    //     vec!["753"],
    // ];
    // let res = char_to_mtrx(vecs.iter().map(|c| str2_string_vec(c.to_vec())).collect());
    // println!("{:?}", res);

    // let vecs1 = vec![
    //     vec![785, 535, 789, 987, 123, 789],
    //     vec![785, 535, 789, 987, 123, 787],
    //     vec![785, 535, 789, 987, 123, 456],
    //     vec![543, 528, 693, 285, 147, 556],
    //     vec![753, 456, 456, 564, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0],
    // ];
    // // let res1 = fill_mtrx_gaps(6, vecs1);
    // // for a in res1 {
    // //     println!("{:?}", a);
    // // }
    // //
    // let s = mtrx_to_vecs(vecs1);
    // for d in s {
    //     println!("{:?}", d);
    // }
    // Example usage
    let salt = "sqdfqfgsffdqsdf";
    let password = "radqsfdfdfdqsdf";
    let res = df1t_encrypt(password.to_owned(), salt.to_owned());

    let saltt = "sqdfqfgsffdqsdf";
    let res1 = df1t_decrypt(res.unwrap().to_owned(), saltt.to_owned());
    println!("{}", res1.unwrap())
}
