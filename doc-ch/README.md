# Rust For Systems Programmers

一个面向C/C++程序员的Rust 教程.

Jump to [contents](#contents).
Jump to [contributing](#contributing).

该教程是面向那些 理解指针和引用是如何工作的，并且已经习惯了整形变量宽度和内存管理的 程序员。我们 主要覆盖 Rust 和 C++ 间的差异，帮助你快速的编写Rust程序，而不需要去学习那些你已经知道的冗余知识点。

理论上，Rust 对 C++ 程序员来说是非常亲切的，它们有很多相似的语法。以我的经验来看，最大的差异是：一些好的系统编程上的模糊概念，Rust 编译器上是强制执行的（将编程思想固化到编译器，倒逼程序员去做到，避免将问题留到程序运行阶段）。

- 一开始这会让程序员恼火-有些事情你想做，但是编译器不会（至少在安全代码中）允许您这样做，即使有时候这些事情是安全的，您也不能说服编译器这样做。
- 但是，您将很快对编译器所允许的内容建立良好的直觉。
  - **向编译器传达您自己的内存安全性概念需要一些新的，有时是复杂的类型注释**。
  - 如果您对对象的生命周期有很深的了解，并且对通用编程有丰富的经验，那么学习它们就不会太困难。

这个教程开始于我的一个 [博客系列](http://featherweightmusings.blogspot.co.nz/search/label/rust-for-c) 。

这个教程一方面是对我（@nrc）学习Rust的一种帮助（没有比尝试向他人解释的方法来检查您是否学过的东西更好的方法），另一方面是因为我发现学习Rust的现有资源不尽人意——他们**在我已经知道的基础知识上花费了太多时间，并使用了较高级别的直觉来描述可以用较低级别的直觉更好地向我解释的概念**。尽管在那以后，Rust的文档在逐步完善，但是我仍然认为**C ++程序员是Rust天生的目标群体，但Rust文档并没有很好地满足C++程序员的需求**。


## Contents

1. [Introduction - Hello world!](hello-world.md)
1. [Control flow](control-flow.md)
1. [Primitive types and operators](primitives.md)
1. [Unique pointers](unique.md)
1. [Borrowed pointers](borrowed.md)
1. [Rc and raw pointers](rc-raw.md)
1. [Data types](data-types.md)
1. [Destructuring pt 1](destructuring.md)
1. [Destructuring pt 2](destructuring-2.md)
1. [Arrays and vecs](arrays.md)
1. [Graphs and arena allocation](graphs/README.md)
1. [Closures and first-class functions](closures.md)


## Other resources

* [The Rust book/guide](http://doc.rust-lang.org/book/) - the best place for
  learning Rust in general and probably the best place to go for a second opinion
  on stuff here or for stuff not covered.
* [Rust API documentation](http://doc.rust-lang.org/std/index.html) - detailed
  documentation for the Rust libraries.
* [The Rust reference manual](https://doc.rust-lang.org/reference/) - a little
  out of date in places, but thorough; good for looking up details.
* [Discuss forum](http://users.rust-lang.org/) - general forum for discussion or
  questions about using and learning Rust.
* [StackOverflow Rust questions](https://stackoverflow.com/questions/tagged/rust) - answers
  to many beginner and advanced questions about Rust, but be careful though - Rust
  has changed *a lot* over the years and some of the answers might be very out of date.


## Contributing

Yes please!

If you spot a typo or mistake, please submit a PR, don't be shy! Please feel
free to file [an issue](https://github.com/nrc/r4cppp/issues/new) for
larger changes or for new chapters you'd like to see. I'd also be happy to see
re-organisation of existing work or expanded examples, if you feel the tutorial
could be improved in those ways.

If you'd like to contribute a paragraph, section, or chapter please do! If you
want ideas for things to cover, see the [list of issues](https://github.com/nrc/r4cppp/issues),
in particular those tagged [new material](https://github.com/nrc/r4cppp/labels/new%20material).
If you're not sure of something, please get in touch by pinging me here
(@nrc) or on irc (nrc, on #rust or #rust-internals).


### Style

很明显，本教程的目标受众是C++程序员。本教程理应专注于那些对C++程序员来说是新颖的事物，避免与其他通用 Rust 文档的冗余和重叠。

Work on edge case use cases (e.g., using a different build system from Cargo, or
writing syntax extensions, using unstable APIs) is definitely welcome, as is
in-depth work on topics already covered at a high level.

非主流的用例(比如使用Cargo之外的工具链，编写语法扩展，使用非stable接口的)也是欢迎的，就像是对 已经在高层次上覆盖的内容 做深入研究。

我会尽量避免使用一些僵硬的例子，将C++代码直接翻译成Rust代码，但有一些这样的小例子是OK的（用于直观地对语言特性做类比）。

使用不同的形式也是欢迎的。(比如：问答形式/FAQs, 或者 更大的示例)

我目前 没有为 mini-projects 添加 练习或建议的计划，但如果你感兴趣的话，请联系我。

我的目标是保持学术风格，但不要太干。所有写作均应使用英语（英式英语，而不是美式英语；不过我很欢迎将文档本地化/翻译成任何语言，包括美式英语），并且必须是有效的GitHub markdown。有关写作风格，语法，标点符号等方面的建议，请参阅 Oxford Style Manualor 或 [The Economist Style Guide](http://www.economist.com/styleguide/introduction)。请将宽度限制为80列。我是 Oxford comma 的粉丝。

不要觉得工作必须完美才能提交，我很高兴为你修正，而且我相信其他人将来也会这样做。



