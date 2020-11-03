# Control flow

## If

Rust 的`if`语句 和C++ 基本一样。少数区别在于：

- 是 {} 是必需的，而条件表达式的 () 不是必需的。
- if 是 一个表达式，因此可以像 C++ 的三目运算符 '?:' 那样使用
  - 回忆一下上一节的知识，如果一个代码块内的最后一个表达式不是以 `;`终止，则它将作为代码块的值

Rust 没有 `?:` ，下面2个函数实现的是类似`?:`的功能：

```rust
fn foo(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}
```

The first is a fairly literal translation of what you might write in C++. The
second is better Rust style.

第一个函数更像是 C++代码的直译，而第二个函数是更好的 Rust 风格。

你也可以写成  `let x = if ...` 


## Loops

Rust has while loops, again just like C++:

Rust 跟C++ 一样，也有 `while` 循环

```rust
fn main() {
    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1;
    }
}
```

Rust 没有 `do ... while`循环，不过有 `loop` 语句，可以无限循环：

```rust
fn main() {
    loop {
        println!("Just looping");
    }
}
```

Rust 同样也有 `break` 和 continue 。


## For loops

Rust also has `for` loops, but these are a bit different. Lets say you have a
vector of integers and you want to print them all (we'll cover vectors/arrays,
iterators, and generics in more detail in the future. For now, know that a
`Vec<T>` is a sequence of `T`s and `iter()` returns an iterator from anything
you might reasonably want to iterate over). A simple `for` loop would look like:

Rust 也有 `for`循环，但是和C++有点不一样。假设你有一个整数数组，你想要打印所有的元素（后续，我们将更详细地讨论 vectors/arrays/iterators/generics ）。目前我们只需要知道，`Vec<T>` 是一个 `T`类型变量的序列， `iter()` 从你 想要迭代的任何对象中返回一个迭代器。下面是`for` 循环的一个简单示例：

```rust
fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}
```

这里用 &all/all 替换 all.iter() 也是可以的。

如果你想要用索引来遍历 all  (跟标准C++的for循环类似)， 你可以这样做：

```rust
fn print_all(all: Vec<i32>) {
    for i in 0..all.len() {
        println!("{}: {}", i, all[i]);
    }
}
```

这里  `len`函数的作用是很明显的。TODO range notation。

更Rust风格的写法是这样的，使用一个enumerate迭代器：

```rust
fn print_all(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}
```

`enumerate()`  从 iter() 开始迭代，并在迭代过程中产生当前计数和当前元素

**注意：以上这几个print_all，传递的都是 Vec<i32> ，一次调用后，实参被move,不能再调用第二次**。

```
error[E0382]: use of moved value: `nums`
 --> src/main.rs:5:15
  |
3 |     let nums = vec![1, 2, 3, 4];
  |         ---- move occurs because `nums` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
4 |     print_all(nums);
  |               ---- value moved here
5 |     print_ref(nums);
  |               ^^^^ value used here after move
```

下面的例子会涉及更高级的语言特性，将在 [Borrowed Pointers](borrowed.md) 进行详细介绍。假设你有一个整型的vector，希望调用这个函数，传递vector的指针，函数内对vector做原地修改。这里的for循环使用 mutable 迭代器，获取 mutable 的引用。`*` 是解引用操作符，与C++类似。

```rust
fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}

//调用
//类型声明的时候必须是mut,才可以转 &mut
let mut nums = vec![1, 2, 3, 4];
double_all(&mut nums);
```


## Switch/Match

Rust 有 match 表达式，类似 C++的 switch语句，比 switch 更强大。简单的例子如下：

```rust
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y),
    }
}
```

There are some syntactic differences - we use `=>` to go from the matched value
to the expression to execute, and the match arms are separated by `,` (that last
`,` is optional). There are also some semantic differences which are not so
obvious: the matched patterns must be exhaustive, that is all possible values of
the matched expression (`x` in the above example) must be covered. Try removing
the `y => ...` line and see what happens; that is because we only have matches
for 0, 1, and 10, but there are obviously lots of other integers which don't get
matched. In that last arm, `y` is bound to the value being matched (`x` in this
case). We could also write:

这里有一些语法差异：

- 我们使用 `=>` 来做分支跳转，各个分支用 `,` 分割（最后一个`,`可有可无）。

还有一些比较不明显的差异，

- **匹配项必须是完备的**，所有可能的情况都必须覆盖。可以尝试移除  `y=>...` 行，看看会发生什么；
  - 那是因为我们只有0、1和10的匹配项，但是显然还有许多其他整数不匹配。在最后一个分支中，“ y”绑定到要匹配的值（在这种情况下为“ x”）。我们也可以这样写：

```rust
fn print_some(x: i32) {
    match x {
        x => println!("x is something else {}", x)
    }
}
```

在这个match内部引入了一个新的变量x, 这跟在一个scope内部定义一个变量是一样的。

如果你不希望给这个变量命名，可以使用`_` 指代 匿名变量，就像是有一个通配符匹配。

如果你不想做任何操作，可以写成一个 空分支 `{}`。

```rust
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        _ => {}
    }
}
```

另一个语义差异是：没有从一个分支到另一条分支的顺延（C++的 switch ，一个分支如果没有break，会顺序往下一个分支走），所以它的工作原理类似于 `if ... else if ... else`。我们将在以后的文章中看到match的功能非常强大。这里我想介绍的是一些额外的特性，在match 内可以配合 条件判断使用，具体如下所示：

```rust
fn print_some_more(x: i32) {
    match x {
        0 | 1 | 10 => println!("x is one of zero, one, or ten"),
        y if y < 20 => println!("x is less than 20, but not zero, one, or ten"),
        y if y == 200 => println!("x is 200 (but this is not very stylish)"),
        _ => {}
    }
}
```

和`if`表达式一样， match语句实际上是表达式，因此我们可以将最后一个示例重写为：

```rust
fn print_some_more(x: i32) {
    let msg = match x {
        0 | 1 | 10 => "one of zero, one, or ten",
        y if y < 20 => "less than 20, but not zero, one, or ten",
        y if y == 200 => "200 (but this is not very stylish)",
        _ => "something else"
    };

    println!("x is {}", msg);
}
```

注意  这个 {} 后面的分号，因为 let 语句是一个语句，格式必须是  `let msg = ...; ` 。 match 表达式不需要分号，但是 let语句需要。

Motivation:  Rust match 语句 规避了 C++ switch 语句中的常见BUG：如果你漏掉了 break，就会造成意料之外的向下一个分支执行；如果你为 enum 添加了一个case，编译器将会验证你保证所有的case都有在match中被覆盖。




## Method call

Rust中的方法与C++很相似，方法总是使用 `.`操作符来调用（而不是`->` ,在另一篇文章会有更多的介绍）。

我们上面看到了 `len`, `iter`的方法的使用。

后续我们会深入细节，介绍方法的定义和调用。

在方法上，Rust 与C++, java 整体上是一致。