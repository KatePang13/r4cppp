# Destructuring

上次，我们讨论了 Rust 的数据类型，一旦你在结构体中有一些数据，肯定会需要将这些数据取出来。对于结构体，Rust 有 字段访问，就像 C++ 一样。对于 `tuple`, `tuple struct` , `enum` ， 你必须使用 解构（库中有很多个方便的函数，但是它们内部都是使用destructuring ）。C++的结构体 解构 是从C++17才有的，所以可能对于Python这一类的多功能语言来说会更熟悉。思路就是：你可以 用一串本地变量来初始化一个结构体，当然也可以用一个结构体来填充一串本地变量。解构这个一开始很简单的功能，如今已经 成为Rust的最强大的功能之一。换句话说，解构结合了 模式匹配 和 局部变量赋值。

解构主要使用 `let` 语句 和 `match` 语句。当结构可能有不同的 `variants` 时(比如一个`enum`，有多个可能的变体), 使用 `match` 语句。

**`let` 表达式将变量拉出当前作用域，而match将变量引入新的作用域**，比较一下下面的示例：

```rust
fn foo(pair: (int, int)) {
    let (x, y) = pair;
    // we can now use x and y anywhere in foo

    match pair {
        (x, y) => {
            // x and y can only be used in this scope
        }
    }
}
```

这两种情况下，pattern 的语法（在上面的示例中let之后的 和 =>之前的  (x, y) 就是 pattern  ）几乎是相同的。你还可以在函数声明的参数位置中使用 pattern。示例：

```rust
fn foo((x, y): (int, int)) {
}
```

(这种方式更适用于  `struct` 和 `tuple-struct`， 而不是 `tuple` )

大多数初始化表达式都可以以解构模式出现，并且可以任意复杂。也包括引用和原始文字以及数据结构。

```rust
struct St {
    f1: int,
    f2: f32
}

enum En {
    Var1,
    Var2,
    Var3(int),
    Var4(int, St, int)
}

fn foo(x: &En) {
    match x {
        &Var1 => println!("first variant"),
        &Var3(5) => println!("third variant with number 5"),
        &Var3(x) => println!("third variant with number {} (not 5)", x),
        &Var4(3, St { f1: 3, f2: x }, 45) => {
            println!("destructuring an embedded struct, found {} in f2", x)
        }
        &Var4(_, ref x, _) => {
            println!("Some other Var4 with {} in f1 and {} in f2", x.f1, x.f2)
        }
        _ => println!("other (Var2)")
    }
}
```

这里请注意我们如何在模式中使用`＆`通过引用来解构结构，以及如何混合使用字面值（`5`，`3`，`St {...}`），通配符（`_`）和变量（`x`）

你可以使用 `_` 来标识 你想忽略的某一项，所以我们可以使用  `Var3(_)` 如果我们不关系这个`int`。

在第一个 `Var4` 分支中，解构 子结构体对象 ；在第二个 `Var4` 分支中，我们将整个子结构体对象 绑定给一个变量。 

你可以使用 `..` 来代表  tuple 或者 struct 的全部字段。因此，如果您想针对每个枚举元素做一些事情，但不关心枚举元素里的内容，则可以这样写：

```rust
fn foo(x: En) {
    match x {
        Var1 => println!("first variant"),
        Var2 => println!("second variant"),
        Var3(..) => println!("third variant"),
        Var4(..) => println!("fourth variant")
    }
}
```

解构 结构体时，字段不需要按顺序读取，你可以使用 `..` 来省略剩余的字段。示例：

```rust
struct Big {
    field1: int,
    field2: int,
    field3: int,
    field4: int,
    field5: int,
    field6: int,
    field7: int,
    field8: int,
    field9: int,
}

fn foo(b: Big) {
    let Big { field6: x, field3: y, ..} = b;
    println!("pulled out {} and {}", x, y);
}
```

对于 `struct` 有更简洁的方式，你可以直接使用字段名称，会直接根据字段名称创建对应的局部变量。下面的`let` 语句 展示了如果创建2个新的局部变量  x 和 y。 示例：

```rust
fn foo(b: Big) {
    let Big { field6, field3, .. } = b;
    println!("pulled out {} and {}", field3, field6);
}
```

Now we create local variables with the same names as the fields, in this case
`field3` and `field6`.

这里，我们创建了与 `field` 同名的局部变量，即示例中的 `field3` 和 `field6` 。



这里还有一些Rust 解构的 技巧。假设你想**在pattern中获取结构体对象中的引用**。你不能使用 `&` ，因为这是要匹配一个已有的引用（某个字段原先就是引用类型），而不是创建一个新的引用（因此会有解引用的效果）。示例：

```rust
struct Foo {
    field: &'static int    //Foo 中有一个 field, 它是一个引用
}

fn foo(x: Foo) {
    let Foo { field: &y } = x;	//获取 Foo x中 的field
}
```

这里，y 是 `int` 类型，是 `x.field` 的一个拷贝。

要**在 `pattern` 中 创建对 某个字段的引用，你可以使用  `ref` 关键字**。示例：

```rust
fn foo(b: Big) {
    let Big { field3: ref x, ref field6, ..} = b;
    println!("pulled out {} and {}", *x, *field6);
}
```

这里  `x` 和  `field6` 类型都是  `&int` ，是对  b结构体中前两个 `field` 的引用。



另一个技巧是，当你解构一个复合对象是，你可能希望 命名中间对象 和 各个字段。

回到我们之前的一个例子，我们有一个这样的pattern `&Var4(3, St{f1: 3, f2: x}, 45)` 。

- 在这个pattern 中，我们命名了子对象的各个字段   `&Var4(3, St{f1: 3, f2: x}, 45)` ；

- 如果你想命名这个子结构对象，你可以写成  `&Var4(3, s, 45)` ,将这个子对象绑定到 s；

- 如果你既想命名 子对象，又想命名子对象的字段。可以写成  `&Var4(3, s @ St{f1: 3, f2: x}, 45)` 



这里几乎涵盖了Rust模式匹配的各种方式。当然还有一些特性没有介绍到，例如匹配向量(matching vector)，希望你可以理解`match` 和 `let` 的使用方法，知道可以用它实现一些强大的东西。下次，我将介绍`match`和`borrowing`之间的一些微妙的相互作用，这在学习Rust时给我造成了很大的困扰。

