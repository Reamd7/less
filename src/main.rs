#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod data;
pub mod logger;

fn main() {
    println!("{:?}", data::color::aliceblue);
    println!("Hello, world!");
}
