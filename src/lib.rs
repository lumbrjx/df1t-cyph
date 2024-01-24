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
pub use encpt::decrypt::df1t_decrypt;
pub use encpt::encrypt::df1t_encrypt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encp_and_decp1() {
        let salt = "qsf876sqdf";
        let buffer = "password";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let res = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
        assert_eq!(res.unwrap().to_owned(), buffer);
    }
    #[test]
    fn encp_and_decp2() {
        let salt = "76sqdf";
        let buffer = "pa89631ord";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        println!("{:?}", &enc);
        let res = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
        assert_eq!(res.unwrap().to_owned(), buffer);
    }
    #[test]
    fn encp_and_decp3() {
        let salt = "76sqdf";
        let buffer = "76sqdf";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let res = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
        assert_eq!(res.unwrap().to_owned(), buffer);
    }
    #[test]
    fn encp_and_decp4() {
        let salt = "afdchjsrh9841d";
        let buffer = "maqzhf49qsd8ds4fgqsd94f89gs4hqsd68dsqh9yk44qds3";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let res = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
        assert_eq!(res.unwrap().to_owned(), buffer);
    }

    #[test]
    #[should_panic]
    fn encp_and_decp5() {
        let salt = "afdchjsrh98fqsd8489qsdfq6sf441d";
        let buffer = "maqzhf49qsd8ds4fgqsd94f89gs4hqsd68dsqh9yk44qds3";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let _ = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
    }
    #[test]
    #[should_panic]
    fn encp_and_decp6() {
        let salt = "";
        let buffer = "maqzhf49qsd8ds4fgqsd94f89gs4hqsd68dsqh9yk44qds3";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let _ = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
    }
    #[test]
    #[should_panic]
    fn encp_and_decp7() {
        let salt = "sdfqsdf";
        let buffer = "";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let _ = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
    }
    #[test]
    fn encp_and_decp8() {
        let salt = "8";
        let buffer = "8";
        let enc = df1t_encrypt(buffer.to_owned(), salt.clone().to_string());
        let _ = df1t_decrypt(enc.unwrap().to_owned(), salt.to_owned());
    }
}
// fn main() {
//     // Example usage
//     let salt = "0hfdf";
//     let password = "1sd41";
//     let res = df1t_encrypt(password.to_owned(), salt.clone().to_string());

//     let res1 = df1t_decrypt(res.unwrap().to_owned(), salt.to_owned());
//     println!("{}", res1.unwrap())
// }

//[[652, 165, 314, 671, 113],
// [422, 103, 923, 314, 194],
// [113, 389, 314, 422, 652],
// [923, 113, 194, 103, 422],
// [652, 389, 000, 000, 000]]
