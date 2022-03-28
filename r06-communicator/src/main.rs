extern crate r06_communicator;

use a::series::of::nested_modules;
use a::series::of::TrafficLight::{Green, Red};

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("A series of nested modules");
            }

            pub enum TrafficLight {
                Red, Green, Yellow,
            }
        }
    }
}


fn main() {
    nested_modules();
    a::series::of::nested_modules();
}