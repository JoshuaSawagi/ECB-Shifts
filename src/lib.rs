#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]

use skyline::nro::{self, NroInfo};
mod common;
mod utils;
pub mod ext;

#[skyline::main(name = "L-Cancels")]
pub fn main() {

    common::install();
}