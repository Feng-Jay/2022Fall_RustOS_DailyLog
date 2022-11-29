use crate::List::*;

enum List{
    // Cons: Tuple struct, an element and a pointer to next node
    Cons(u32, Box<List>),
    // Nil: a node that signifies the end of ll
    Nil,
}

impl List{
    // create a new ll
    fn new() -> List{
        Nil
    }
    // return a same list with new element as its front
    fn prepend(self, elem:u32) -> List{
        Cons(elem, Box::new(self))
    }
    // return length of ll
    fn len(&self) -> u32{
        match *self{
            Cons(_, ref tail) => 1+tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String{
        match *self{
            Cons(head, ref tail) =>{
                format!("{}, {}", head, tail.stringify())
            },
            Nil=>{
                format!("Nil")
            },
        }
    }
}

fn main(){
    let mut list = List::new();
    
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Link list has length: {}", list.len());
    println!("{}", list.stringify());
}