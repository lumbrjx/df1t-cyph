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
    // Example usage
    let salt = "0hfdf";
    let password = "1sd41";
    let res = df1t_encrypt(password.to_owned(), salt.clone().to_string());

    let res1 = df1t_decrypt(res.unwrap().to_owned(), salt.to_owned());
    println!("{}", res1.unwrap())
}

//[[652, 165, 314, 671, 113],
// [422, 103, 923, 314, 194],
// [113, 389, 314, 422, 652],
// [923, 113, 194, 103, 422],
// [652, 389, 000, 000, 000]]
