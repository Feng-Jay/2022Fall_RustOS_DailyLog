# Comment

rust support comments:
1. 编译器忽略的注释:

```rust
// line be commented

/* Block lines be commented*/
```

2. 被parse为html的doc注释:

```rust
/// generate library doc line
//! generate library doc for enclosing item.
```

# Formatted print

printing 被`std::fmt`中的一系列宏定义:

`format!`  : 将文本转为`String`  
`print!`   : 和format!类似，但文本向终端输出  
`println!` : 和print!类似，但换行符会被加上  
`eprint!`  : 与print!类似，但文本会输出到标准错误输出  
`eprintln!`: 与eprint!类似，但换行符会被加上  

std::fmt中有很多特性，主要可以分为两大类:

1. `fmt::Debug`: 使用`{:?}`标识符, 将文本以debug目的使用

2. `fmt::Display`: 使用`{}`标识符, 将文本以更优美的方式展示

实现fmt::Display特性会自动实现`ToString`特性，让我们将这些变量转变为String.

## Debug

只有std中的类型已经提供了该特性。想要使用 std::fmt特性的类型都需要实现该特性，其他数据类型必须手动实现该特性。

但fmt::Debug特性使得输出变得简单, 所有的类型可以从fmt::Debug中继承实现。但在fmt::Display中必须手动实现

当然，std中所有类型也可以使用该特性进行输出。

## Dispaly

fmt::Debug 看起来并不紧凑整洁，所以有时需要将输出自定义。这就是fmt::Display所做的, 使用`{}`标识符。

*实现细节可在Display.rs中看到*

但对于模糊类型如何输出呢，例如对于模板`Vec<T>` 将输出什么风格呢？

```rust
Vec<path>: /:/etc:/home/username:/bin (split on :)
Vec<number>: 1,2,3 (split on ,)
```
例如上述例子，可能是上面两种中的哪一种呢?

答案是都不是，fmt::Display 没有对`Vec<T>`或其他的模板容器实现，必须使用`fmt::Debug`来打印。

但对于声明的确定型容器来说，可以实现`fmt::Display`特性 *见对应章节代码*

### Testcase: List

对于一个元素必须被线性处理的结构体实现`fmt::Display`特性是比较难的。问题在于每一个`write!`都会产生一个`fmt::Result`. 需要正确处理所有的results. Rust 提供了 `?` 算符来处理此种情况:

```rust
write!(f, "{}", value)?;
```

问号的逻辑是: 对write! 查看是否出错,如果出错返回错误; 否则继续执行后面语句.

## Fromatting

格式化要做的就是格式化一个字符串:

```rust
format!("{}", foo) -> "3735928559"
format!("0x{:X}", foo) -> "0xDEADBEEF"
format!("0o{:o}", foo) -> "0o33653337357"
```
可以看到相同的变量`foo`可以通过不同的进制表示为不同的字符串.

格式化的功能是通过实现`triats`实现的, 对于每一个参数类型对应一种trait. 最常见的trait就是前面提到的Display, 可以处理参数类型未指定`{}`的情况.


