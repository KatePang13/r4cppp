# Data types

本篇中，我将讨论 Rust 的数据类型。这些大体上等效于C ++中的类，结构和枚举。其中一个不同是Rust的数据和行为是严格分开的，相比于 C++或 java 等 面向对象语言。行为 通过函数来定义，可以定义在 `traits` 和 `impls` （implementations）中，但是 `traits` 不能包含数据，它们和java中的接口很像。后面的章节我们会讨论  `traits` 和 `impls` 。本章讨论的的都是关于数据的。

## Structs

rust struct 就像 C/C++ 中不带方法的 struct。struct 就是一个包含若干个命名字段的列表。语法如下所示：

```rust
struct S {
    field1: i32,
    field2: SomeOtherStruct
}
```

这里，我们定义了一个 名为 `S`的 结构体。

- 每个字段使用 `,` 分隔；
- 最后一个字段的 `,` 可以省略。

定义一个结构体即引入一个新的类型。这里，我们可以 将 `S`作为一个类型使用。`SomeOtherStruct` 这里假设是另外一个自定义的结构体（在例子中作为类型来使用），并且（和C++一样）它包含在值中，也就是说，内存中没有指向另一个结构对象的指针

结构体中的字段可以使用 `.` 和字段名 进行访问。示例：

```rust
fn foo(s1: S, s2: &S) {
    let f = s1.field1;
    if f == s2.field1 {
        println!("field1 matches!");
    }
}
```

这里  `S1` 是一个结构体对象，以值的形式传递；`S2` 是一个结构体对象，以 引用的形式传递。

结构体使用 struct literals (结构体列表) 来初始化，示例如下：

```rust
fn foo(sos: SomeOtherStruct) {
    let x = S { field1: 45, field2: sos };  // initialise x with a struct literal
    println!("x.field1 = {}", x.field1);
}
```

**结构体不可以递归**；就是说，字段的类型不能与结构体的类型一样。这是因为结构是值语义的。举个例子，`struct R {r: Option<R>}`是非法的，会造成编译器错误（有关Option的更多信息，后面将有讨论）。如果你需要这样的结构体，你可以使用指针；**类型的指针递归是允许的**。示例：

```rust
struct R {
    r: Option<Box<R>>
}
```

如果上面的结构中没有Option，那么将无法实例化该结构，Rust会发出错误信号。

没有字段的结构在其定义或文字使用中均不使用花括号。任何定义语句都需要终止的分号，不过大概只是为了便于解析。

```rust
struct Empty;

fn foo() {
    let e = Empty;
}
```

## Tuples

Tuple(元组) ，元组是匿名的异构数据序列。作为一个类型，它们在括号中声明为一个若干数据类型的序列。由于没有名称，因此按结构进行标识。比如， `(i32, i32) ` 是一个 整数对，`(i32, f32, S)` 是一个三元组。元组值的初始化方法与声明元组类型的方法相同，就是用值代替字段的类型，比如 `(4, 5)` 。示例：

```rust
// foo takes a struct and returns a tuple
fn foo(x: SomeOtherStruct) -> (i32, f32, S) {
    (23, 45.82, S { field1: 54, field2: x })
}
```

元组可以使用  `let` 表达式来 解构。示例：

```rust
fn bar(x: (i32, i32)) {
    let (a, b) = x;
    println!("x was ({}, {})", a, b);
}
```

后续我们将详细讨论 解构的方法 。


## Tuple structs

Tuple structs 元组结构体，是命名的元组，或者说是 含 匿名字段的结构体。它使用`struct` 关键字，一个括号内的类型列表和一个分号来声明。这样的声明将其名称作为类型引入。**必须通过解构（和元组一样）来访问其字段，而不是通过名称来访问**。元组结构不是很常见。

```rust
struct IntPoint (i32, i32);

fn foo(x: IntPoint) {
    let IntPoint(a, b) = x;  // Note that we need the name of the tuple
                             // struct to destructure.
    println!("x was ({}, {})", a, b);
}
```



## Enums

`enum` 枚举，类似于 C++ 的 `enum` 或者 `union`， 枚举是可以采用多个值的类型。最简单的枚举就像C ++枚举一样

```rust
enum E1 {
    Var1,
    Var2,
    Var3
}

fn foo() {
    let x: E1 = Var2;
    match x {
        Var2 => println!("var2"),
        _ => {}
    }
}
```

However, Rust enums are much more powerful than that. Each variant can contain
data. Like tuples, these are defined by a list of types. In this case they are
more like unions than enums in C++. Rust enums are tagged unions rather than untagged unions (as in C++). 

但是，Rust的`enum` 功能更强大，每个变体都可以包含数据。就像元组一样，这些是由一个类型列表定义的，在这种情况下，它更像是 C++中的 `union` 而不是 `enum` 。Rust `enum` 更像是C++中的 带tag 的 `union` 而不是 不带 tag 的 union。 这意味着您不会在运行时将枚举的一个变体误认为另一个变体 <sup>[1](#1)</sup>. 示例：

```rust
enum Expr {
    Add(i32, i32),
    Or(bool, bool),
    Lit(i32)
}

fn foo() {
    let x = Or(true, false);   // x has type Expr
}
```

在Rust中使用枚举可以更好地处理许多面向对象多态的简单情况。

`enum` 经常是配合 `match` 一起使用的。记住，这和 C++ 的 `switch` 是类似的。下次，我将更深入地探讨这些内容，和解构 数据的各种方法。示例：

```rust
fn bar(e: Expr) {
    match e {
        Add(x, y) => println!("An `Add` variant: {} + {}", x, y),
        Or(..) => println!("An `Or` variant"),
        _ => println!("Something else (in this case, a `Lit`)"),
    }
}
```

`match` 的每个分支匹配 `Expr` 的一个 变体。所有的变体都必须覆盖。最后一个分支 `_` 覆盖所有剩余的变体，虽然在示例中只有`Lit`一种剩余变体 。 一个变体中的任意数据都可以绑定变量。Add 分支中，我们将两个 `i32`绑定给2个变量  x, y。如果我不关系这些数据，可以使用 `..` 来匹配任意数据，就像上面的 `Or`分支。




## Option

Rust中一个特别常见的枚举是Option，它有2个变体，分别是 `Some` 和 `None`。

- `None` 没有数据
- `Some` 有 唯一的一个字段，类型为 `T` 。

(Option 是一个通用的`enum`， 后面我们会讨论到，但这里希望C ++的经验可以帮助你清楚地了解总体思路 )。

Option 用于声明一个值可能有，可能没有。C++中你经常使用 空指针 来检查合法性，来表明一个值某些情况下可能为定义，未初始化，或者 是错误的。

Rust中 你应该使用 `Option`。 **使用 `Option` 更安全**，因为它**强制你在使用变量前必须要做检查**；这样你就不可能去解引用一个空指针了。

**`Option` 也更加通用，你可以用它来携带 值或者指针**，示例：

```rust
use std::rc::Rc;

struct Node {
    parent: Option<Rc<Node>>,
    value: i32
}

fn is_root(node: Node) -> bool {
    match node.parent {
        Some(_) => false,
        None => true
    }
}
```

这里，parent 字段 可以是  `None` 或者 包含一个 `Rc<Node>` 的`Some` 。 在该示例中，我们没有真正发挥它的威力，但在实际开发中会经常使用。



## Inherited mutability and Cell/RefCell

Rust的本地变量默认是不可变的，可以使用 `mut` 标记为 可变**。我们不会将 `struct` 或者 `enum` 中的字段标记为可变，它们的可变性是遗传的**。这意味着 **一个字段的可变性取决于它所属的结构体对象的可变性**。示例：

```rust
struct S1 {
    field1: i32,
    field2: S2
}
struct S2 {
    field: i32
}

fn main() {
    let s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s is deeply immutable, the following mutations are forbidden
    // s.field1 = 46;
    // s.field2.field = 24;

    let mut s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s is mutable, these are OK
    s.field1 = 46;
    s.field2.field = 24;
}
```

可变性异常会被 引用 终止。这和C++是类似的，在C ++中，您可以通过const对象中的指针来修改非const对象。如果你想要一个引用字段是可变的，你必须在这个字段类型上使用 `&mut` 。

```rust
struct S1 {
    f: i32
}
struct S2<'a> {
    f: &'a mut S1   // mutable reference field
}
struct S3<'a> {
    f: &'a S1       // immutable reference field
}

fn main() {
    let mut s1 = S1{f:56};
    let s2 = S2 { f: &mut s1};
    s2.f.f = 45;   // legal even though s2 is immutable
    // s2.f = &mut s1; // illegal - s2 is not mutable
    let s1 = S1{f:56};
    let mut s3 = S3 { f: &s1};
    s3.f = &s1;     // legal - s3 is mutable
    // s3.f.f = 45; // illegal - s3.f is immutable
}
```

( S2, S3中的  `'a` 参数 是 生命期参数，之后我们会讨论到 )。 

有时，虽然对象在逻辑上是不可变的，但它的某些部分需要内部可变。考虑各种类型的缓存或引用计数就是这种情况（由于可以通过析构函数观察更改引用计数的效果，因此不会提供真正的逻辑不变性）。在C ++中，即使对象是const，也可以使用mutable关键字来允许这种突变。在Rust中，我们拥有有 Cell和RefCell结构。这些允许对不可变对象的某些部分进行突变。尽管这很有用，但它意味着您需要注意，**当您在Rust中看到不可变对象时，某些字段实际上可能是可变的**。

RefCell和Cell使你可以绕开Rust关于突变和别名的严格规则。使用它们是安全的，因为它们确保动态遵循 Rust的，即使编译器无法确保静态保持不变。`Cell` 和 `RefCell` 都是单线程对象。

具有 拷贝语言的类型使用 `Cell`（几乎只有原始类型）。`Cell` 有 `get/set`方法来修改保存的值，`new` 方法来初始化一个 cell。

`Cell` 是一个非常简单的对象：它不需要做任何明智的事情，因为具有复制语义的对象无法将引用保留在其他地方（在Rust中），并且它们不能在线程之间共享，因此没有太多可能出错的地方。

具有移动语义的类型 使用 `RefCell` ，Rust 中的大多数类似都是 移动语义的，结构体对象就是一个常见的例子。`RefCell`  也是有 `new` 方法用以创建，`set`方法由于修改值。要获取 `RefCell` 的值，必须使用 borrow方法来 借用它，(borrow方法包含：borrow, borrow_mut, try_borrow, try_borrow_mut)，这些方法会返回一个 borrowed 引用，指向 `RefCell` 中的对象。

这些方法遵循与静态借用相同的规则：**只能有一个可变借用，并且不能同时有可变和不可变借用**。**否则，您会触发运行时失败， 而不是编译错误**。

try_变体方法 会返回一个Option：**如果可以借用该值，则将获得Some（val）；如果不能，则将返回None**。如果值已经被其他人借用，则调用set也将失败。

这里有个示例，在 一个引用技术指针上 使用 `RefCell` (这是一个常见的用例) ：

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct S {
    field: i32
}

fn foo(x: Rc<RefCell<S>>) {
    {
        let s = x.borrow();
        println!("the field, twice {} {}", s.field, x.borrow().field);
        // let s = x.borrow_mut(); // Error - we've already borrowed the contents of x
    }

    let mut s = x.borrow_mut(); // OK, the earlier borrows are out of scope
    s.field = 45;
    // println!("The field {}", x.borrow().field); // Error - can't mut and immut borrow
    println!("The field {}", s.field);
}

fn main() {
    let s = S{field:12};
    let x: Rc<RefCell<S>> = Rc::new(RefCell::new(s));
    foo(x.clone());

    println!("The field {}", x.borrow().field);
}
```

如果你使用  `Cell` `RefCell` , 你应该 尝试将 它们放在 尽可能小的对象上。也就是说，尽量将它们放在结构体的某些字段上，而不是放在整个结构体上。可以联想 单线程锁，**细粒度的锁定效果更好**，因为您更有可能避免命中锁。


##### 1

C++17， 引入了   `std::variant<T>`类型， 这个更接近 Rust 的 `enum` ，而不是 `union`

##### 2

C++17, 引入了 `std::optional<T>`， 是 Rust 中 的 `Option`   之外的最佳选择

