fn main(){
    // like python
    // you can format string by {}
    println!("{} days", 31);
    // {int} represent index
    println!("{0}, this is {1}; {1}, this is {0}", "alice", "bob");
    // {string} named variable
    println!("{subject} {verb} {object}",
             object = "lazy dog",
             subject= "owner",
             verb = "hit");
    // by using :b/:o/:x in {}, can change base
    println!("Base 10 : {}", 1024);
    println!("Base 2  : {:b}", 1024);
    println!("Base 8  : {:o}", 1024);
    println!("Base 16 : {:x}", 1024);
    println!("Base 16 : {:X}", 1024);
    // control output width
    // > number on the right side
    // < number on the left  side
    println!("{number:>5}", number=1);
    println!("{numbersss:0<5}", numbersss=1);
    // by adding $, can use varible after :
    println!("{numbersss:0<width$}", numbersss=1, width=5);
    // println!("My name is ")
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    // it can see surrouding variables
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("{pi:.3}");
}