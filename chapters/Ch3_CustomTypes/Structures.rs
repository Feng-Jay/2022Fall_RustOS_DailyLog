// hide warnings for unused code

#![allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point{
    x: f32,
    y: f32,
}

// 结构体也可以作为其他结构体的成员
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle{
    fn rect_area(&self) -> f32{
        let Point{x:tl_x, y:tl_y} = self.top_left;
        let Point{x:br_x, y:br_y} = self.bottom_right;
        if (tl_x > br_x) || (tl_y < br_y){
            -1.0
        }else{
            (br_x - tl_x) * (tl_y - br_y)
        }
    }
}

fn square(tl:Point, num:f32) -> Rectangle{
    let br:Point = Point{x:tl.x + num, y:tl.y - num};
    Rectangle{top_left: tl, bottom_right: br}
}

fn main(){
    let name = String::from("peter");
    let age  = 27;
    let peter= Person{name, age};
    
    // print debug struct
    println!("{:?}", peter);

    let point: Point = Point{x:5.3, y:0.4};
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // 通过使用结构体更新语法, 将原结构体的部分域放在新结构体中
    let bottom_right = Point{x:10.2, ..point};

    // bottom_right.y == point.y
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Deconstructure the point using let bind
    let Point{x: left_edge, y: top_edge} = point;
    println!("{}",left_edge);
    let _rectangle = Rectangle{
        // struct instantiation is an expression too
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    println!("rectangle area: {}", _rectangle.rect_area());
    // 实例化一个单元结构体
    let _unit = Unit;
    // 实例化元组结构体
    let pair = Pair(1, 0.1);
    // 元组结构体下标访问
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // 解析一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    let rec_out = square(point, 10.0);
    println!("square outcome: {},{}; {},{}", rec_out.top_left.x, rec_out.top_left.y, rec_out.bottom_right.x, rec_out.bottom_right.y);
}