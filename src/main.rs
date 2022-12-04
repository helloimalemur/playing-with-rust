mod asdf;
use asdf as d;
use crate::object::{Runner, Swimmer, Thing};

mod object;


fn main() {
    println!("Hello, world!");

    d::h("hello");

    // let o = object::Swimmer::new();
    // o.swim();
    //https://doc.rust-lang.org/error_codes/E0790.html

    let e = object::Thing {
        data: 10
    };
    e.swim();
    e.run();

    let g = <Thing as Runner>::new();
    g.run();

    let d = <Thing as Swimmer>::new();
    d.swim();




}
