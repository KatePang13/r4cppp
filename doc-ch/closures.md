# Closures and first-class functions

Closures and first-class and higher order functions are a core part of Rust. In
C and C++ there are function pointers (and those weird member/method pointer
things in C++ that I never got the hang of). However, they are used relatively
rarely and are not very ergonomic. C++11 introduced lambdas, and these are
pretty close to Rust closures, in particular they have a very similar
implementation strategy.

必包  和 高阶函数 是 Rust的一个核心部分。在C/C++中，有函数指针，然而它们使用场景很少，且很不符合任何思维。C++11 引入了 lambda 表达式，这个和 Rust的闭包更接近，特别是它们具有非常相似的实现策略。

To start with, I want to establish some intuition for these things. Then, we'll
dive in to the details.

首先，我想为这些东西建立一些直觉的印象。然后再将深入研究细节。

Lets say we have a function `foo`: `pub fn foo() -> u32 { 42 }`. Now let's
imagine another function `bar` which takes a function as an argument (I'll leave
`bar`'s signature for later): `fn bar(f: ...) { ... }`. We can pass `foo` to
`bar` kind of like we would pass a function pointer in C: `bar(foo)`. In the
body of `bar` we can call `f` as if it were a function: `let x = f();`.

假设我们有一个函数 `foo` : `pub fn foo() -> u32 {42}` 。想象有另一个函数 `bar` ，接收一个函数作为参数（我会给出bar的函数声明，供后续使用）： `fn bar(f: ...) {...}` 。 我们可以把 `foo`传递给 `bar`，就像 C语言中，传递函数指针作为参数。再`bar`的函数体中，我可以调用 `f` ，就像它是一个函数 ： `let x = f();`

We say that Rust has first-class functions because we can pass them around and
use them like we can with other values. We say `bar` is a higher-order function
because it takes a function as an argument, i.e., it is a function that operates
on functions.

我们说 Rust 有 first-class 函数，因为我们可以传递它们，并像其他值一样使用它们。我们说 `bar` 是 一个 告诫函数，因为它接收函数作为参数，也就是说，它是对函数做操作的函数。

Closures in Rust are anonymous functions with a nice syntax. A closure `|x| x +
2` takes an argument and returns it with `2` added. Note that we don't have to
give types for the arguments to a closure (they can usually be inferred). We
also don't need to specify a return type. If we want the closure body to be more
than just one expression, we can use braces: `|x: i32| { let y = x + 2; y }`. We
can pass closures just like functions: `bar(|| 42)`.

Rust中的闭包是具有良好语法的匿名函数。一个闭包 `|x| x + 2` 接收一个参数x，并返回 x+2。注意，不需要我们必须指定闭包的参数类型(其类型通常可以推断)。我们也不需要指定 返回类型。如果我们 希望闭包体更像函数，而不是一个简单的表达式，我们可以写成  `|x: i32| {let y = x+2; y}` 。我们可以像函数一样传递闭包：`bar(|| 42)`

The big difference between closures and other functions is that closures capture
their environment. This means that we can refer to variables outside the closure
from the closure. E.g.,

闭包与其他函数的最大的差别是，闭包快照了它所在的外部环境。这意味着我们可以访问 闭包外面的函数。

```rust
let x = 42;
bar(|| x);
```

Note how `x` is in scope in the closure.

We've seen closures before, used with iterators, and this is a common use case
for them. E.g., to add a value to each element of a vector:

注意  x 在 闭包中的scope。

我们之前已经见过闭包，它与迭代器一起使用，这是它们的常见用例。例如，向向量的每个元素添加值：

```rust
fn baz(v: Vec<i32>) -> Vec<i32> {
    let z = 3;
    v.iter().map(|x| x + z).collect()
}
```

Here `x` is an argument to the closure, each member of `v` will be passed as an
`x`. `z` is declared outside of the closure, but because it's a closure, `z` can
be referred to. We could also pass a function to map:

这里有一个 closure 的参数 x，每个 v 的元素 都会 以一个 `x` 传递给闭包。`z`在闭包外定义，但因为这是个闭包，z仍然可以范文。我们还可以传递一个函数来用于map：

```rust
fn add_two(x: i32) -> i32 {
    x + 2
}

fn baz(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(add_two).collect()
}
```

Note that Rust also allows declaring functions inside of functions. These are
*not* closures - they can't access their environment. They are merely a
convenience for scoping.

需要注意的是，Rust 也允许 在函数内定义函数。这样的函数不是闭包，它们不能访问它们的上下文环境，函数内定义函数只是为了方便地划分子scope。

```rust
fn qux(x: i32) {
    fn quxx() -> i32 {
        x // ERROR x is not in scope.
    }

    let a = quxx();
}
```

## Function types

Lets introduce a new example function:

接下来介绍  一个新的示例函数：

```rust
fn add_42(x: i32) -> i64 {
    x as i64 + 42
}
```

As we saw before, we can store a function in a variable: `let a = add_42;`. The
most precise type of `a` cannot be written in Rust. You'll sometimes see the
compiler render it as `fn(i32) -> i64 {add_42}` in error messages. Each function
has its own unique and anonymous type. `fn add_41(x: i32) -> i64` has a different
type, even though it has the same signature.

如你所知，我们可以将函数保存成一个变量：`let a = add_42` 。在Rust，`a` 基本不能写出它的具体的类型。我们后续会看到编译器对 `fn(i32) -> i64{add_42}` 报错。每个函数有它自己的独占的且匿名的类型。`fn add_41(x: i32) -> i64` 就是另外一个类型，即使二者有相同的签名。

We can write less precise types, for example, `let a: fn(i32) -> i64 = add_42;`.
All function types with the same signature can be coerced to a `fn` type
(which can be written by the programmer).

我们可以写成 不那么具体的类型，比如： `let a : fn(i32) -> i64 = add_42`;  。具有相同签名的函数类型都可以强制转换为`fn`类型（可以由程序员编写转换语句）。

`a` is represented by the compiler as a function pointer, however, if the
compiler knows the precise type, it doesn't actually use that function pointer.
A call like a() is statically dispatched based on the type of a. If the
compiler doesn't know the precise type (e.g., it only knows the fn type), then
the call is dispatched using the function pointer in the value.

`a` 由编译器表示成一个函数指针，然而，如果编译器知道确切的类型，则它实际上并不适用这个函数指着。

- 编译器知道确切类型，`a()`函数调用， 是根据 a的类型静态dispatch的；
- 如果编译器不知道确切的类型（比如，只知道是 fn 类型），则函数调用使用 函数指针来 dispatch 。

There are also `Fn` types (note the capital 'F'). These `Fn` types are bounds,
just like traits (in fact they *are* traits, as we'll see later). `Fn(i32) -> i64`
is a bound on the types of all function-like objects with that signature. When
we take a reference to a function pointer, we're actually creating a trait
object which is represented by a fat pointer (see DSTs).

Rust还有一种类型叫 `Fn`。`Fn`类型 与 `trait` 一样 ，代表一个集合(bound) 。(事实上，`Fn` 就是 `trait`)。`Fn(i32 -> i64)` 是具有该签名 的所有 function-like 对象的类型的集合。当我们获取一个指针函数的引用时，我们本质上是创建了一个 trait 对象，由一个 pat ptr 表示(就像  DSTs)。

To pass a function to another function, or to store the function in a field, we
must write a type. We have several choices, we can either use either a `fn` type
or a `Fn` type. The latter is better because it includes closures (and
potentially other function-like things), whereas `fn` types don't. The `Fn`
types are dynamically sized which means we cannot use them as value types. We
must either pass function objects or use generics. Let's look at the generic
approach first. For example,

要 将一个函数作为参数传递给另一个参数，或者将函数存储在一个字段中，我们必须指定一个类型。有多种选择，可以使用一个 `fn`类型或者 一个`Fn`类型。后者更好，因为它可以包含 闭包(和其他的 function-like对象)。而 `fn` 只能接收函数。`Fn`类型是动态尺寸，意味着我们不能将它们用作值类型。我们必须 传递 一个 函数对象或者使用 泛型。我们先来看看泛型的方式。示例：

```rust
fn bar<F>(f: F) -> i64
    where F: Fn(i32) -> i64
{
    f(0)
}
```

`bar` takes any function with the signature `Fn(i32) -> i64`, i.e., we can
instantiate the `F` type parameter with any function-like type. We could call
`bar(add_42)` to pass `add_42` to `bar` which would instantiate `F` with
`add_42`'s anonymous type. We could also call `bar(add_41)` and that would work
too.

`bar` 接收任何签名为 `Fn(i32) -> i64` 的函数等，我们可以用任何的 类函数对象来实例化 `F`类型的参数。我们可以调用 `bar(add_42)` 来传递 `add_42` 给 `bar`，这时  `F` 被实例化成 `add_42`匿名类型。我们也可以调用 `bar(add_41)`，道理都是一样的。 

You can also pass closures to `bar`, e.g., `bar(|x| x as i64)`. This works
because closure types are also bounded by the `Fn` bound matching their
signature (like functions, each closure has it's own anonymous type).

你也可以传递闭包给 `bar`，比如 `bar(|x| x as i64)` 。这是可行的，因为 闭包类型也属于 匹配这签名的 `Fn`范畴（和函数一样，每个闭包都有自己独有的匿名类型）。

Finally, you can pass references to functions or closures too: `bar(&add_42)` or
`bar(&|x| x as i64)`.

其实，你也可以传递 函数或闭包的引用 给 bar : `bar(&add_42)` 或者 `bar(&|x| x as i64)`。

One could also write `bar` as `fn bar(f: &Fn(i32) -> i64) ...`. These two
approaches (generics vs a function/trait object) have quite different semantics.
In the generics case, `bar` will be monomorphised so when code is generated, the
compiler know the exact type of `f`, that means it can be statically dispatched.
If using a function object, the function is not monomorphised. The exact type of
`f` is not known, and so the compiler must generate a virtual dispatch. The
latter is slower, but the former will produce more code (one monomorphised
function per type parameter instance).

另外，你也可以将 bar函数写成 `fn bar(f: &Fn(i32) -> i64)...` 。这两者方法（泛型 VS  function/trait 对象）有相当不一样的语法。

- 对于泛型方法，当代码生成之后，bar 是 单态的，编译器知道 `f` 的确切类型，这意味着可以静态分发。
- 如果使用函数对象，则函数是多态的。不知道 `f`的确切类型，编译器会生成一个虚拟分发(virtual dispatch)。后者更慢，单前者会产生过多的代码（每个类型参数实例一个单态函数）。

There are actually more function traits than just `Fn`; there are `FnMut` and
`FnOnce` too. These are used in the same way as `Fn`, e.g., `FnOnce(i32) ->
i64`. A `FnMut` represents an object which can be called and can be mutated
during that call. This doesn't apply to normal functions, but for closures it
means the closure can mutate its environment. `FnOnce` is a function which can
only be called (at most) once. Again, this is only relevant for closures.

除了`Fn`，还有其他类型的函数trait，比如 `FnMut` 和 `FnOnce` 。它们的用法和`Fn`是类型的。比如，`FnOnece(i32)->i64` 。

- 一个 `FnMut` 代表一个可以被调用对象，可以在调用时进行变更
  - 不能用于普通函数，可以用于闭包，这时意味着 这个闭包可以 变更它的上下文环境。
- `FnOnce` 表示最多只能被调用一次
  - 也仅适用于闭包

`Fn`, `FnMut`, and `FnOnce` are in a sub-trait hierarchy. `Fn`s are `FnMut`s
(because one can call a `Fn` function with permission to mutate and no harm is
done, but the opposite is not true). `Fn`s and `FnMut`s are `FnOnce`s (because
there is no harm done if a regular function is only called once, but not the
opposite).

`Fn` , `FnMut` , `FnOnce` 是 子特征层次结构。`Fn` 也 `FnMut` (因为调用具有变更权限的`Fn` 函数，不会有什么问题，反之就不成立了)。`Fn` 和 `FnMut` 也是 `FnOnce` （因为一个常规的函数只调用次不会有什么问题，反之就有问题了）。

So, to make a higher-order function as flexible as possible, you should use the
`FnOnce` bound, rather than the `Fn` bound (or use the `FnMut` bound if you must
call the function more than once).

所以，为了 让 一个 高阶函数 尽可能灵活，你应该使用 `FnOnce`范畴，而不是`Fn`范畴(如果你必须多次调用，则使用 `FnMut`)。【啥意思...】


### Methods

You can use methods in the same way as functions - take pointers to them store
them in variables, etc. You can't use the dot syntax, you must explicitly name
the method using the fully explicit form of naming (sometimes called UFCS for
universal function call syntax). The `self` parameter is the first argument to
the method. E.g.,

你可以像 函数(function)一样使用方法(method)：比如，将它的指针存放在一个变量上。但是，不可以使用`.`语法，你必须完全显式地命名方这个方法（有时将 通用函数调用语法称为 UFCS）。 `self` 参数是方法的第一个参数。示例：

```rust
struct Foo;

impl Foo {
    fn bar(&self) {}
}

trait T {
    fn baz(&self);
}

impl T for Foo {
    fn baz(&self) {}
}

fn main() {
    // Inherent method.
    let x = Foo::bar;
    x(&Foo);
    
    // Trait method, note the fully explicit naming form.
    let y = <Foo as T>::baz;
    y(&Foo);
}
```


### Generic functions

You can't take a pointer to a generic function and there is no way to express a
generic function type. However, you can take a reference to a function if all
its type parameters are instantiated. E.g.,

你不能拥有一个 指向 泛型函数的指针，也没有方法来表达一个泛型函数类型。但是，你可以获取一个指针函数的引用，如果它的所有类型参数都已经实例化了。示例：

```rust
fn foo<T>(x: &T) {}

fn main() {
    let x = &foo::<i32>;
    x(&42);
}
```

There is no way to define a generic closure. If you need a closure to work over
many types you can use trait objects, macros (for generating closures), or pass
a closure which returns closures (each returned closure can operate on a
different type).

也没有方法能定义一个泛型闭包。如果你需要一个能覆盖很多类型的闭包，你可以使用 trait对象，宏（用于生成闭包），或者传递一个返回闭包（每个返回的闭包可以各自作用于一个不同的类型）的闭包。


### Lifetime-generic functions and higher-ranked types

It *is* possible to have function types and closures which are generic over
lifetimes. 

Imagine we have a closure which takes a borrowed reference. The closure can work
the same way no matter what lifetime the reference has (and indeed in the
compiled code, the lifetime will have been erased). But, what does the type look
like?

For example,

**在整个生命期内通用**的 函数类型 和 闭包类型 是有办法做到的。

想象以下，我们有一个 接收 `borrowed ref` 的闭包。这个闭包可以以同样的方式执行，无论这个 ref 有什么样的生命期（而且事实上在编译好的代码里，声明期将会被删除）。但是，这样的类型长什么样呢。

举个例子：

```rust
fn foo<F>(x: &Bar, f: F) -> &Baz
    where F: Fn(&Bar) -> &Baz
{
    f(x)
}
```

what are the lifetimes of the references here? In this simple example, we can
use a single lifetime (no need for a generic closure):

这里的 ref 的生命期是什么样的呢。在这个简单示例中，我们可以使用 单独的生命期（对于泛型闭包来说不需要）：

```rust
fn foo<'b, F>(x: &'b Bar, f: F) -> &'b Baz
    where F: Fn(&'b Bar) -> &'b Baz
{
    f(x)
}
```

But what if we want `f` to work on inputs with different lifetimes? Then we need
a generic function type:

但是，如果我们希望  `f` 在 各个输入拥有不同的声明期的情况下工作呢？ 这时我们需要一个 泛型函数类型

```rust
fn foo<'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: for<'a> Fn(&'a Bar) -> &'a Baz
{
    (f(x), f(y))
}
```

The novelty here is the `for<'a>` syntax, this is used to denote a function type
which is generic over a lifetime. It is read "for all 'a, ...". In theoretical
terms, the function type is universally quantified.

这里比较新颖的地方在于  `for<'a'>` ，这是用来将一个函数类型标记为在一个声明期内通用。这个代码读作 "for all 'a, ... " (对于任何的声明周期 'a, ...) 。理论上 这个函数类型是 普遍量化的。

Note that we cannot hoist up `'a` to `foo` in the above example. Counter-example:

注意，上面的例子中，我们不能将 `a` 替换成 `foo`。 反例：

```rust
fn foo<'a, 'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: Fn(&'a Bar) -> &'a Baz
{
    (f(x), f(y))
}
```

will not compile because when the compiler infers lifetimes for a call to `foo`,
it must pick a single lifetime for `'a`, which it can't do if `'b` and `'c` are
different.

这是无法编译的，因为当编译器在一个调用中推断 `foo`的生命期时，必须为 `'a` 选择一个生命期，如果 `'b` 和 `'c` 不一致，就无法选择。

A function type which is generic in this way is called a higher-ranked type.
Lifetime variables at the outer level have rank one. Because `'a` in the above
example cannot be moved to the outer level, it's rank is higher than one.

这种方式实现泛型的函数，叫做 higher-ranked type 更高阶类型。外层变量的生命期级别是rank-1。因为 上面的例子中，`'a` 不能被 move到外层，所以它是高于 rank-1 等。

Calling functions with higher-ranked function type arguments is easy - the
compiler will infer the lifetime parameters. E.g., `foo(&Bar { ... }, &Bar
{...}, |b| &b.field)`.

使用  **higher-ranked 函数类型参数**的 函数调用是很简单的——编译器将推导它的生命期参数。比如 `foo(&Bar {...} &Bar{...}, |b| &b.field )` 。

In fact, most of the time you don't even need to worry about such things. The
compiler will allow you to elide the quantified lifetimes in the same way that
you are allowed to elide many lifetimes on function arguments. For example, the
example above can just be written as

事实上，多数时候，你都不需要担心这些事情。编译器将允许你像函数参数省略生命期的方式一样，省略 量化生命期。上面的例子可以简化成：

```rust
fn foo<'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: Fn(&Bar) -> &Baz
{
    (f(x), f(y))
}
```

(and you only need `'b` and `'c` because it is a contrived example).

(而且你只需要 `'b` `'c` ，因为这只是一个故意为之的示例)

Where Rust sees a function type with a borrowed references, it will apply the
usual elision rules, and quantify the elided variables at the scope of the
function type (i.e., with higher rank).

在Rust中看到带有 `borrowed ref` 的函数类型，会应用通用的省略规则，并在函数类型的scope内量化被省略的变量。

You might be wondering why bother with all this complexity for what looks like a
fairly niche use case. The real motivation is functions which take a function
to operate on some data provided by the outer function. For example,

你或许会疑惑，为什么对于非常简单的例子，要引入这么多复杂的东西。这样做的真正动机，是要**使得函数可以对外部函数提供的数据进行操作**。示例：

```rust
fn foo<F>(f: F)
    where F: Fn(&i32) // Fully explicit type: for<'a> Fn(&'a i32)
{
    let data = 42;
    f(&data)
}
```

In these cases, we *need* higher-ranked types. If we added a lifetime parameter
to `foo` instead, we could never infer a correct lifetime. To see why, let's
look at how it might work, consider `fn foo<'a, F: Fn(&'a i32')> ...`. Rust
requires that any lifetime parameter must outlive the item it is declared on (if
this were not the case, an argument with that lifetime could be used inside that
function, where it is not guaranteed to be live). In the body of `foo` we use
`f(&data)`, the lifetime Rust will infer for that reference will last (at most)
from where `data` is declared to where it goes out of scope. Since `'a` must
outlive `foo`, but that inferred lifetime does not, we cannot call `f` in this
way.

However, with higher-ranked lifetimes `f` can accept any lifetime and so the
anonymous one from `&data` is fine and the function type checks.


### Enum constructors

This is something of a digression, but it is sometimes a useful trick. All
variants of an enum define a function from the fields of the variant to the enum
type. For example,

这个内容有点离题，但是个很好用的技巧，一个 enum 的 各个variant 定义了一个 从 variant字段 到 enum 类型的函数映射。示例：

```rust
enum Foo {
    Bar,
    Baz(i32),
}
```

defines two functions, `Foo::Bar: Fn() -> Foo` and `Foo::Baz: Fn(i32) -> Foo`.
We don't normally use the variants in this way, we treat them as data types
rather than functions. But sometimes it is useful, for example if we have a list
of `i32`s we can create a list of `Foo`s with

定义2个函数 `Foo:Bar: Fn() -> Foo` 和 `Foo:Baz: Fn(i32) -> Foo` 。这里我们不是正常的使用这个 variants，我们将它们当作数据类型而不是函数。这在有些时候很有用，比如，你有一个 `i32`的列表，你可以这样创建一个 Foo的列表：

```rust
list_of_i32.iter().map(Foo::Baz).collect()
```


## Closure flavours

A closure has two forms of input: the arguments which are passed to it explicitly
and the variables it *captures* from its environment. Usually, everything about
both kinds of input is inferred, but you can have more control if you want it.

一个闭包有两方面的输入，显示传递的参数和从上下文中捕捉到的变量。通常，关于这两类输入的一切信息都是推导而来的，但是如果愿意，你可以对此做很多的控制。

For the arguments, you can declare types instead of letting Rust infer them. You
can also declare a return type. Rather than writing `|x| { ... }` you can write
`|x: i32| -> String { ... }`. Whether an argument is owned or borrowed is 
determined by the types (either declared or inferred).

对参数来说，你可以 定义类型，而不是靠编译器推导。你也可以定义返回类型。你可以不写成`|x| {...}` 而是 `|x: i32| -> String {...}` 。 不管一个参数是 `owned` 还是 `borrowed` ，都是由类型决定的（不管类型是声明的还是推导的）。

For the captured variables, the type is mostly known from the environment, but
Rust does a little extra magic. Should a variable be captured by reference or
value? Rust infers this from the body of the closure. If possible, Rust captures
by reference. E.g.,

对于捕捉的变量，类型通常在上下文是已知的，但是Rust准备了额外的魔术。一个变量应该以 引用的形式，来世以值的形式被捕捉？Rust 通过 闭包体来推导。条件允许的情况下，Rust以引用的形式捕捉变量，比如：

```rust
fn foo(x: Bar) {
    let f = || { ... x ... };
}
```

All being well, in the body of `f`, `x` has the type `&Bar` with a lifetime
bounded by the scope of `foo`. However, if `x` is mutated, then Rust will infer
that the capture is by mutable reference, i.e., `x` has type `&mut Bar`. If `x`
is moved in `f` (e.g., is stored into a variable or field with value type), then
Rust infers that the variable must be captured by value, i.e., it has the type
`Bar`.

看起来很不错，在 `f` 函数体内， `x` 是 `&Bar`类型，生命期在 `foo` scope 内。但是，如果 `x` 变更了，Rust 将会把这个捕捉推导为 `mutable ref`形式， 比如 `&mut Bar` 。如果  x 在 `f`内被move（比如被存到一个变量或者以值形式存到一个filed），则Rust 推导 这个变量 必须 以 值形式 捕捉，类型为 `Bar`。

This can be overridden by the programmer (sometimes necessary if the closure
will be stored in a field or returned from a function). By using the `move`
keyword in front of a closure. Then, all of the captured variables are captured
by value. E.g., in `let f = move || { ... x ... };`, `x` would always have type
`Bar`.

类型推导可以被程序员重写（有时候是需要的，日光闭包会被存到一个field或者从一个函数返回）。在 闭包的开头使用`move` 关键字，则所有的变量捕捉都使用值形式，保留 `let f = move || {... x ...};`  x 将总是 `Bar`类型 。

We talked earlier about the different function kinds: `Fn`, `FnMut`, and `FnOnce`.
We can now explain why we need them. For closures, the mutable-ness and once-ness
refer to the captured variables. If a capture mutates any of the variables it
captures then it will have a `FnMut` type (note that this is completely inferred
by the compiler, no annotation is necessary). If a variable is moved into a
closure, i.e., it is captured by value (either because of an explicit `move` or
due to inference), then the closure will have a `FnOnce` type. It would be unsafe
to call such a closure multiple times because the captured variable would be
moved more than once.

之前我们讨论过不同的函数种类：`Fn`, `FnMut`, `FnOnce` ，现在我们可以解释为什么需要这些种类的。对于闭包来说， mutable-ness 和 once-ness 只的是 捕捉的变量。如果一个捕捉变更了所有捕捉的变量，则必须是 `FnMut`类型 （注意，这是完全由编译器推断的，不需要标记）。如果一个变量被move进一个闭包，比如以值形式捕捉（不管是因为显式的`move` 会是推导而来的），则这个闭包必须是 `FnOnce` 类型。多次调用这样的闭包是不安全的，因为捕捉的变量可能被多次 move。

Rust will do its best to infer the most flexible type for the closure if it can.

Rust会在允许的条件下，尽可能推断出最灵活的封闭类型。


## Implementation

A closure is implemented as an anonymous struct. That struct has a field for
each variable captured by the closure. It is lifetime-parametric with a single
lifetime parameter which is a bound on the lifetime of captured variables. The
anonymous struct implements a `call` method which is called to execute the
closure.

闭包实现为一个匿名的结构体。这个结构体为每个捕捉到的变量准备一个字段。它有单一的生命期参数，该参数限制了捕捉变量的生命期。这个匿名结构体必须实现一个 `call`方法，调用`call` 以执行这个闭包。

For example, consider

比如，对于代码

```rust
fn main() {
    let x = Foo { ... };
    let f = |y| x.get_number() + y;
    let z = f(42);
}
```

the compiler treats this as

编译器将其视为：

```rust
struct Closure14<'env> {
    x: &'env Foo,
}

// Not actually implemented like this, see below.
impl<'env> Closure14<'env> {
    fn call(&self, y: i32) -> i32 {
        self.x.get_number() + y
    }
}

fn main() {
    let x = Foo { ... };
    let f = Closure14 { x: x }
    let z = f.call(42);
}
```

As we mentioned above, there are three different function traits - `Fn`,
`FnMut`, and `FnOnce`. In reality the `call` method is required by these traits
rather than being in an inherent impl. `Fn` has a method `call` which takes
`self` by reference, `FnMut` has `call_mut` taking `self` by mutable reference,
and `FnOnce` has `call_once` which takes `self` by values.

正如我们之前提高的，这里由三个三种不同的 函数 traits - `Fn`,
`FnMut`, and `FnOnce`。 事实上，这些 trait需要的是 这个 `call` 方法，而不是固有的隐含特性。

- `Fn` 的 `call` 以ref 形式 接受 `self` ;
-  `FnMut` 的 `call_mut` 以 mutable ref 形式接受 `self` ; 
- 而`FnOnce` 以 值形式接受`self`

When we've seen function types above, they look like `Fn(i32) -> i32` which
doesn't look much like a trait type. There is a little bit of magic here. Rust allows
this round bracket sugar only for function types. To desugar to a regular type
(an 'angle bracket type'), the argument types are treated as a tuple type and
passed as a type parameter and the return type as an associated type called
`Output`. So, `Fn(i32) -> i32` is desugared to `Fn<(i32,), Output=i32>` and the
`Fn` trait definition looks like

我们已经看过了上面的函数类型，我们看起来像 `Fn(i32) -> i32` ，这不是很像一个 trait类型。这里有一点点小魔术。Rust 只允许 函数类型使用这种 `()` 语法糖。

为了将其转换为常规类型（`<>`类型），需要将参数类型视为元组类型，并使用类型参数传递，将返回类型作为关联类型(称为输出)传输。`Fn(i32) ->32` 被还原为 `Fn<(i32,) Output=i32>` ，则这个 `Fn` trait 定义 变成这样：

```rust
pub trait Fn<Args> : FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

The implementation for `Closure14` above would therefore look more like

`Closure14`的实现因此变成这样：

```rust
impl<'env> FnOnce<(i32,)> for Closure14<'env> {
    type Output = i32;
    fn call_once(self, args: (i32,)) -> i32 {
        ...
    }
}
impl<'env> FnMut<(i32,)> for Closure14<'env> {
    fn call_mut(&mut self, args: (i32,)) -> i32 {
        ...
    }
}
impl<'env> Fn<(i32,)> for Closure14<'env> {
    fn call(&self, args: (i32,)) -> i32 {
        ...
    }
}
```

You can find the function traits in [core::ops](https://dxr.mozilla.org/rust/source/src/libcore/ops.rs)

你可以在代码中找到这个函数trait   [core::ops](https://dxr.mozilla.org/rust/source/src/libcore/ops.rs)

We talked above about how using generics gives static dispatch and using trait
objects gives virtual dispatch. We can now see in a bit more detail why.

我们前面讨论了如何使用泛型来进行静态分发，和使用trait对象来进行虚拟转发。现在，我们可以更详细地了解原因。

When we call `call`, it is a statically dispatched method call, there is no
virtual dispatch. If we pass it to a monomorphised function, we still know the
type statically, and we still get a static dispatch.

当我们调用 `call` 时，这时一个静态分发的方法调用，不是动态分发。如果我们将其传递给一个单态函数，我们仍然确切地知道它的类型，依然时获得一个静态分发。

We can make the closure into a trait object, e.g., `&f` or `Box::new(f)` with
types `&Fn(i32)->i32` or `Box<Fn(i32)->i32>`. These are pointer types, and
because they are pointer-to-trait types, the pointers are fat pointers. That
means they consist of the pointer to the data itself and a pointer to a vtable.
The vtable is used to lookup the address of `call` (or `call_mut` or whatever).

我们可以将闭包变成 trait 对象。比如 `&f` 或者 `Box::new(f)` 变成 `&Fn(i32)->i32` 或者`Box<Fn(i32)->i32>` 。这些时指针类型，而且因为它们是指向trait的指针，所以是 fat ptr。这意味着它们包含了指向数据本身的指着和一个指向`vtable`的指针。`vtable` 用于查找 `call`的地址（或者`call_mut` 等)。

You'll sometimes hear these two representations of closures called boxed and
unboxed closures. An unboxed closure is the by-value version with static
dispatch. A boxed version is the trait object version with dynamic dispatch. In
the olden days, Rust only had boxed closures (and the system was quite a bit
different).

有时你会看到将这两类闭包称为 boxed 闭包 和 unboxed 闭包。unboxed 闭包是静态分发的传值版本；unboxed 闭包是动态分发的trait对象版本。在Rust的早期版本，只有boxed闭包（那时的boxed闭包系统也和现在有些不同）。

## References

* [RFC 114 - Closures](https://github.com/rust-lang/rfcs/blob/master/text/0114-closures.md)
* [Finding Closure in Rust blog post](http://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
* [RFC 387 - Higher ranked trait bounds](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md)
* [Purging proc blog post](http://smallcultfollowing.com/babysteps/blog/2014/11/26/purging-proc/)

FIXME: relate to closures in C++ 11
