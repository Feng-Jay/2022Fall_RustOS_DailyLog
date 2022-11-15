use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        // a reference of vec
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate(){
            if count != 0 {write!(f,", ")?;}
            write!(f,"{}:{}", v, count)?;
        }
        write!(f,"]")
    }
}

fn main(){
    let v = List(vec![0, 1, 2]);
    println!("{}", v);
}