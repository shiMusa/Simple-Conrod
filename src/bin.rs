#![feature(duration_extras)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]



extern crate sabi;

#[allow(unused_imports)]
use sabi::{elements};
#[allow(unused_imports)]
use elements::{*, action::*, basic::*, container::*};
#[allow(unused_imports)]
use std::sync::mpsc::{self, Sender, Receiver};









fn main() {
    sabi::example3();
    //sabi::expample2();
    //sabi::example();
}
