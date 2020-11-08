# Graphs and arena allocation

(Note you can run the examples in this chapter by downloading this directory and
running `cargo run`).

（注意：可以 `cargo run`来运行本路径下的例子）

Graphs are a bit awkward to construct in Rust because of Rust's stringent
lifetime and mutability requirements. Graphs of objects are very common in OO
programming. In this tutorial I'm going to go over a few different approaches to
implementation. My preferred approach uses arena allocation and makes slightly
advanced use of explicit lifetimes. I'll finish up by discussing a few potential
Rust features which would make using such an approach easier.

在 Rust 中构建 图 是有一点尴尬的，因为 Rust 严格的生命周期 和 可变性 的要求。对象的图 在面向对象语言中是非常常见的。在本教程中，我将用一些不一样的方法实现。我更偏爱的方法是使用  arena 分配 和 少量使用 生命周期。最后，我将讨论一些潜在的Rust功能，这些功能将使实现方法更加容易。

A [graph](http://en.wikipedia.org/wiki/Graph_%28abstract_data_type%29) is a
collection of nodes with edges between some of those nodes. Graphs are a
generalisation of lists and trees. Each node can have multiple children and
multiple parents (we usually talk about edges into and out of a node, rather
than parents/children). Graphs can be represented by adjacency lists or
adjacency matrices. The former is basically a node object for each node in the
graph, where each node object keeps a list of its adjacent nodes. An adjacency
matrix is a matrix of booleans indicating whether there is an edge from the row
node to the column node. We'll only cover the adjacency list representation,
adjacency matrices have very different issues which are less Rust-specific.

[图](http://en.wikipedia.org/wiki/Graph_%28abstract_data_type%29) 是节点的集合，一些节点之间有边。图 是 列表 和 树 的一般形式。每个节点有多个父节点和子节点（在图中，我们倾向于称其为 入边和出边，而不是父节点，子节点）。图可以用 邻接表 或 临界矩阵标识。连接表记录的是图中每个节点的节点对象，其中每个节点对象都保留了其相邻节点的列表。邻接矩阵是布尔矩阵，每个布尔值指示从行节点到列节点之间是否存在边。这里我们只讨论 邻接表的表示形式，邻接矩阵涉及到一些不同的话题，这些话题与 Rust 语言特性本身关系较少。

There are essentially two orthogonal problems: how to handle the lifetime of the
graph and how to handle it's mutability.

这里本质上存在两个问题：如何处理图的生命期；如何处理 图的可变性。

The first problem essentially boils down to what kind of pointer to use to point
to other nodes in the graph. Since graph-like data structures are recursive (the
types are recursive, even if the data is not) we are forced to use pointers of
some kind rather than have a totally value-based structure. Since graphs can be
cyclic, and ownership in Rust cannot be cyclic, we cannot use `Box<Node>` as our
pointer type (as we might do for tree-like data structures or linked lists).

第一个问题总结起来就是 使用什么类型的指针存储图中的其他的节点。由于 图类型数据结构存在递归(类型是递归的，即使数据可能不是)，我们倾向于使用 指针 而不是  值类型的数据结构。由于 图 可能有环，而 Rust 的所有权 不能有环，所有不能使用 `Box<Node>` 来存储（树和链表不存在环，所以可以 `Box<Node>` ）。  

No graph is truly immutable. Because there may be cycles, the graph cannot be
created in a single statement. Thus, at the very least, the graph must be mutable
during its initialisation phase. The usual invariant in Rust is that all
pointers must either be unique or immutable. Graph edges must be mutable (at
least during initialisation) and there can be more than one edge into any node,
thus no edges are guaranteed to be unique. So we're going to have to do
something a little bit advanced to handle mutability.

没有图是真正不变的。由于可能存在循环，因此无法在单个语句中创建图。所以，至少图在其初始化阶段必须是可变的。对于Rust来说，通常指针要么是唯一，要么是不可变的。图的边缘必须是可变的（至少在初始化期间），并且任何节点中可能有多个边缘，因此不能保证任何边缘都是唯一的。因此，我们不得不用一些高级操作来处理可变性。

One solution is to use mutable raw pointers (`*mut Node`). This is the most
flexible approach, but also the most dangerous. You must handle all the lifetime
management yourself without any help from the type system. You can make very
flexible and efficient data structures this way, but you must be very careful.
This approach handles both the lifetime and mutability issues in one fell swoop.
But it handles them by essentially ignoring all the benefits of Rust - you will
get no help from the compiler here (it's also not particularly ergonomic since
raw pointers don't automatically (de-)reference). Since a graph using raw
pointers is not much different from a graph in C++, I'm not going to cover that
option here.

一个方案 是 使用 mutable raw pointers( `*mut Node` )。这是最灵活的一种方法，同时也是最危险的。你必须自己处理所有的生命期管理，没法借助类型系统的任何机制。用这种方法你可以 创造 非常灵活 和 高效 的数据结构，但必须非常小心。这种方法一下子解决了生命期和可变性问题，不过 是在 忽略 Rust 的所有优秀语言特性下实现的——你不能从编译器上得到任何帮助。（它也是不符合人体工学设计的，因为原始指针不支持 自动 引用/解引用）。因为 用 原始指针实现的图和C++并没有太多区别，所有这种方式我就不过多讨论了。

The options you have for lifetime management are reference counting (shared
ownership, using `Rc<...>`) or arena allocation (all nodes have the same lifetime,
managed by an arena; using borrowed references `&...`). The former is
more flexible (you can have references from outside the graph to individual
nodes with any lifetime), the latter is better in every other way.

生命周期管理的一种方式是使用引用计数（共享所有权，使用`Rc <...>`）或 arena分配（竞技场分配，所有节点具有相同的生存期，由竞技场管理；使用borrowed的引用 `＆...`）。前者更灵活（您可以从图外部引用具有任何生存期的单个节点），而后者则在其他方面表现更好。

For managing mutability, you can either use `RefCell`, i.e., make use of Rust's
facility for dynamic, interior mutability, or you can manage the mutability
yourself (in this case you have to use `UnsafeCell` to communicate the interior
mutability to the compiler). The former is safer, the latter is more efficient.
Neither is particularly ergonomic.

对于生命期管理，你可以使用 `RefCell`，利用 Rust 特性 来实现动态的内部可变性，或者自行管理可变性（这里，你必须使用 `unsafeCell`来向 编译器 通信 内部可变性）。前者更加安全，而后者更加高效，两者都不怎么符合人体工学。 

Note that if your graph might have cycles, then if you use `Rc`, further action
is required to break the cycles and not leak memory. Since Rust has no cycle
collection of `Rc` pointers, if there is a cycle in your graph, the ref counts
will never fall to zero, and the graph will never be deallocated. You can solve
this by using `Weak` pointers in your graph or by manually breaking cycles when
you know the graph should be destroyed. The former is more reliable. We don't
cover either here, in our examples we just leak memory. The approach using
borrowed references and arena allocation does not have this issue and is thus
superior in that respect.

要注意的是，图是可能存在环的，如果你使用 `Rc` ,后续的操作就需要 解环 和 保证 没有内存泄漏。由于 Rust 有 `Rc` 指针的 无环容器，如果图种存在一个环，则 引用计数 永远不会为0，这个图永远不会被回收内存。你可以使用 `weak pointer` 来解决这个问题，或者在知道图应该被销毁的时候，手动的把环解掉。前者更可靠，这里我们2种方法都没有覆盖，在我们的示例中，只是造成了泄漏内存。使用borrowed引用 和 arena分配的方法没有这个问题，因此在这方面表现更佳。 

To compare the different approaches I'll use a pretty simple example. We'll just
have a `Node` object to represent a node in the graph, this will hold some
string data (representative of some more complex data payload) and a `Vec` of
adjacent nodes (`edges`). We'll have an `init` function to create a simple graph
of nodes, and a `traverse` function which does a pre-order, depth-first
traversal of the graph. We'll use this to print the payload of each node in the
graph. Finally, we'll have a `Node::first` method which returns a reference to
the first adjacent node to the `self` node and a function `foo` which prints the
payload of an individual node. These functions stand in for more complex
operations involving manipulation of a node interior to the graph.

为了比较 各种不同的方法，我会用一个 很简单的例子来演示。我们会只使用一个 `node` 对象来表示 图中的一个节点，会保存一些字符串数据(表示一些更复杂的数据负载) 和 一个 邻接节点的 容器 `Vec` 。我们会有一个 `init` 方法 来 创建一个图，一个 `traverse` 方法 以 前序，深度邮箱的方式遍历图。我们使用 

To try and be as informative as possible without boring you, I'll cover two
combinations of possibilities: ref counting and `RefCell`, and arena allocation
and `UnsafeCell`. I'll leave the other two combinations as an exercise.

为了给你一个更直观的体验，我将 介绍 两种组合 方案:  引用计数器 + `RefCell`，arena 分配器  + `unsafeCell` 。另外2种组合方案供读者自行练习。


## `Rc<RefCell<Node>>`

See [full example](src/rc_graph.rs).

This is the safer option because there is no unsafe code. It is also the least
efficient and least ergonomic option. It is pretty flexible though, nodes of the
graph can be easily reused outside the graph since they are ref-counted. I would
recommend this approach if you need a fully mutable graph, or need your nodes to
exist independently of the graph.

这是比较安全的方案，因为没有用到 unsafe 代码，当然也是效率最低，最不符合人体工学的方案。它的节点非常灵活，由于可以对节点进行引用计数，图的节点可以可以在在外被简单地重用。如果需要 完全可变的图，或者需要节点独立于图而存在，推荐使用这种方案。

The node structure looks like

```rust
struct Node {
    datum: &'static str,
    edges: Vec<Rc<RefCell<Node>>>,
}
```

Creating a new node is not too bad: `Rc::new(RefCell::new(Node { ... }))`. To
add an edge during initialisation, you have to borrow the start node as mutable,
and clone the end node into the Vec of edges (this clones the pointer,
incrementing the reference count, not the actual node). E.g.,

创建一个新节点看起来还不错： `Rc::new(RefCell::new(Node{...}))` 。为了在初始化过程中添加边，你必须要 borrow 起点节点 为可变，然后clone 终点节点 到 边 Vec （这是clone 指针，累计引用计数，不是clone真正的节点）。示例： 

```rust
let mut mut_root = root.borrow_mut();
mut_root.edges.push(b.clone());
```

The `RefCell` dynamically ensures that we are not already reading or writing the
node when we write it.

`RefCell` 动态地确保我们在写操作的时候，没有其他的读或写。

Whenever you access a node, you have to use `.borrow()` to borrow the `RefCell`.
Our `first` method has to return a ref-counted pointer, rather than a borrowed
reference, so callers of `first` also have to borrow:

无论你什么时候访问节点，都必须使用 `.borrow()` 来借用这个 `RefCell` 。 `first` 方法返回 一个 引用计数 指针，而不是一个 borrowed reference，所以 `first` 的调用者也必须 borrow 。

```rust
fn first(&self) -> Rc<RefCell<Node>> {
    self.edges[0].clone()
}

pub fn main() {
    let g = ...;
    let f = g.first();
    foo(&*f.borrow());
}
```


## `&Node` and `UnsafeCell`

See [full example](src/ref_graph.rs).

In this approach we use borrowed references as edges. This is nice and ergonomic
and lets us use our nodes with 'regular' Rust libraries which primarily operate
with borrowed references (note that one nice thing about ref counted objects in
Rust is that they play nicely with the lifetime system. We can create a borrowed
reference into the `Rc` to directly (and safely) reference the data. In the
previous example, the `RefCell` prevents us doing this, but an `Rc`/`UnsafeCell`
approach should allow it).

这种方案 我们使用 borrowed reference 来表示 边。这种方式优雅且符合人体工学，并可以将节点与操作 borrowed reference 的常规 Rust 库函数一起使用（注意：关于引用计数对象，好的一点是它们与生命期系统协作很好）。我们可以创建一个 `borrowed reference` 进 `Rc` 来直接且安全地引用数据。在前面的例子中，`RefCell` 不允许我们做到直接引用数据， 但是 `Rc/UnsafeCell` 理应允许这样做。

Destruction is correctly handled too - the only constraint is that all the nodes
must be destroyed at the same time. Destruction and allocation of nodes is
handled using an arena.

析构也可以正确的处理：唯一的缺点是所有的节点必须同时销毁。节点的析构和分配使用一个arena来处理。

On the other hand, we do need to use quite a few explicit lifetimes.
Unfortunately we don't benefit from lifetime elision here. At the end of the
section I'll discuss some future directions for the language which could make
things better.

另一方面。我们需要使用很多显式的生命期。不幸的是，我们没法利用 系统生命期管理的有点。在本节的最后，我会讨论 语言的未来发展方向，来让这种情况变得更好。

During construction we will mutate our nodes which might be multiply referenced.
This is not possible in safe Rust code, so we must initialise inside an `unsafe`
block. Since our nodes are mutable and multiply referenced, we must use an
`UnsafeCell` to communicate to the Rust compiler that it cannot rely on its
usual invariants.

在构造期间，我们会 mutate 可能被多次引用的节点。这不可能在 safe Rust code 中实现，所以我们必须在unsafe block中 进行初始化。由于我们的节点都是 mutable 且 被多引用，所以必须使用 `unsafeCell` 来告知 Rust编译器 ，这些节点不能依赖于常规的不变性。

When is this approach feasible? The graph must only be mutated during
initialisation. In addition, we require that all nodes in the graph have the
same lifetime (we could relax these constraints somewhat to allow adding nodes
later as long as they can all be destroyed at the same time). Similarly, we
could rely on more complicated invariants for when the nodes can be mutated, but
it pays to keep things simple, since the programmer is responsible for safety
in those respects.

什么时候这种方案是可行的？这个图必须 只在初始化期间是 mutated 。另外，我们需要 所有节点有同样的生命期（我们可以稍微放宽这些约束以允许添加节点，只要它们可以在同一时间被销毁）。同样，我们何时可以更改节点要依赖更复杂的不变性，但是保持事情简单很值得，因为程序员负责保证各个方面的安全。

Arena allocation is a memory management technique where a set of objects have
the same lifetime and can be deallocated at the same time. An arena is an object
responsible for allocating and deallocating the memory. Since large chunks of
memory are allocated and deallocated at once (rather than allocating individual
objects), arena allocation is very efficient. Usually, all the objects are
allocated from a contiguous chunk of memory, that improves cache coherency when
you are traversing the graph.

Arena 分配器 是一个内存管理计数，维护一系列的对象，这些对象具有相同的生命期，可以在同一时间被释放。因为 一大块的内存 被 同时分配和同时释放（而不是一次分配/释放一个对象），arena 分配器是非常高效的。通常，所有的对象都被分配在一段连续的内存块中，在遍历图的时候，可以提高 cache 的相关性。

In Rust, arena allocation is supported by the [libarena](https://doc.rust-lang.org/1.1.0/arena/index.html)
crate and is used throughout the compiler. There are two kinds of arenas - typed
and untyped. The former is more efficient and easier to use, but can only
allocate objects of a single type. The latter is more flexible and can allocate
any object. Arena allocated objects all have the same lifetime, which is a
parameter of the arena object. The type system ensures references to arena
allocated objects cannot live longer than the arena itself.

在Rust，由 [libarena](https://doc.rust-lang.org/1.1.0/arena/index.html) 库 提供对 arena 分配的支持， 并且是编译器支持的。有2种类型的arena—— typed 和 untyped 。前者更高效，且使用更简单，但只能分配单一类型的对象。后者更灵活，可以分配任何的对象。Arena 分类的对象有相同的生命期，这是 arena 对象的一个参数。类型系统确保 arena所分配的对象，其对象的引用 不会比 arena 本身 的生命周期更长。

Our node struct must now include the lifetime of the graph, `'a`. We wrap our
`Vec` of adjacent nodes in an `UnsafeCell` to indicate that we will mutate it
even when it should be immutable:

我们的节点结构必须包含 图的生命期 `'a` 。我们使用 `UnsafeCell` 来封装 我们的 邻接节点 数组 `Vec` ，表明我们将 mutate 它，即使它应该是 immutable ：

```rust
struct Node<'a> {
    datum: &'static str,
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
}
```

Our new function must also use this lifetime and must take as an argument the
arena which will do the allocation:

我们的`new`方法 也必须使用  这个 生命期，并且必须传递一个参数来指定哪个 `arena` 进行分配。

```rust
fn new<'a>(datum: &'static str, arena: &'a TypedArena<Node<'a>>) -> &'a Node<'a> {
    arena.alloc(Node {
        datum: datum,
        edges: UnsafeCell::new(Vec::new()),
    })
}
```

We use the arena to allocate the node. The lifetime of the graph is derived from
the lifetime of the reference to the arena, so the arena must be passed in from
the scope which covers the graph's lifetime. For our examples, that means we
pass it into the `init` method. (One could imagine an extension to the type
system which allows creating values at scopes outside their lexical scope, but
there are no plans to add such a thing any time soon). When the arena goes out
of scope, the whole graph is destroyed (Rust's type system ensures that we can't
keep references to the graph beyond that point).

Adding an edge is a bit different looking:

我们使用 arena 来分配节点。图的生命期 来源于 arena 的引用的生命期，所以 arena 必须被传进这个覆盖图生命周期的 scope。比如我们的例子种，意味着我们将它传递进 `init`方法。（可以设想类型系统有一个插件，可以允许在其词法范围之外创建值，但是目前还没有添加此类插件的计划）。当 arena 离开scope，整个 graph 被销毁。(Rust 类型系统 确保我们 对 图的引用 不会超过这个时间点)。

添加一个边的方式变得有点不一样了：

```rust
(*root.edges.get()).push(b);
```

We're essentially doing the obvious `root.edges.push(b)` to push a node (`b`) on
to the list of edges. However, since `edges` is wrapped in an `UnsafeCell`, we
have to call `get()` on it. That gives us a mutable raw pointer to edges (`*mut
Vec<&Node>`), which allows us to mutate `edges`. However, it also requires us to
manually dereference the pointer (raw pointers do not auto-deref), thus the
`(*...)` construction. Finally, dereferencing a raw pointer is unsafe, so the
whole lot has to be wrapped up in an unsafe block.

The interesting part of `traverse` is:

我们通过 `root.edges.push(b)` 来讲 一个节点`b` 添加到这个 边列表。然而，由于 `edges` 被封装进一个 `UnsafeCell` ,我们必须对它调用 `get()` ，这可以获取一个指向 edges 的 原始指针 ( `*mut Vec<&Node>` )，它允许我们修改 `edges` 。但是，这需要我们手动的解引用这个指针（raw pointer 不支持 auto-deref），即 `(*...)` 来解引用。最后，解引用一个 原始指针 是不安全的，所以 相关的操作都必须 放在  unsafe block内。

`traverse()` 方法中有意思的部分是：

```rust
for n in &(*self.edges.get()) {
    n.traverse(f, seen);
}
```

We follow the previous pattern for getting at the edges list, which requires an
unsafe block. In this case we know it is in fact safe because we must be post-
initialisation and thus there will be no mutation.

我们遵循前面的模式来获取edges list，这需要一个 unsafe block 。在这中情况下，我们知道它其实是安全的，因为我们必须后初始化，所以这里不会有变换发生。

Again, the `first` method follows the same pattern for getting at the `edges`
list. And again must be in an unsafe block. However, in contrast to the graph
using `Rc<RefCell<_>>`, we can return a straightforward borrowed reference to
the node. That is very convenient. We can reason that the unsafe block is safe
because we do no mutation and we are post-initialisation.

同样，`first` 方法也准寻同样的模式来获取 edges 同样也必须在 unsafe block 中。但是，与 使用 `Rc<RefCell<T>>` 的图相反，我们可以返回一个 指向该节点的直接借用引用(straightforward borrowed reference)。这是很方便的。我们可以保证 unsafe block 实际上是安全的，我们没有做任何的修改，而是 后初始化。

```rust
fn first(&'a self) -> &'a Node<'a> {
    unsafe {
        (*self.edges.get())[0]
    }
}
```

### Future language improvements for this approach

I believe that arena allocation and using borrowed references are an important
pattern in Rust. We should do more in the language to make these patterns safer
and easier to use. I hope use of arenas becomes more ergonomic with the ongoing
work on [allocators](https://github.com/rust-lang/rfcs/pull/244). There are
three other improvements I see:

我相信  arena 分配 和 使用 borrowed reference 是Rust的一个重要范式。我们应该在语言上做更多的事情来让这个模式更加安全和更加易用。我希望正在进行的工作[allocators](https://github.com/rust-lang/rfcs/pull/244)能让 arenas 的使用 变得更符合人类思维。这里我还看到了三个可能的改进点：

#### Safe initialisation

There has been lots of research in the OO world on mechanisms for ensuring
mutability only during initialisation. How exactly this would work in Rust is an
open research question, but it seems that we need to represent a pointer which
is mutable and not unique, but restricted in scope. Outside that scope any
existing pointers would become normal borrowed references, i.e., immutable *or*
unique.

The advantage of such a scheme is that we have a way to represent the common
pattern of mutable during initialisation, then immutable. It also relies on the
invariant that, while individual objects are multiply owned, the aggregate (in
this case a graph) is uniquely owned. We should then be able to adopt the
reference and `UnsafeCell` approach, without the `UnsafeCell`s and the unsafe
blocks, making that approach more ergonomic and more safer.

Alex Summers and Julian Viereck at ETH Zurich are investigating this
further.

面向对象领域里有很多关于 仅在初始化期间确保可变性机制的研究。如何在Rust中做到这个一点仍然是一个待解决的问题，但是理论上 我们需要一个 可变的指针，该指针不是唯一的的，但范围是有限的。在该范围之外，任何存在的指针都回变成 普通的借用引用，要么是不可变的，或者是唯一的。




#### Generic modules

The 'lifetime of the graph' is constant for any particular graph. Repeating the
lifetime is just boilerplate. One way to make this more ergonomic would be to
allow the graph module to be parameterised by the lifetime, so it would not need
to be added to every struct, impl, and function. The lifetime of the graph would
still need to be specified from outside the module, but hopefully inference
would take care of most uses (as it does today for function calls).

See [ref_graph_generic_mod.rs](src/ref_graph_generic_mod.rs) for how that might look.
(We should also be able to use safe initialisation (proposed above) to remove
the unsafe code).

See also this [RFC issue](https://github.com/rust-lang/rfcs/issues/424).

This feature would vastly reduce the syntactic overhead of the reference and
`UnsafeCell` approach.

对于一个特定的图，图的生命周期 都是固定的。重复寿命只是样板。使该模块更符合人类思维的一种方法是，可以根据生命周期对图模块进行参数化，因此无需将其添加到每个结构，隐式函数和函数中。图的生存期仍然需要从模块外部指定，但是希望推断可以处理大多数用途（就像今天的函数调用一样）。

请阅读相关代码体验以下，  [ref_graph_generic_mod.rs](src/ref_graph_generic_mod.rs) 

(我们还应该能够使用安全的初始化（上面提出过）来移除  unsafe code) 。

也可以查阅  [RFC issue](https://github.com/rust-lang/rfcs/issues/424).

这个特性 将大大减少引用和UnsafeCell方法的语法开销。


#### Lifetime elision

We currently allow the programmer to elide some lifetimes in function signatures
to improve ergonomics. One reason the `&Node` approach to graphs is a bit ugly
is because it doesn't benefit from any of the lifetime elision rules.

A common pattern in Rust is data structures with a common lifetime. References
into such data structures give rise to types like `&'a Foo<'a>`, for example
`&'a Node<'a>` in the graph example. It would be nice to have an elision
rule that helps in this case. I'm not really sure how it should work though.

Looking at the example with generic modules, it doesn't look like we need to
extend the lifetime elision rules very much (I'm not actually sure if
`Node::new` would work without the given lifetimes, but it seems like a fairly
trivial extension to make it work if it doesn't). We might want to add some new
rule to allow elision of module-generic lifetimes if they are the only ones in
scope (other than `'static`), but I'm not sure how that would work with multiple
in- scope lifetimes (see the `foo` and `init` functions, for example).

If we don't add generic modules, we might still be able to add an elision rule
specifically to target `&'a Node<'a>`, not sure how though.

目前，我们允许程序员在函数声明中 省略一些生命期，以改善人体工学。`&Node`方法有点不好看的一个原因是，它没有享受到 任何 生命期省略规则的好处。

Rust 中常见的模式是 具有通用生命期的数据结构。对此类数据结构的引用回产生 `&'a Foo<'a>` 之类的类型，比如例子中的 `&'a Node<'a>` 。在这种情况下，有省略生命期的规则是很有帮助的。我还不清楚应该怎么做到这一点。

看看使用 generic modules的例子，看起来我们并不是很需要拓展 生命期省略规则（我不确定`Node:new` 在没有给定生命期的情况下是不是也能正常工作，如果不能运行，似乎需要很琐碎的插件来使它能够正常运行）。如果这个通用模块的生命期在scope中是唯一的（且不是 ’static），则我们可以添加一些规则来省略它。但是我不知道一个 scope中有多个生命期的情况下，该怎么处理。(比如例子中的 `foo`函数 和 `init` 函数 ) .  

如果我们不添加 generic modules，我们或许仍然可以添加针对 `&'a Node<'a>` 这种情况的省略规则，但是我还不知道怎么做是可行的。

