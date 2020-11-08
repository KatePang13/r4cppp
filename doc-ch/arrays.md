# Arrays and Vectors

Rust arrays are pretty different from C arrays. For starters they come in
statically and dynamically sized flavours. These are more commonly known as
fixed length arrays and slices. As we'll see, the former is kind of a bad name
since both kinds of array have fixed (as opposed to growable) length. For a
growable 'array', Rust provides the `Vec` collection.

Rust的 array 和 C语言的 差别很大。【 For starters they come in
statically and dynamically sized flavours. 】 。对于数组有 定长和 不定长 2种需求，一般定长的叫数组，不定长的叫分片。前者的命名并不是太好，因为所有的数组都是定长的。对于 可增长的 array, Rust 提供了 `Vec` 内从。


## Fixed length arrays

The length of a fixed length array is known statically and features in its
type. E.g., `[i32; 4]` is the type of an array of `i32`s with length four.

Array literal and array access syntax is the same as C:

定长的 array 包含 长度 和 元素类型。比如 `[i32; 4]` 是一个 类型为 `i32` 的数组，长度为4。

数组的初始化和访问语法 和 C语言是一样的：

```rust
let a: [i32; 4] = [1, 2, 3, 4];     // As usual, the type annotation is optional.
println!("The second element is {}", a[1]);
```

You'll notice that array indexing is zero-based, just like C.

可以看到  array 的索引是 以0 为起点的，和C 一样。

However, unlike C/C++<sup>[1](#1)</sup>, array indexing is bounds checked. In
fact all access to arrays is bounds checked, which is another way Rust is a
safer language.

但是，和C/C++ 不一样的是， Rust 的 array 索引 是有边界检查的。Rust 的所有数组访问都有边界检查，这是Rust保证语言安全性的一个手段。

If you try to do `a[4]`, then you will get a runtime panic. Unfortunately, the
Rust compiler is not clever enough to give you a compile time error, even when
it is obvious (as in this example).

如果你想要 执行 `a[4]`， 你会得到一个 runtime panic。不幸的是，Rust 编译器 还是不够聪明，来对此产生编译时错误，即使是在越界很明显的时候（比如我们上面的示例）。

If you like to live dangerously, or just need to get every last ounce of
performance out of your program, you can still get unchecked access to arrays.
To do this, use the `get_unchecked` method on an array. Unchecked array accesses
must be inside an unsafe block. You should only need to do this in the rarest
circumstances.

如果你喜欢游走于刀尖，或者你想要追求更极致的性能，你也可以对数组做 无检查的访问，对数组使用 `get_unchecked` 方法。无检查访问代码必须放在一个 `unsafe block`。

Just like other data structures in Rust, arrays are immutable by default and
mutability is inherited. Mutation is also done via the indexing syntax:

就像  Rust 的其他 数据结构，array 默认是 不可变的，可变性是遗传的。对于可变的数组，索引访问语句也可以修改数组 ：

```rust
let mut a = [1, 2, 3, 4];
a[3] = 5;
println!("{:?}", a);
```

And just like other data, you can borrow an array by taking a reference to it:

和 其他数据一样，你可以通过获取数组的引用来 borrow 它。 

```rust
fn foo(a: &[i32; 4]) {
    println!("First: {}; last: {}", a[0], a[3]);
}

fn main() {
    foo(&[1, 2, 3, 4]);
}
```

Notice that indexing still works on a borrowed array.

注意：索引 对于  borrowed array 同样是可用的。 

This is a good time to talk about the most interesting aspect of Rust arrays for
C++ programmers - their representation. Rust arrays are value types: they are
allocated on the stack like other values and an array object is a sequence of
values, not a pointer to those values (as in C). So from our examples above, `let
a = [1_i32, 2, 3, 4];` will allocate 16 bytes on the stack and executing `let b
= a;` will copy 16 bytes. If you want a C-like array, you have to explicitly
make a pointer to the array, this will give you a pointer to the first element.

这块内容可以算是 C++程序员最好兴趣的内容之一。Rust array 是 值类型：他们和其他值一样 在栈内存种分类，一个array对象是一个 值序列，没有一个指针指向这些值(和C语言一样)。所有上面的例子 `let a = [1_i32, 2, 3, 4]; `会分配一个 16 byte 的栈空间，`let b = a` 会拷贝 16 byte。如果你想要一个 C风格的 array， 你必须显示的创建一个指向这个数组的指针，这样会得到一个指向第一个元素的指针。

A final point of difference between arrays in Rust and C++ is that Rust arrays
can implement traits, and thus have methods. To find the length of an array, for
example, you use `a.len()`.

Rust 和 C++ ，array的最后一个不同点是 Rust array 可以 实现  traits，也可以拥有方法。例如要过去一个 array 的长度，你可以使用 `a.len()` 。


## Slices

A slice in Rust is just an array whose length is not known at compile time. The
syntax of the type is just like a fixed length array, except there is no length:
e.g., `[i32]` is a slice of 32 bit integers (with no statically known length).

Rust 的 slice 其实就是 在编译期 长度位置的 array 。语法和array 很像，但是没有长度： `[i32]` 是一个 32 bit integer 的 slice。

There is a catch with slices: since the compiler must know the size of all
objects in Rust, and it can't know the size of a slice, then we can never have a
value with slice type. If you try and write `fn foo(x: [i32])`, for example, the
compiler will give you an error.

对于 slice 的一个直观理解是：Rust编译器必须知道所有对象的size，然而 不知道 slice的size，所以我们不可能得到一个 slice类型的值。如果你写成  `fn foo(x: [i32])` ，编译器会报错

So, you must always have pointers to slices (there are some very technical
exceptions to this rule so that you can implement your own smart pointers, but
you can safely ignore them for now). You must write `fn foo(x: &[i32])` (a
borrowed reference to a slice) or `fn foo(x: *mut [i32])` (a mutable raw pointer
to a slice), etc.

所以，你必须持有 指向 slice 的指针 (关于这个规则，有一些技巧上的特例，你可以实现你自己的只能指针，目前可以先忽略这种情况)。- 你必须写成

- `fn foo(x: &[i32])` (a borrowed reference to a slice) 
- 或者  `fn foo(x: *mut [i32])`  (a mutable raw pointer to a slice)  。

The simplest way to create a slice is by coercion. There are far fewer implicit
coercions in Rust than there are in C++. One of them is the coercion from fixed
length arrays to slices. Since slices must be pointer values, this is
effectively a coercion between pointers. For example, we can coerce `&[i32; 4]`
to `&[i32]`, e.g.,

创建 slice 最简单的方法是 强制转换。 Rust种的 隐式强制转换 比C++少得多，其中一种情况是 定长 array 转 slice。由于 slice 必须是一个指针值，指针之间的强转 是效果很高的。比如，我们可以将 `&[i32; 4]` 强转成 `&[i32]` ,示例： 

```rust
let a: &[i32] = &[1, 2, 3, 4];
```

Here the right hand side is a fixed length array of length four, allocated on
the stack. We then take a reference to it (type `&[i32; 4]`). That reference is
coerced to type `&[i32]` and given the name `a` by the let statement.

这里，左边是一个定长为4的 array，在 栈中分配，我么可以对它获取一个引用 （`&[i32, 4]`）。这个引用被强制转换成  `&[i32]` 并命名为  `a` 。  

Again, access is just like C (using `[...]`), and access is bounds checked. You
can also check the length yourself by using `len()`. So clearly the length of
the array is known somewhere. In fact all arrays of any kind in Rust have known
length, since this is essential for bounds checking, which is an integral part
of memory safety. The size is known dynamically (as opposed to statically in the
case of fixed length arrays), and we say that slice types are dynamically sized
types (DSTs, there are other kinds of dynamically sized types too, they'll be
covered elsewhere).

同样,  slice访问也是类C风格的( `s[]` )，同时也是有边界检查的。你也可以用 `len()` 求 slice的长度。对于 array 来说，长度肯定是一致的，事实上，Rust中任意类型的 array 都有一个已知的长度，因为长度对边界检查是必不可少的。对于slice 来说，size是动态的， 我们称  slice 类型 是 动态尺寸类型(dynamically sized types, DSTs, Rust 还有其他类型的动态尺寸类型，会在其他的地方涉及到 )。 

Since a slice is just a sequence of values, the size cannot be stored as part of
the slice. Instead it is stored as part of the pointer (remember that slices
must always exist as pointer types). A pointer to a slice (like all pointers to
DSTs) is a fat pointer - it is two words wide, rather than one, and contains the
pointer to the data plus a payload. In the case of slices, the payload is the
length of the slice.

因为 slice 就是 一个 若干个值的 序列，size 不会存放在 slice 里，而是存在一个 指针中(记住，slice 必须以 指针类型的形式存在)。一个指向slice的指针(所有的DSTs指针都一样) 是一个 fat pointer 。fat pointer 是 2字宽，而不是1字宽，包含一个指向 data的指针 和一个 payload。对于slice ，payload 是 slice的长度。

So in the example above, the pointer `a` will be 128 bits wide (on a 64 bit
system). The first 64 bits will store the address of the `1` in the sequence
`[1, 2, 3, 4]`, and the second 64 bits will contain `4`. Usually, as a Rust
programmer, these fat pointers can just be treated as regular pointers. But it
is good to know about (it can affect the things you can do with casts, for
example).

所以上面的例子中， 这个指针 128 bit 宽度 (假设是在 64bit系统中)。第一个 64bit 存储 序列[1, 2, 3, 4] 中 1所在的地址，第二个 64bit 存着 数字 4。作为一个Rust程序员，通常  fat pointer 被看做是 常规的指针。但是理解它总归是一件好事（例如，它会影响您可以进行的类型转换操作）


### Slicing notation and ranges

A slice can be thought of as a (borrowed) view of an array. So far we have only
seen a slice of the whole array, but we can also take a slice of part of an
array. There is a special notation for this which is like the indexing
syntax, but takes a range instead of a single integer. E.g., `a[0..4]`, which
takes a slice of the first four elements of `a`. Note that the range is
exclusive at the top and inclusive at the bottom. Examples:

slice 可以被看做是 对应的array的 一个  (borrowed) 视图。目前，我么只看过 整个 array 的 slice，但是我们同样可以 获取一个 部分 array 的 slice。这里有一个很像下标索引的符号，但是传递的是一个范围，而不是一个单独的整数。比如 `a[0..4]` 获取一个slice, 对应的是 a 的前四个元素。注意，这个范围是 左包含，右不包含的，即左闭右开区间。示例：

```rust
let a: [i32; 4] = [1, 2, 3, 4];
let b: &[i32] = &a;   // Slice of the whole array.
let c = &a[0..4];     // Another slice of the whole array, also has type &[i32].
let c = &a[1..3];     // The middle two elements, &[i32].
let c = &a[1..];      // The last three elements.
let c = &a[..3];      // The first three elements.
let c = &a[..];       // The whole array, again.
let c = &b[1..3];     // We can also slice a slice.
```

Note that in the last example, we still need to borrow the result of slicing.
The slicing syntax produces an unborrowed slice (type: `[i32]`) which we must
then borrow (to give a `&[i32]`), even if we are slicing a borrowed slice.

···咋翻译呢...  

Range syntax can also be used outside of slicing syntax. `a..b` produces an
iterator which runs from `a` to `b-1`. This can be combined with other iterators
in the usual way, or can be used in `for` loops:

Range 语法 也可以用于  slice 之外， `a..b`  产生一个 迭代器，从 a 到 b-1，可以连结其他的迭代器，或者用于 for 循环：

```rust
// Print all numbers from 1 to 10.
for i in 1..11 {
    println!("{}", i);
}
```

## Vecs

A vector is heap allocated and is an owning reference. Therefore (and like
`Box<_>`), it has move semantics. We can think of a fixed length array
analogously to a value, a slice to a borrowed reference. Similarly, a vector in
Rust is analogous to a `Box<_>` pointer.

`vector` 是 堆分配，它是一个  owning 引用，因此（ 就像 `Box<T>` ） 它有 move 语义。我们可以将定长的array 类比成 一个值，slice 类比成 一个 borrowed 引用，vector 类比 成  一个 `Box<T>` 指针，即 owning 引用。

It helps to think of `Vec<_>` as a kind of smart pointer, just like `Box<_>`,
rather than as a value itself. Similarly to a slice, the length is stored in the
'pointer', in this case the 'pointer' is the Vec value.

为了便于立即，可以将 `vec<T>` 看成是一种智能指针，就像 `Box<T>` ，而不是一个值本身。和 slice 类型， vec的长度也是存储在这个指针，这里的指针指的是  `Vec` 的值。 

A vector of `i32`s has type `Vec<i32>`. There are no vector literals, but we can
get the same effect by using the `vec!` macro. We can also create an empty
vector using `Vec::new()`:

一个 `i32`类型的vector的类型是  `Vec<i32>` 。Rust 没有 vector 的字面值初始化语法，但是可以用 宏`vec!` 来达到同样的效果的；我们也可以童 `Vec::new()` 来创建一个空 vector :

```rust
let v = vec![1, 2, 3, 4];      // A Vec<i32> with length 4.
let v: Vec<i32> = Vec::new();  // An empty vector of i32s.
```

In the second case above, the type annotation is necessary so the compiler can
know what the vector is a vector of. If we were to use the vector, the type
annotation would probably not be necessary.

上面的第二个例子，类型符号是必须的，编译器才能知道这个vector装的是什么类型的元素；如果我们是使用 vector，则类型符号不是必须的。

Just like arrays and slices, we can use indexing notation to get a value from
the vector (e.g., `v[2]`). Again, these are bounds checked. We can also use
slicing notation to take a slice of a vector (e.g., `&v[1..3]`).

就像 array 和 slice，我们可以用索引符号来获取vector的成员值（`v[2]`），同样也是有边界检查的。我们也可以使用 分片符号来获取一个 vector的分片 （比如：`&v[1..3]`）。

The extra feature of vectors is that their size can change - they can get longer
or shorter as needed. For example, `v.push(5)` would add the element `5` to the
end of the vector (this would require that `v` is mutable). Note that growing a
vector can cause reallocation, which for large vectors can mean a lot of
copying. To guard against this you can pre-allocate space in a vector using
`with_capacity`, see the [Vec docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)
for more details.

vector的一个额外特性是，长度是可变的 —— 可以在需要的时候变长变短。举个例子，`v.push(5)` 添加元素 `5` 到 vector 的末尾（当然前提是 v 是 mutable）。 注意， 拓展一个 vector 会引起重新分配，vector越大，需要的拷贝也越多。应对这种问题，你可以 使用 `with_capacity` 进行 空间 预分配。  


## The `Index` traits

Note for readers: there is a lot of material in this section that I haven't
covered properly yet. If you're following the tutorial, you can skip this
section, it is a somewhat advanced topic in any case.

读者需要注意的是：本节还有很多内容还没有覆盖到，如果你是按这个课程的顺序读下来的，你可以先跳过这一节。

The same indexing syntax used for arrays and vectors is also used for other
collections, such as `HashMap`s. And you can use it yourself for your own
collections. You opt-in to using the indexing (and slicing) syntax by
implementing the `Index` trait. This is a good example of how Rust makes
available nice syntax to user types, as well as built-ins (`Deref` for
dereferencing smart pointers, as well as `Add` and various other traits, work in
a similar way).

The `Index` trait looks like

```rust
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}
```

`Idx` is the type used for indexing. For most uses of indexing this is `usize`.
For slicing this is one of the `std::ops::Range` types. `Output` is the type
returned by indexing, this will be different for each collection. For slicing it
will be a slice, rather than the type of a single element. `index` is a method
which does the work of getting the element(s) out of the collection. Note that
the collection is taken by reference and the method returns a reference to the
element with the same lifetime.

Let's look at the implementation for `Vec` to see how what an implementation
looks like:

```rust
impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &(**self)[index]
    }
}
```

As we said above, indexing is done using `usize`. For a `Vec<T>`, indexing will
return a single element of type `T`, thus the value of `Output`. The
implementation of `index` is a bit weird - `(**self)` gets a view of the whole
vec as a slice, then we use indexing on slices to get the element, and finally
take a reference to it.

If you have your own collections, you can implement `Index` in a similar way to
get indexing and slicing syntax for your collection.


## Initialiser syntax

As with all data in Rust, arrays and vectors must be properly initialised. Often
you just want an array full of zeros to start with and using the array literal
syntax is a pain. So Rust gives you a little syntactic sugar to initialise an
array full of a given value: `[value; len]`. So for example to create an array
with length 100 full of zeros, we'd use `[0; 100]`.

Similarly for vectors, `vec![42; 100]` would give you a vector with 100
elements, each with the value 42.

The initial value is not limited to integers, it can be any expression. For
array initialisers, the length must be an integer constant expression. For
`vec!`, it can be any expression with type `usize`.


##### 1

In C++11 there is `std::array<T, N>` that provides boundary checking when
`at()` method is used.
