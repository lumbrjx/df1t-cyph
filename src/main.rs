use crate::encpt::mapping::mapper::mapper_lvl1;

pub mod maps {
    pub mod first_lvl;
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
    let _ = mapper_lvl1(vec!["A", "B", "C"]);
}
