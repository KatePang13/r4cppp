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

现在，很重要的一点是 **borrowed 对象 不能被move，否则你会持有一个指向 无效对象的 引用**，等价于你持有一个引用，这个引用指向的对象在离开scope后被销毁——这就是一种悬空指针。如果你有一个指针，指向一个对象，这个对象可能有其他的引用。因此如果这个对象有删除语义，你有一个指向它的指针，解引用这个指针式不安全的。（如果对象有拷贝语义，解引用会创建一个新的对象，旧对象仍然存在，这样其他引用还能正常工作）。

OK, back to match expressions. As I said earlier, if you want to match some `x`
with type `&T` you can dereference once in the match clause or match the
reference in every arm of the match expression. Example:

OK，现在回到 match表达式。就像我之前说的，如果你想要用类型 `&T` 来 match  对象 `x`，你可以在 match clause 中 做一次 解引用  `*x`，或者在每个分支取各个候选值的引用。示例：

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

在这个例子中，2种方法都是可行的，因为  `Enum1` 有复制语义。让我们更细致地来观察这2个方法：

方法一 ：

- 我们解引用  x 并赋值给一个`Enum1`类型的临时变量。

- 这是一个 一级 match，因为我们没有深入到 字段的类型。

方法二：

- 没有解引用
- 我们将＆Enum1类型的值与对每个变量的引用进行匹配。
- 这是一个  二级  match,  它 match类型，并进入类型内部，查找以匹配所引用的类型（即Enum1  ）

Either way, we must ensure that we (that is, the compiler) respect 
Rust's invariants around moves and references - we must not move any
part of an object if it is referenced. If the value being matched has copy
semantics, that is trivial. If it has move semantics then we must make sure that
moves don't happen in any match arm. This is accomplished either by ignoring
data which would move, or making references to it (so we get by-reference
passing rather than by-move).

不管是哪种方法，我们都必须确保遵循 Rust中拷贝和引用的不可变性：

- 一个对象一旦被引用，对象的任意部分都不能被移除
  - 如果被 match 的值有 copy语义，这一点就无关紧要
  - 如果这个值 有 move语义，则我们必须确保 在任意的match分支中 ，move 不会发生。
    - 这可以通过忽略将要move的数据或对其进行引用来实现（因此，我们通过采用引用传递而不是move传递）。



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

上面的两种方法，我们都没有引用任何嵌套数据，所以没有数据会被 move。

方法一

即使传递的 x 是一个引用，我们是解引用再去match的，在解引用的scope内（比如示例中的match表达式）不会涉及到引用的语义要求。所以没有东西会 escape

方法二

我们没有绑定整个值（比如，把 `*x` 绑定给变量 ），所以我们没有 move 整个 对象。

We can take a reference to any variant in the second match, but not in the
dereferenced version. So, in the second approach replacing the second arm with `a
@ &Var2 => {}` is OK (`a` is a reference), but under the first approach we
couldn't write `a @ Var2 => {}` since that would mean moving `*x` into `a`. We
could write `ref a @ Var2 => {}` (in which `a` is also a reference), although
it's not a construct you see very often.

我们可以在第二个match获取任一variant的引用，而不是解引用之后的variant。所以方法二中，将第二个分支改写成 `a @ &Var2 =>{}` 也是可以的(a 是一个引用)，

但是方法一中，我们不能写成 `a @ Var2 => {}` 因为这很可能意味着  move  `*x` 到 a，导致 旧的对象销毁。我们可以写成 `ref a @ Var2 => {} ` （这里 `a` 也是一个引用），虽然这不是一种常见的解法。

But what about if we want to use the data nested inside `Var1`? We can't write:

但是，如果我们要使用 `Var1` 中嵌套的数据怎么办？我们不能写：

```rust
match *x {
    Var1(y) => {}
    _ => {}
}
```

或者

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

因为这2种写法意味着  将 x 的一部分 move 给 y。我们可以使用 `ref`关键字来获取 `Var1`中数据的引用，写成`Var1： &Var1(ref y) => {}` 。这也是可以的，因为这样我们没有对任何地方解引用，因为我们没有 move x的任何部分，而是创建了一个指向 x 内部的指针。

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

另外，我们可以解构 `Box` (这样 match 变成了3层)：`&Var1(box y) => {}` 。这是可行的，因为 i32 有 copy语义，y是 var1内的Box中的 `i32`的一个 拷贝 (var1 在 一个 borrowed 引用内)。因为 `i32` 有 copy 语义，我们不会 move x 的任意部分。

我们也可以创建一个 `int`的引用 而不是 copy 它： `&var1(box ref y) => {}` ，这也是可行的，因为我们没有做任何的解引用，所以没有move x 的任意部分。

如果  Box 中的成员有 move 语义，我们就不能写成  `&Var1(box y) => {}` ，就必须使用引用的版本。看起来很像，但是没有开头的`&` 。比如写成  `Var1(box ref y)=> {}`

Now lets get more complex. Lets say you want to match against a pair of
reference-to-enum values. Now we can't use the first approach at all:

现在看些更复杂的例子，假设你想要 对 一个  enum 引用对  做 match，我们知道不能使用第一种方法： 

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

方法一是非法的，因为 被 match 的值 通过  解引用  x，y 来创建，然后将它们分别 move 到 新的 tuple 对象。在这种情况下，只有方法二可行，当然，你仍然必须遵循 **避免移除 x和y的一部分** 的规则。

If you do end up only being able to get a reference to some data and you need
the value itself, you have no option except to copy that data. Usually that
means using `clone()`. If the data doesn't implement clone, you're going to have
to further destructure to make a manual copy or implement clone yourself.

如果你终归 必须同时获取 指向某数据的引用和数据本身，只能选择 copy 这个数据，通常 是使用 `clone`。如果这个数据没有实现 `clone` , 你必须在 进一步 解构 数据并做手动的copy；或者自己实现 `clone` 。

What if we don't have a reference to a value with move semantics, but the value
itself. Now moves are OK, because we know no one else has a reference to the
value (the compiler ensures that if they do, we can't use the value). For
example,

如果一个数据没有 move语义，而我们只有数据本身，没有它的引用，该怎么办。这种情况 move 是OK的，因为我们知道 没有人拥有 这个数据的引用（编译器确保如果有引用，我们没法使用这个数据）。示例：

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

这里还有几件事要注意：

- **你只能将 对象 move 到一个地方**
  - 上面的例子中，我们将  x 的部分 move 到 y，忘了 x 的其他部分。
  - 如果我们写成  `a @Var1(y) => {}` 我们会尝试 move 整个 x 到 a ，move 部分 x 到 y。这是不允许的，这样的分支是非法的。
  - 让 a 或者 y 的其中一个 为 引用也是不行的，那么我们就会遇到上面说过的问题：在持有引用的同时做move 操作
  - 我们可以让 a ，y 都为引用，这是可行的。没有数据会被 move，所以 x 保持 原封不动，我们也拥有了 指向 x  整体 和 部分 的指针。

Similarly (and more common), if we have a variant with multiple pieces of nested
data, we can't take a reference to one datum and move another. For example if we
had a `Var4` declared as `Var4(Box<int>, Box<int>)` we can have a match arm
which references both (`Var4(ref y, ref z) => {}`) or a match arm which moves
both (`Var4(y, z) => {}`) but you cannot have a match arm which moves one and
references the other (`Var4(ref y, z) => {}`). This is because a partial move
still destroys the whole object, so the reference would be invalid.

同样的道理（也是更常见的情况），**如果我们有一个包含多个嵌套数据 variant， 我们 在使用方法上必须统一，不能一些用 引用的方式，一些用 move 的方式**。比如，你有一个声明为 `Var4(Box<int>, Box<int>)` 的 `Var4`， 可以在一个 分支里 

- 都使用 引用  ( `Var4(ref y , ref z) => {}` )  
- 或者都使用 move  ( `Var4(y, z) =>{} ` )
- 但是不可以 一个用 move 一个用引用  ( `Var4(ref y, z) => {}` )
  - 因为 对象的部分move 同样会销毁整个对象，所以这个的引用是非法的