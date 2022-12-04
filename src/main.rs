mod asdf;
use asdf as d;
use crate::object::{Runner, Swimmer, Thing};

mod object;


fn main() {
    println!("Hello, world!");

    d::h("hello");

    // let o = object::Swimmer::new();
    // o.swim();

    let d = <Thing as Swimmer>::new();
    d.swim();




}
