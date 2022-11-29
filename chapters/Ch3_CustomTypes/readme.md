# 自定义类型

rust的自定义类型主要通过两个关键词声明:

* `struct`: 定义结构体

* `enum`: 定义枚举

常量也可以通过`const`和`static`关键字创建

## 结构体

`struct`关键字可以创建三种类型的结构体:

* 元组结构体, 就是有名字的元组

* 经典C结构体

* 单元结构体(unit struct), 不带字段, 在泛型中很有用

## 枚举 enum

`enum`关键字允许从多个不同取值中选一个作为枚举类型创建, 任何一个在结构体中合法的值, 在枚举中也合法. 很有用, 例如ip地址可以分为ipv4和ipv6, 通过将二者定义在ip地址这个枚举类型中, 可以直接使用ip地址类型调用对应函数, 在函数内做判断即可.

如果使用类型别名, 那么对于枚举变量也可以通过别名访问其每个变体.当枚举类型名称过长时可能很有用:

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}
```
最常用的情景是使用`self`别名`impl`一个block

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```
### use

使用`use`声明可以免去手动添加作用域. 详细内容看对应rust代码

### C-like

`enum`可以像C-like的枚举变量一样使用: 每个变量都有一个隐式值(默认从0开始)

也可以进行赋值, 详情看代码.

### testcase: Link-list

## constants

Rust有两种不同的常量, 可以在任何域中声明. 二者都需要显式声明:

* `const`: 无法改变的值

* `static`: 可能改变的值, 但只有static的生命周期. 修改或者访问一个static的可变值是被看作`unsafe`的

