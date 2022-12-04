pub struct Thing {
    pub data: i32
}


pub trait Swimmer {
    fn new() -> Thing;
    fn swim(&self);
}
impl Swimmer for Thing {
    fn new() -> Thing {
        Thing {
            data: 7
        }
    }
    fn swim(&self) {
        println!("{} {}", "swimming", self.data);
    }
}

pub trait Runner {
    fn new() -> Thing;
    fn run(&self) {
        println!("{}", "running");
    }
}
impl Runner for Thing {
    fn new() -> Thing {
        Thing {
            data : 5
        }
    }
    fn run(&self) {
        println!("{} {}", "running", self.data);
    }
}
