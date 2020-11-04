# Unique pointers

Rust是一种系统语言，因此必须为您提供对内存的原始访问。它通过指针完成此操作（与C ++中一样）。**指针是Rust和C ++在语法和语义上截然不同的地方**。 **Rust通过检查指针的类型来增强内存安全性**。这是它相对于其他语言的主要优势之一。尽管类型系统有些复杂，但是您可以获得内存安全性和裸机性能。

我本打算在一篇文章中介绍Rust的所有指针，但是我认为这个主题太大了。因此，本文仅涉及一种, `unique pointers`， 其他类型将在后续文章中讨论。

首先，我们看一个没有指针的示例：

```rust
fn foo() {
    let x = 75;

    // ... do something with `x` ...
}
```

当foo 结束时，x超出scope（在Rust和C ++都是这样的）。这意味着无法再访问该变量，并且可以重复使用该变量的内存。

在Rust, 每个类型 `T` 我们都可以通过 `Box<T>` 来拥有（独占）一个指向`T`的指针。我们使用 `Box::new(...)` 来在 堆上分配一个空间，并用指定的值来初始化这个空间。这个和 C++中的 `new`是类似的。

示例：

```rust
fn foo() {
    let x = Box::new(75);
}
```

这里  x 是一个指针，指向向堆内存中一个特定的位置，这个位置包含的值是 75。

x 是  `Box<isize>` 类型；所以我们也可以写成  `let x: Box<isize> = Box::new(75);` 。

在C++中，写成  `int* x = new int(75);` 。和C++不同的是，Rust 会为我们整理内存，所以这里不需要调用  free 或者 delete[1](#1). 。 Unique pointers 的行为 和 值 是类似的：当离开 scope 时，它们会被删除。

在我们的例子中，foo 结束后，x 不能再被访问，x指向的内存可以被重新利用。

和C++一样，使用 `*` 来解引用一个指针。

```rust
fn foo() {
    let x = Box::new(75);
    println!("`x` points to {}", *x);
}
```

与Rust中的原始类型一样，**拥有指针及其指向的数据默认情况下是不可变的**。与C ++不同，你**不能有一个指向不变数据的可变（唯一）指针**，反之亦然。**数据的可变性来自指针。**例如:

```rust
fn foo() {
    let x = Box::new(75);
    let y = Box::new(42);
    // x = y;         // Not allowed, x is immutable.
    // *x = 43;       // Not allowed, *x is immutable.
    let mut x = Box::new(75);
    x = y;            // OK, x is mutable.
    *x = 43;          // OK, *x is mutable.
}
```

Owning pointers can be returned from a function and continue to live on. If they
are returned, then their memory will not be freed, i.e., there are no dangling
pointers in Rust. The memory will not leak. However, it will eventually go out of
scope and then it will be freed. 

拥有的指针可以从函数中返回并继续存在。如果作为返回值返回，则内存不会被释放，即**Rust中没有悬空的指针**。内存不会泄漏。但是，它**将最终超出范围，然后将其释放**。例如。

E.g.,

```rust
fn foo() -> Box<i32> {
    let x = Box::new(75);
    x
}

fn bar() {
    let y = foo();
    // ... use y ...
}
```

这里，内存在 foo 里 初始化，并返回给 bar。x 从 foo返回，并存放在 y, 所以不会被删除。在 bar 结束时，y 离开scope，所以内存被回收。

Owning pointers are unique (also called linear) because there can be only one
(owning) pointer to any piece of memory at any time. This is accomplished by
move semantics. When one pointer points at a value, any previous pointer can no
longer be accessed. E.g.,

owning指针是独占的(也叫线性的)，因为对于任何一块内存的，在任何时刻只会有一个(owning)指针。这是通过移动语义来完成的。当一个指针指向一个值时，先前的指针都不再可访问。

```rust
fn foo() {
    let x = Box::new(75);
    let y = x;
    // x can no longer be accessed
    // let z = *x;   // Error.
}
```

同样，如果一个owning指针传递给另一个函数或存储到一个字段中，则这个指针不再可访问。

```rust
fn bar(y: Box<isize>) {
}

fn foo() {
    let x = Box::new(75);
    bar(x);
    // x can no longer be accessed
    // let z = *x;   // Error.
}
```

**Rust的唯一指针类似于C ++ std :: unique_ptrs**。在Rust中，与在C ++中一样，只能有一个指向值的唯一指针，并且当指针超出范围时，该值将被删除。 **Rust大部分是静态检查，而不是在运行时检查**。因此，在C ++中，访问其值已移动的唯一指针将导致运行时错误（因为它将为null）。在Rust中，这会产生编译时错误，在运行时不会出现这样的出错。

稍后我们将看到可以创建其他指针类型，来指向Rust中 Unique Pointers 的值。

这个与C ++类似。但是，在C ++中，这会导致在运行时，因为持有指向被释放内存的指针而产生错误。在Rust中这是不可能的（我们将在介绍Rust的其他指针类型时看到）。

我们前面个看到， owning 指针必须解引用才能使用它们的值。但是，方法调用会自动解引用，所以我们不需要用 `->` 或者 `*`来调用方法。从这一点上来说，Rust的指针 即像指针，又像引用。示例：

```rust
fn bar(x: Box<Foo>, y: Box<Box<Box<Box<Foo>>>>) {
    x.foo();
    y.foo();
}
```

Assuming that the type `Foo` has a method `foo()`, both these expressions are OK.

假设 Foo 类型拥有方法  `foo()` ，以上 的写法都是OK的。

**指定一个已存在的变量来 调用 `Box::new()` 不会引用原有的变量，而是进行拷贝**。  示例：

```rust
fn foo() {
    let x = 3;
    let mut y = Box::new(x);
    *y = 45;
    println!("x is still {}", x);
}
```

通常，Rust具有移动而不是复制语义（如上图所示，具有唯一的指针）。基本类型具有复制语义，因此在上面的示例中，值3被复制，但是对于更复杂的值，它将被移动。稍后我们将详细介绍。

但是，有时在编程时，我们需要对一个值的多个引用。为此，Rust提供了 borrowed pointers 。我将在下一篇文章中介绍。


##### 1

`std::unique_ptr<T>` 在C++11 引入，与 `Box<T>` 有相似的地方，也有不同的地方：

相同点:
* `std::unique_ptr<T>`  和 `Box<T>` 在离开scope后，都会自动被释放。
*  `std::unique_ptr<T>` 和 `Box<T>` 都只支持移动语义。

不同点:

1. C++ 11 允许由已有的指针来创建，因为允许多个 `unique_ptr` 指向 同一块内存。这在 `Box<T>` 是不允许的。
2. 解引用一个已经被转移到其他变量或函数的 `unique_ptr` ，会造成 未知的异常。Rust 会在编译时做检查，避免这种运行时错误。
3. 解引用`unique_ptr` 后，可变性取决于原变量的可用性；解引用 `Box<T>` 后，可变性取决于  `Box<T>`的可用性。



| Language | Code                                                     |
| -------- | -------------------------------------------------------- |
| Rust     | `let x = Box::new(75)`;                                  |
| C++11    | `const auto x =std::unique_ptr<const int>{new int{75}};` |
| C++14    | `const auto x =std::make_unique<const int>(75);`         |

