fn main(){
    // viriables can be annotated
    let logical: bool = true;
    let a_float: f64  = 1.0;
    let an_integer    = 5i32;

    // default will be used
    let default_float = 3.0; // f64
    let default_integer= 9; //i32

    // type can be inferred from context
    let mut inferred_type = 12;
    inferred_type =  4294967296i64;
    // inferred_type will be inferred as i64

    let mut mutable = 12;
    mutable = 21; //mut variable can be changed

    mutable = true // Error!!! the type of variable can't be changed.

    let mutable = true;
    // but varibles can be overwritten with shadowing
}