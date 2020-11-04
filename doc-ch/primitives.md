# Primitive types and operators

Rust has pretty much the same arithmetic and logical operators as C++. `bool` is
the same in both languages (as are the `true` and `false` literals). Rust has
similar concepts of integers, unsigned integers, and floats. However the syntax
is a bit different. Rust uses `isize` to mean an integer and `usize` to mean an
unsigned integer. These types are pointer sized. E.g., on a 32 bit system,
`usize` means a 32 bit unsigned integer. Rust also has explicitly sized types
which are `u` or `i` followed by 8, 16, 32, or 64. So, for example, `u8` is an 8
bit unsigned integer and `i32` is a 32 bit signed integer. For floats, Rust has
`f32` and `f64`.

Numeric literals can take suffixes to indicate their type (using `i` and `u`
instead of `isize` and `usize`). If no suffix is given, Rust tries to infer the
type. If it can't infer, it uses `isize` or `f64` (if there is a decimal point).
Examples:

Rust具有与C ++几乎相同的算术和逻辑运算符。

`bool` 在两种语言中都是相同的（`true, false` 也是相同的）。 

Rust具有类似的整数，无符号整数和浮点数的概念。但是语法有点不同。 

- Rust使用isize表示整数，使用usize表示无符号整数。这些类型是指针大小的。
  - 例如，在32位系统上，usize表示32位无符号整数。 
- Rust还具有显式调整大小的类型，即u或i，后跟8、16、32或64。
  - 例如，u8是8位无符号整数，而i32是32位有符号整数。

- 对于浮点数，Rust具有f32和f64。 

数字文字可以带后缀指示其类型（使用i和u代替isize和usize）。如果没有给出后缀，Rust会尝试推断类型。如果无法推断，则使用isize或f64（如果有小数点）。例子如下：

```rust
fn main() {
    let x: bool = true;
    let x = 34;   // type isize
    let x = 34u;  // type usize
    let x: u8 = 34u8;
    let x = 34i64;
    let x = 34f32;
}
```

As a side note, Rust lets you redefine variables so the above code is legal -
each `let` statement creates a new variable `x` and hides the previous one. This
is more useful than you might expect due to variables being immutable by
default.

Numeric literals can be given as binary, octal, and hexadecimal, as well as
decimal. Use the `0b`, `0o`, and `0x` prefixes, respectively. You can use an
underscore anywhere in a numeric literal and it will be ignored. E.g,

另一方面，Rust 允许你重定义变量，所以下面的代码是合法的: 

每个 let 语句创建一个新的变量x 并隐藏之前的x。由于变量在默认情况下是不可变的，因此这个比您预期的要有用。

```rust
fn main() {
    let x = 12;
    let x = 0b1100;
    let x = 0o14;
    let x = 0xe;
    let y = 0b_1100_0011_1011_0001;
}
```

Rust具有字符和字符串，但是由于它们是Unicode，因此与C ++有点不同。我将在介绍完指针，引用和向量（数组）之后，介绍字符串。



数字类型转换：

- Rust不会隐式强制指定数字类型。
  - 通常，Rust具有比C ++更少的隐式强制和子类型化。

- Rust使用as关键字进行显式强制类型和强制类型转换。
  - 任何数字值都可以转换为其他数字类型。
  -  as不能用于在布尔类型和数字类型之间进行转换。

示例：

```rust
fn main() {
    let x = 34u as isize;   // cast usize to isize
    let x = 10 as f32;      // isize to float
    let x = 10.45f64 as i8; // float to i8 (loses precision)
    let x = 4u8 as u64;     // gains precision
    let x = 400u16 as u8;   // 144, loses precision (and thus changes the value)
    println!("`400u16 as u8` gives {}", x);
    let x = -3i8 as u8;     // 253, signed to unsigned (changes sign)
    println!("`-3i8 as u8` gives {}", x);
    //let x = 45u as bool;  // FAILS!
}
```

Rust 有以下操作符：

| Type                                   | Operators                        |
| -------------------------------------- | -------------------------------- |
| Numeric   算术运算符                   | `+`, `-`, `*`, `/`, `%`          |
| Bitwise     位运算符                   | `\|`, `&`, `^`, `<<`, `>>`       |
| Comparison   比较运算符                | `==`, `!=`, `>`, `<`, `>=`, `<=` |
| Short-circuit logical   短路逻辑运算符 | `\|\|`, `&&`                     |

所有这些行为都与C ++中的行为相同，但是，Rust在哪些类型可以使用哪些运算符是比较严格的：

- 逐位运算符只能应用于整数，
- 而逻辑运算符只能应用于布尔值。 
- Rust具有 `-` 一元运算符，该运算符可对数字取反。 
- ！运算符取反布尔值，或者将整数类型的每个位取反（在后一种情况下，相当于C ++中的 `~` ）。 
- Rust具有与C ++中相同的复合赋值运算符，例如`+ =`，但没有递增或递减运算符（例如 `++` ）。