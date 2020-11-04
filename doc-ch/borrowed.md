# Borrowed pointers

上一篇中我介绍了 unique pointers ，本篇将讨论的是另一种指针，在Rust程序中，它是更常见的：borrowed pointers (又叫 borrowed 引用，或者引用)。

如果我们想拥有一个已有变量的引用（unique ptr 是创建一个新变量，同时创建一个指向它的指针），我们必须使用 `&`, 声明一个 borrowed reference。这些可能是Rust中最常见的一种指针，如果您想为C ++指针或引用填充某些内容（例如，通过引用将参数传递给函数），就可以用borrowed reference 。

我们使用`&`操作符来创建一个 borrowed reference 并指定引用类型，并使用`*`解引用。自动解引用的规则跟unique ptr 是一致的。示例：

```rust
fn foo() {
    let x = &3;   // type: &i32
    let y = *x;   // 3, type: i32
    bar(x, *x);
    bar(&y, y);
}

fn bar(z: &i32, i: i32) {
    // ...
}
```

`&` 操作符 不分配内存（我们只是为已存在的值创建一个 borrowed reference ），一个borrowed reference离开scope时，没有内存被释放。

borrowed reference 不是独占的：你可以创建多个 borrowed reference 来指向同一个值。示例：

```rust
fn foo() {
    let x = 5;                // type: i32
    let y = &x;               // type: &i32
    let z = y;                // type: &i32
    let w = y;                // type: &i32
    println!("These should all be 5: {} {} {}", *w, *y, *z);
}
```

和值一样，borrowed reference 默认是不可变的。你也可以使用`&mut` 来获取 可变引用，或者表示可变引用类型。可变 borrowed reference 是 独占的（一个值只能有一个borrowed reference，并且只有当前没有不可变引用的情况下，才能申请到可变引用）。您可以在需要不可变引用时使用可变引用，反之亦然。

这里将所有示例放在一起：

```rust
fn bar(x: &i32) { ... }
fn bar_mut(x: &mut i32) { ... }  // &mut i32 is a reference to an i32 which
                                 // can be mutated

fn foo() {
    let x = 5;
    //let xr = &mut x;     // Error - can't make a mutable reference to an
                           // immutable variable
    let xr = &x;           // Ok (creates an immutable ref)
    bar(xr);
    //bar_mut(xr);         // Error - expects a mutable ref

    let mut x = 5;
    let xr = &x;           // Ok (creates an immutable ref)
    //*xr = 4;             // Error - mutating immutable ref
    //let xr = &mut x;     // Error - there is already an immutable ref, so we
                           // can't make a mutable one

    let mut x = 5;
    let xr = &mut x;       // Ok (creates a mutable ref)
    *xr = 4;               // Ok
    //let xr = &x;         // Error - there is already a mutable ref, so we
                           // can't make an immutable one
    //let xr = &mut x;     // Error - can only have one mutable ref at a time
    bar(xr);               // Ok
    bar_mut(xr);           // Ok
}
```

要注意的是，引用的可变性与对应变量的可用性是不相关的。这一点和C++是类似的，指针的可变性和数据的可变性是相互独立的。unique pointer 正好相反，它的可用性会关联到变量的可用性。示例：

```rust
fn foo() {
    let mut x = 5;
    let mut y = 6;
    let xr = &mut x;
    //xr = &mut y;        // Error xr is immutable

    let mut x = 5;
    let mut y = 6;
    let mut xr = &mut x;
    xr = &mut y;          // Ok

    let x = 5;
    let y = 6;
    let mut xr = &x;
    xr = &y;              // Ok - xr is mut, even though the referenced data is not
}
```

如果一个可变值是 borrowed 状态（有borrowed ref）， 则在 borrowed期间，它是 不可变的。一旦 borrowed pointer 离开 scope，这个值又变成可变的。 unique pointer 正好相反，一旦被移动后，就永远不可用。

示例：

```rust
fn foo() {
    let mut x = 5;            // type: i32
    {
        let y = &x;           // type: &i32
        //x = 4;              // Error - x has been borrowed
        println!("{}", x);    // Ok - x can be read
    }
    x = 4;                    // OK - y no longer exists
}
```

为一个值获取可变引用也是一样的：这个值不能被修改。通常，**在Rust中，只能通过一个变量或指针修改数据**。此外，**如果我们具有可变的引用，就不能获取不可变的引用**。这限制了我们对 底层值 的使用方式：

```rust
fn foo() {
    let mut x = 5;            // type: i32
    {
        let y = &mut x;       // type: &mut i32
        //x = 4;              // Error - x has been borrowed
        //println!("{}", x);  // Error - requires borrowing x
    }
    x = 4;                    // OK - y no longer exists
}
```

和C++不同，Rust 不会为你自动为你创建 值的引用。因此，如果函数通过引用获取参数，则调用者必须引用实际参数。但是，**指针类型会自动转换为引用**：

```rust
fn foo(x: &i32) { ... }

fn bar(x: i32, y: Box<i32>) {
    foo(&x);
    // foo(x);   // Error - expected &i32, found i32
    foo(y);      // Ok
    foo(&*y);    // Also ok, and more explicit, but not good style
}
```

## `mut` vs `const`

这里我们对 Rust的`mut` 和C++的 `const` 进行比较。表面上它们是相互对应的。Rust中的值默认是 不可变的，可以使用`mut` 显式声明为 可变。 C++中的值默认是可变的，可以使用 `const` 声明为不可变。更微妙和更重要的区别是**C ++的 const 不变性 仅适用于值的当前使用，而Rust的不变性适用于值的所有使用**。因此，**在C ++中，如果我有一个const变量，其他人可能会对它有非const引用，并且它可能在我不知情的情况下发生变化。在Rust中，如果您有一个不可变的变量，可以保证它不会改变**。

如上所述，所有可变变量都是唯一的。因此，

- 如果你拥有一个可变的值，你可以确定除非你去做修改，或者它不会改变。

- 此外，您可以自由更改它，因为你知道没有其他人会依赖它的不变性。



## Borrowing and lifetimes

Rust的其中一个主要的安全性目标是 避免 空悬指针（一个指针的生命周期超过它指向内存的生命周期）。在Rust，是不可能持有 空悬的 borrowed reference 的。 创建一个引用，只有变量存活时间比引用长，才是合法的（当然，可以一样长）。  换句话说，引用的生命周期 必须比 它指向的内存。

```rust
fn foo() {
    let x = 5;
    let mut xr = &x;  // Ok - x and xr have the same lifetime
    {
        let y = 6;
        //xr = &y     // Error - xr will outlive y
    }                 // y is released here
}                     // x and xr are released here
```

在上面的例子中，  xr 和 y 的生命周期是不一样的，因为y的起始时间晚于xr，但是更有意义的是生命周期的结束，因为无论如何您都**无法在变量存在之前引用它** —— Rust强制执行的这种限制，使得其比C ++更安全。



## Explicit lifetimes

在使用 borrowed pointers 一段时间后，您可能会遇到具有显式 生命周期的borrowed pointers。它们的语法`为＆'a T` ()`&T`    [cf.](https://en.wikipedia.org/wiki/Cf.)。这是一个很大的话题，因为我需要同时介绍生命周期多态性，因此我将其留在另一篇文章中（不过，有一些不常见的指针类型需要首先介绍）。现在，我只想说**`＆T`是 `＆'a T`的简写，其中a是当前作用域，即声明该类型的作用域**。

