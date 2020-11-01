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

TODO also &all/all instead of all.iter()

If we want to index over the indices of `all` (a bit more like a standard C++
for loop over an array), you could do

```rust
fn print_all(all: Vec<i32>) {
    for i in 0..all.len() {
        println!("{}: {}", i, all[i]);
    }
}
```

Hopefully, it is obvious what the `len` function does. TODO range notation

A more Rust-like equivalent of the preceding example would be to use an
enumerating iterator:

```rust
fn print_all(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}
```

Where `enumerate()` chains from the iterator `iter()` and yields the current
count and the element during iteration.

*The following example incorporates more advanced topics covered in the section
on [Borrowed Pointers](borrowed.md).* Let's say you have a vector of integers
and want to call the function, passing the vector by reference and have the
vector modified in place. Here the `for` loop uses a mutable iterator which
gives mutable refererences - the `*` dereferencing should be familiar to C++
programmers:

```rust
fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}
```


## Switch/Match

Rust has a match expression which is similar to a C++ switch statement, but much
more powerful. This simple version should look pretty familiar:

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

```rust
fn print_some(x: i32) {
    match x {
        x => println!("x is something else {}", x)
    }
}
```

Here the `x` in the match arm introduces a new variable which hides the argument
`x`, just like declaring a variable in an inner scope.

If we don't want to name the variable, we can use `_` for an unnamed variable,
which is like having a wildcard match. If we don't want to do anything, we can
provide an empty branch:

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

Another semantic difference is that there is no fall through from one arm to the
next so it works like `if...else if...else`.

We'll see in later posts that match is extremely powerful. For now I want to
introduce just a couple more features - the 'or' operator for values and `if`
clauses on arms. Hopefully an example is self-explanatory:

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

Just like `if` expressions, `match` statements are actually expressions so we
could re-write the last example as:

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

Note the semi-colon after the closing brace, that is because the `let` statement
is a statement and must take the form `let msg = ...;`. We fill the rhs with a
match expression (which doesn't usually need a semi-colon), but the `let`
statement does. This catches me out all the time.

Motivation: Rust match statements avoid the common bugs with C++ switch
statements - you can't forget a `break` and unintentionally fall through; if you
add a case to an enum (more later on) the compiler will make sure it is covered
by your `match` statement.


## Method call

Finally, just a quick note that methods exist in Rust, similarly to C++. They
are always called via the `.` operator (no `->`, more on this in another post).
We saw a few examples above (`len`, `iter`). We'll go into more detail in the
future about how they are defined and called. Most assumptions you might make
from C++ or Java are probably correct.
