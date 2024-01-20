use crate::encpt::mapping::mapper::*;
use encpt::mapping::switcher::switch_chars;
pub mod maps {
    pub mod chars;
    pub mod salt;
}
mod shared {
    pub mod parse;
}
mod encpt {
    pub mod mapping {
        pub mod mapper;
        pub mod switcher;
    }
}

fn main() {
    let _ = chr_to_mp(vec!["A", "B", "C"], MpType::CharMap);
    match salt_extender(String::from("abc"), String::from("dsdfsqdfsqdff")) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
    // DA fR FC
    match switch_chars(vec!["AD", "fR", "Fm"], vec!["AD", "BR", "FC"]) {
        Ok(res) => println!("nested vec {:?}", res),
        Err(err) => println!("hmmmm {}", err),
    };
    let charvc = vec!["A", "v", "b", "Q", "T", "G"];
    let res = chr_to_mxas(charvc);
    println!("{:?}", res)
}
