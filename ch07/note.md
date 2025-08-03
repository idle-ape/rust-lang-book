#### Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容可以作为私有部分，以及程序每个作用域中的名字。这些功能有时被称为“模块系统”，包括：
- 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
    1. 包(package) 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。
    2. 一个包中至多只能包含一个库 crate(library crate) 
    3. 包中可以包含任意多个二进制 crate(binary crate)
    4. 包中至少包含一个 crate，无论是库的还是二进制的
    通过 cargo new my-project 来创建包，如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。
    通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
- Crates：一个模块的树形结构，它形成了库或二进制项目
    crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块
- 模块（Modules）和 use：允许你控制作用域和路径的私有性
- 路径：一个命名例如结构体、函数或者模块等项的方式

#### 定义模块来控制作用域和私有性
    允许你命名项的路径 (paths)，用来将路径引入作用域的 use 关键字;
    使项变为公有的 pub 关键字。我们还将讨论 as 关键字、外部包和 glob 运算符。
    
    通过 cargo new --lib restaurant ，来创建一个新的名为 restaurant 的库