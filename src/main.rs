use environment::abstract_file_manager::AbstractFileManager;
use environment::abstract_file_manager::AbstractFileManagerStruct;
mod data;
// pub mod logger;
mod environment;

#[allow(non_upper_case_globals)]
#[allow(dead_code)]

fn main() {
    println!("{:?}", data::color::aliceblue);
    println!("Hello, world!");
    println!("{}", AbstractFileManagerStruct::get_path("ext/111"))
}
