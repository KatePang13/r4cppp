# Destructuring pt2 - match and borrowing

When destructuring there are some surprises in store where borrowing is
concerned. Hopefully, nothing surprising once you understand borrowed references
really well, but worth discussing (it took me a while to figure out, that's for
sure. Longer than I realised, in fact, since I screwed up the first version of
this blog post).

涉及到 borrowing 的时候，解构 可能会有一些意外的问题。理论上，如果你已经很好地理解了 borrowed 引用，就不会有什么问题，但是这仍然是很值得讨论的。

Imagine you have some `&Enum` variable `x` (where `Enum` is some enum type). You
have two choices: you can match `*x` and list all the variants (`Variant1 =>
...`, etc.) or you can match `x` and list reference to variant patterns
(`&Variant1 => ...`, etc.). (As a matter of style, prefer the first form where
possible since there is less syntactic noise). `x` is a borrowed reference and
there are strict rules for how a borrowed reference can be dereferenced, these
interact with match expressions in surprising ways (at least surprising to me),
especially when you are modifying an existing enum in a seemingly innocuous way
and then the compiler explodes on a match somewhere.

假设你有一个 `&Enum` 变量 x (这里Enum 是一个 enum 类型)。你有两种选择：

- match `*x` 并列举所有 `variant`，`variant1 =>..., 等`
  - 考虑到代码风格，应该尽量采用第一种，语法更清晰简单。
- natch  `x`  并列举所有 `variant`的引用 ，`&variant1 =>..., 等`
  - x 是 一个 borrowed 引用，
    - 对于 什么情况下borrowed引用可以被解引用， 有一些很严苛的规则
    - borrowed 与  match 之间会有很惊喜的化学反应，特别是当你 修改一个已有的enum
      - 看似没有什么问题，但是编译器在 match 中 炸了	

Before we get into the details of the match expression, lets recap Rust's rules
for value passing. In C++, when assigning a value into a variable or passing it
to a function there are two choices - pass-by-value and pass-by-reference. The
former is the default case and means a value is copied either using a copy
constructor or a bitwise copy. If you annotate the destination of the parameter
pass or assignment with `&`, then the value is passed by reference - only a
pointer to the value is copied and when you operate on the new variable, you are
also operating on the old value.

在我们深入 match 表达式的细节之前，我们先来回顾以下 Rust 的值传递。在C++, 当我们给变量分配值时，或者将值传递给一个函数时，有2种方式：值传递和引用传递。前者是默认方式，这种方式意味着值发生了拷贝，或许是用拷贝构造器，或许是 字节拷贝 。如果用 `&` 注释形参或者 赋值目标，则该值将通过引用传递-仅复制指向该值的指针，并且当您对新变量做操作时，同时也是对旧值做了操作。

Rust has the pass-by-reference option, although in Rust the source as well as
the destination must be annotated with `&`. For pass-by-value in Rust, there are
two further choices - copy or move. A copy is the same as C++'s semantics
(except that there are no copy constructors in Rust). A move copies the value
but destroys the old value - Rust's type system ensures you can no longer access
the old value. As examples, `i32` has copy semantics and `Box<i32>` has move
semantics:

Rust 也可以 引用传递，要注意的是 Rust 的 原变量和目的变量都必须使用 `&` 标注。

Rust 的值传递，有2种选择：复制(copy) 或者 移动(move)

- copy  和 C++ 语义上是一致的 （不过Rust没有拷贝构造器）
- move  将旧值拷贝，但是销毁 旧值
  - Rust 类型系统确保你不会再访问到旧值

比如， `i32` 是 拷贝语义，`Box<i32>`  是移动语义。

```rust
    fn foo() {
    let x = 7i;
    let y = x;                // x is copied
    println!("x is {}", x);   // OK

    let x = box 7i;
    let y = x;                // x is moved
    //println!("x is {}", x); // error: use of moved value: `x`
}
```

You can also choose to have copy semantics for user-defined types
by implementing the `Copy` trait. One straightforward way to do that is 
to add `#[derive(Copy)]` before the definition of the `struct`. Not all
user-defined types are allowed to implement the `Copy` trait. All fields of 
a type must implement `Copy` and the type must not have a destructor. 
Destructors probably need a post of their own, but for now, an object 
in Rust has a destructor if it implements the `Drop`trait. 
Just like C++, the destructor is executed just before an object is 
destroyed.

自定义类型默认都是移动语义，你可以通过实现  `Copy` trait 来添加 移动语义。一个直接的方法是再定义 `struct`前添加 `#[derive(Copy)]` 。不是所有的自定义类型都允许实现  `Copy` trait 。`struct` 的所有`field`都必须实现`Copy` ，并且类型必须有析构函数。析构函数应该有自己的指责，但是目前，Rust中的对象如果实现了`Drop` trait，就认为具有析构函数。与C ++一样，析构函数在对象被销毁之前执行。

Now, it is important that a borrowed object is not moved, otherwise you would
have a reference to the old object which is no longer valid. This is equivalent
to holding a reference to an object which has been destroyed after going out of
scope - it is a kind of dangling pointer. If you have a pointer to an object,
there could be other references to it. So if an object has move semantics and
you have a pointer to it, it is unsafe to dereference that pointer. (If the
object has copy semantics, dereferencing creates a copy and the old object will
still exist, so other references will be fine).

OK, back to match expressions. As I said earlier, if you want to match some `x`
with type `&T` you can dereference once in the match clause or match the
reference in every arm of the match expression. Example:

```rust
enum Enum1 {
    Var1,
    Var2,
    Var3
}

fn foo(x: &Enum1) {
    match *x {  // Option 1: deref here.
        Var1 => {}
        Var2 => {}
        Var3 => {}
    }

    match x {
        // Option 2: 'deref' in every arm.
        &Var1 => {}
        &Var2 => {}
        &Var3 => {}
    }
}
```

In this case you can take either approach because `Enum1` has copy semantics.
Let's take a closer look at each approach: in the first approach we dereference
`x` to a temporary variable with type `Enum1` (which copies the value in `x`)
and then do a match against the three variants of `Enum1`. This is a 'one level'
match because we don't go deep into the value's type. In the second approach
there is no dereferencing. We match a value with type `&Enum1` against a
reference to each variant. This match goes two levels deep - it matches the type
(always a reference) and looks inside the type to match the referred type (which
is `Enum1`).

Either way, we must ensure that we (that is, the compiler) respect 
Rust's invariants around moves and references - we must not move any
part of an object if it is referenced. If the value being matched has copy
semantics, that is trivial. If it has move semantics then we must make sure that
moves don't happen in any match arm. This is accomplished either by ignoring
data which would move, or making references to it (so we get by-reference
passing rather than by-move).

```rust
enum Enum2 {
    // Box has a destructor so Enum2 has move semantics.
    Var1(Box<i32>),
    Var2,
    Var3
}

fn foo(x: &Enum2) {
    match *x {
        // We're ignoring nested data, so this is OK
        Var1(..) => {}
        // No change to the other arms.
        Var2 => {}
        Var3 => {}
    }

    match x {
        // We're ignoring nested data, so this is OK
        &Var1(..) => {}
        // No change to the other arms.
        &Var2 => {}
        &Var3 => {}
    }
}
```

In either approach we don't refer to any of the nested data, so none of it is
moved. In the first approach, even though `x` is referenced, we don't touch its
innards in the scope of the dereference (i.e., the match expression) so nothing
can escape. We also don't bind the whole value (i.e., bind `*x` to a variable),
so we can't move the whole object either.

We can take a reference to any variant in the second match, but not in the
dereferenced version. So, in the second approach replacing the second arm with `a
@ &Var2 => {}` is OK (`a` is a reference), but under the first approach we
couldn't write `a @ Var2 => {}` since that would mean moving `*x` into `a`. We
could write `ref a @ Var2 => {}` (in which `a` is also a reference), although
it's not a construct you see very often.

But what about if we want to use the data nested inside `Var1`? We can't write:

```rust
match *x {
    Var1(y) => {}
    _ => {}
}
```

or

```rust
match x {
    &Var1(y) => {}
    _ => {}
}
```

because in both cases it means moving part of `x` into `y`. We can use the 'ref'
keyword to get a reference to the data in `Var1`: `&Var1(ref y) => {}`. That is
OK, because now we are not dereferencing anywhere and thus not moving any part
of `x`. Instead we are creating a pointer which points into the interior of `x`.

Alternatively, we could destructure the Box (this match is going three levels
deep): `&Var1(box y) => {}`. This is OK because `i32` has copy semantics and `y`
is a copy of the `i32` inside the `Box` inside `Var1` (which is 'inside' a
borrowed reference). Since `i32` has copy semantics, we don't need to move any
part of `x`. We could also create a reference to the int rather than copy it:
`&Var1(box ref y) => {}`. Again, this is OK, because we don't do any
dereferencing and thus don't need to move any part of `x`. If the contents of
the Box had move semantics, then we could not write `&Var1(box y) => {}`, we
would be forced to use the reference version. We could also use similar
techniques with the first approach to matching, which look the same but without
the first `&`. For example, `Var1(box ref y) => {}`.

Now lets get more complex. Lets say you want to match against a pair of
reference-to-enum values. Now we can't use the first approach at all:

```rust
fn bar(x: &Enum2, y: &Enum2) {
    // Error: x and y are being moved.
    // match (*x, *y) {
    //     (Var2, _) => {}
    //     _ => {}
    // }

    // OK.
    match (x, y) {
        (&Var2, _) => {}
        _ => {}
    }
}
```

The first approach is illegal because the value being matched is created by
dereferencing `x` and `y` and then moving them both into a new tuple object. So
in this circumstance, only the second approach works. And of course, you still
have to follow the rules above for avoiding moving parts of `x` and `y`.

If you do end up only being able to get a reference to some data and you need
the value itself, you have no option except to copy that data. Usually that
means using `clone()`. If the data doesn't implement clone, you're going to have
to further destructure to make a manual copy or implement clone yourself.

What if we don't have a reference to a value with move semantics, but the value
itself. Now moves are OK, because we know no one else has a reference to the
value (the compiler ensures that if they do, we can't use the value). For
example,

```rust
fn baz(x: Enum2) {
    match x {
        Var1(y) => {}
        _ => {}
    }
}
```

There are still a few things to be aware of. Firstly, you can only move to one
place. In the above example we are moving part of `x` into `y` and we'll forget
about the rest. If we wrote `a @ Var1(y) => {}` we would be attempting to move
all of `x` into `a` and part of `x` into `y`. That is not allowed, an arm like
that is illegal. Making one of `a` or `y` a reference (using `ref a`, etc.) is
not an option either, then we'd have the problem described above where we move
whilst holding a reference. We can make both `a` and `y` references and then
we're OK - neither is moving, so `x` remains intact and we have pointers to the
whole and a part of it.

Similarly (and more common), if we have a variant with multiple pieces of nested
data, we can't take a reference to one datum and move another. For example if we
had a `Var4` declared as `Var4(Box<int>, Box<int>)` we can have a match arm
which references both (`Var4(ref y, ref z) => {}`) or a match arm which moves
both (`Var4(y, z) => {}`) but you cannot have a match arm which moves one and
references the other (`Var4(ref y, z) => {}`). This is because a partial move
still destroys the whole object, so the reference would be invalid.
