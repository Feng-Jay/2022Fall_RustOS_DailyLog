#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age : u8
}

fn main(){
    println!("{:?} months in a year.", 12);
    println!("{1:?}{0:?} is the {author:?} name.",
             "A",
             "B",
             author = "FFengJay's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
    // these two code lines will print the string of input, not value.

    // this is a kind of ugly
    // can use {:#?} for pretty print
    let name = "Peter";
    let age  = 27;
    let peter= Person{name, age};
    println!("{:#?}", peter);
    println!("{:?}",peter);
    /* Output comparsion
    Person {
    name: "Peter",
    age: 27,
    }
    Person { name: "Peter", age: 27 }
     */
}