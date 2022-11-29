#![allow(dead_code)]

enum Statue{
    Rich,
    Poor,
}

enum Work{
    Civilian,
    Soldier,
}

fn main(){
    use crate::Statue::{Poor, Rich};
    use crate::Work:: *;

    let status = Poor; // equal to Statue::Poor
    let work   = Civilian;

    match status{
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money!"),
    }
    match work{
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldier fight!"),
    }
}