use crate::encpt::mapping::mapper::*;

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
    }
}

fn main() {
    let _ = chr_to_mp(vec!["A", "B", "C"], MpType::CharMap);
    match salt_extender(String::from("abc"), String::from("dsdfsqdfsqdff")) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
