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

这个教程一方面是对我（@nrc）学习Rust的一种帮助（没有比尝试向他人解释的方法来检查您是否学过的东西更好的方法），另一方面是因为我发现学习Rust的现有资源不尽人意——他们**在我已经知道的基础知识上花费了太多时间，并使用了较高级别的直觉来描述可以用较低级别的直觉更好地向我解释的概念**。尽管在那以后，Rust的文档在逐步完善，但是我仍然认为**现有的C ++程序员是Rust的天生目标，但Rust文档并没有很好地满足他们的需求**。


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

Obviously, the intended audience is C++ programmers. The tutorial should
concentrate on things that will be new to experienced C++ programmers, rather
than a general audience (although, I don't assume the audience is familiar with
the most recent versions of C++). I'd like to avoid too much basic material and
definitely avoid too much overlap with other resources, in particular the Rust
guide/book.

Work on edge case use cases (e.g., using a different build system from Cargo, or
writing syntax extensions, using unstable APIs) is definitely welcome, as is
in-depth work on topics already covered at a high level.

I'd like to avoid recipe-style examples for converting C++ code to Rust code,
but small examples of this kind are OK.

Use of different formats (e.g., question and answer/FAQs, or larger worked
examples) are welcome.

I don't plan on adding exercises or suggestions for mini-projects, but if you're
interested in that, let me know.

I'm aiming for a fairly academic tone, but not too dry. All writing should be in
English (British English, not American English; although I would be very happy
to have localisations/translations into any language, including American
English) and be valid GitHub markdown. For advice on writing style, grammar,
punctuation, etc. see the Oxford Style Manual
or [The Economist Style Guide](http://www.economist.com/styleguide/introduction).
Please limit width to 80 columns. I am a fan of the Oxford comma.

Don't feel like work has to be perfect to be submitted, I'm happy to edit and
I'm sure other people will be in the future.
