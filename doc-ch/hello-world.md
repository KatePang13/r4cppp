# Introduction - hello world!

如果你熟悉C/C++，当然很有可能是因为你不得不熟悉 C/C++ —— 或许是因为需要访问操作系统底层，或者是需要极致的性能。**Rust 致力于 提供 同样底层的内存抽象，同样的高性能，并且 提供 更高的安全性和生产力**。

当然，除了C++, 现在有 很多的编程语言可够选择：Java, Scala, Haskell, Python 等等，但是你无法切换到这些语言上，因为它们的抽象级别太高了（你无法直接访问内存，必须使用垃圾回收器，等等），或者因为这些语言性能上的问题（性能不可预测（译者：容易因为虚拟机等原因而产生性能抖动），或者不够快）。Rust 没有 强制你使用垃圾回收，C++也是，你可以获取指向内存的原始指针。Rust不会强迫您使用垃圾回收，并且像在C ++中一样，您会获得指向内存的原始指针。 Rust遵循C ++的 “**Pay for what you use**” 的哲学。如果您不使用某个功能，则无需为它的存在付出任何性能开销。此外，Rust中的所有语言功能的开销都有可预测（通常很小）。

这些约束条件 使得 Rust成为C ++可行的替代品，当然，Rust 也有它自己的优势，那就是 **内存安全**

- Rust 类型系统 保证你不会遇到 C++里经常出现的内存错误( 内存泄漏，访问未初始化的内存，空悬指针 )，这些在Rust 里都是不可能出现的。
- 此外，即使其他约束条件没问题，Rust也会努力防止其他安全问题-例如，对所有数组索引进行边界检查（当然，**如果要避免开销，可以以安全为代价，不做额外的安全检查，Rust确保不安全块中的不安全性停留在不安全块中，并且不会影响程序的其余部分**）。最后，**Rust从现代编程语言中汲取了许多概念，并将它们引入了系统语言空间**。希望这可以使Rust中的编程更加高效，高效和有趣。

本节的剩余内容，我们将下载并安全 Rust, 创建一个最小化的 `minimalCargo`  工程，实现 Hello World。


## Getting Rust

你可以从  [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html)  下载 Rust， 里面包含 Rust 编译器， 标准库 和  Cargo [ Rust包管理器和构建工具集 ] 。

Rust 有三个分支： stable, beta, nightly。Rust的发布模式是快速发布，每六周发布一次新版本。在发布日期，nightly变为beta，beta 变为 stable。

- Nightly  每晚更新，非常适合想要尝试新特性并确保其库可与未来Rust一起使用的用户
- Stable 对 大多数使用者来说是正确的选择。Rust  只对 Stable 提供稳定性保证

- Beta  主要用于用户的CI中，以检查其代码在后续版本将继续按预期运行

如果你是Linux或OS x系统， 最简单的安装方式是 运行如下命令：

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

windows下，有类似的方法：

```
choco install rust
```

更多的安全方式，请参考  [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html).

你可以在Github上访问Rust源码:  [github.com/rust-lang/rust](https://github.com/rust-lang/rust).
要从源码构建编译器，你可以运行  `./configure && make rustc` 。更多细节请参考 [building-from-source](https://github.com/rust-lang/rust#building-from-source) 。


## Hello World!

最简单且常用的构建 Rust 程序 方法是 使用 Cargo。 

 `cargo new --bin hello` 创建一个Cargo项目 `hello`, 它会创建一个新目录 `hello`, 包含一个 `Cargo.toml` 文件 和一个 `src` 文件夹，包含一个源文件 `main.rs`

`Cargo.toml` 定义编译依赖和其他的项目元信息， 后面将会有具体介绍。

All our source code will go in the `src` directory. `main.rs` already contains
a Hello World program. It looks like this:

所有的源文件都会在 `src` 目录， `main.rs` 自动包含了一个 HelloWorld 程序。如下所示:

```rust
fn main() {
    println!("Hello, world!");
}
```

编译程序，可以运行 `cargo build`； 编译并运行，可以执行 `cargo run`。 

Cargo 将会 创建一个 `target` 目录，在这里生成可执行文件。

如果你想要直接使用编译器，你可以执行  `rustc src/hello.rs`  , 将会创建一个名为 hello 的可执行文件。`rustc --help` 可以查看更多的编译选项。

现在回到这段代码中，我们可以看到一些有趣的点：

- 我们使用 `fn` 来定义一个 函数/方法。
- `main（）` 是我们程序的默认入口点（稍后我们会给它添加参数）
-  与C++相比，这里没有单独的声明或头文件
- `println!` 是 Rust 中的 `printf` ; 
  -  !表示这是一个宏
  - 标准库中有一个子集是直接可用，无需显式地 import/include。
  - `println!` 宏 是这个子集的一部分 

可以简单的修改一下我们的例子:

```rust
fn main() {
    let world = "world";
    println!("Hello {}!", world);
}
```

`let` 用于声明一个变量， world 是变量名，这个变量是一个String ( 更具体地，这个变量的类型是 `&'static str'`, 这个留到后面再深入)。我们不需要指定变量的类型，它会自动推导。

在`println!` 语句里 使用 `{}`， 相当于`printf`里的 `%s` 。事实上，它更加通用，因为Rust会尝试将变量转换为字符串，如果它还不是字符串的话<sup>[1](#1)</sup>（更像是C ++中的 `<< `操作符 ）

当然，你也可以显式地指定变量类型：

```rust
let world: &'static str = "world";
```

在C++，我们用 `T x` 来指定变量x的类型为T。在Rust ，我们写成 `x: T`， 不管是在 `let` 语句，还是在函数定义中。 大多数情况下，我们在`let`语句中使用显式类型；当然，还是参数也需要指定函数类型。现在我们添加一个带参数的函数：

```rust
fn foo(_x: &'static str) -> &'static str {
    "world"
}

fn main() {
    println!("Hello {}!", foo("bar"));
}
```

函数 `foo` 有一个参数 `_x` ，它是一个 字符串字面量（我们会从main传递这个参数）<sup>[2](#2)</sup>。

函数返回值的类型在 `->` 后面指定。如果函数没有返回（就像C++的 void函数），我们就不需要指定返回类型，比如这里的 `main`函数。 如果你想要足够清晰的表达函数没有返回值，你可以写成 `-> () ` ，`()` 是 Rust的 void 类型。

在Rust 里，你 不需要 return 关键字，如果在函数体中的最后一个表达式（或任何其他块，我们将在后面看到更多内容）不以分号结尾，则你在Rust中不需要写`return`关键字。因此，`foo`将返回“ world”。 return关键字仍然存在，保证我们可以提早返回。您可以将“ world”替换为“ return“ world”;`，效果是相同的。


## Why?

I would like to motivate some of the language features above. Local type
inference is convenient and useful without sacrificing safety or performance
(it's even in modern versions of C++ now). A minor convenience is that language
items are consistently denoted by keyword (`fn`, `let`, etc.), this makes
scanning by eye or by tools easier, in general the syntax of Rust is simpler and
more consistent than C++. The `println!` macro is safer than printf - the number
of arguments is statically checked against the number of 'holes' in the string
and the arguments are type checked. This means you can't make the printf
mistakes of printing memory as if it had a different type or addressing memory
further down the stack by mistake. These are fairly minor things, but I hope
they illustrate the philosophy behind the design of Rust.

首先我想夸一下上面提到的某些语言特性：

- 本地类型推断在不牺牲安全性或性能的情况下既方便又实用（甚至现在在C ++的现代版本中）。

- 另一个小的便利是**语言项始终用关键字（fn，let等）表示，这使得用眼睛或工具进行扫描变得更容易**，通常，Rust的语法比C ++更简单且更一致。 

- **println！宏比printf 更安全**：相对于字符串中的“空洞”数量，将静态检查参数的数量，并对参数进行类型检查。这意味着您不能犯错误的打印错误，就好像打印内存有不同的类型一样，或者将内存寻址到堆栈的更下方。

当然，这些倒还其次，我更希望的是，能用这些小例子来表现  Rust设计背后的哲学。

## 注释

##### 1

根据程序员指定的方式转换，它使用`Display`特性，其工作方式类似于Java中的toString。您也可以使用{：？}，它给出了编译器生成的表示形式，该表示形式有时对于调试很有用。与 `printf` 一样，`println！`还有许多其他选项。

##### 2

我们实际上不在foo中使用该参数。通常，Rust会就此警告我们。通过在参数名称前面加上 `_` ，可以避免出现这些警告。实际上，我们根本不需要命名参数，我们可以使用 `_` 。

