### Cargo
    Cargo 是 Rust 的构建系统和包管理器，它可以自动下载依赖项并构建项目。
### 使用 Cargo 创建项目
    $ cargo new hello_cargo
    $ cd hello_cargo
    $ ll
    total 8
    drwxr-xr-x 1 root root  58 May 30 06:15 ./
    drwxr-xr-x 1 root root  86 May 30 06:23 ../
    -rwxr-xr-x 1 root root  82 May 30 06:09 Cargo.toml*
    drwxr-xr-x 1 root root  14 May 30 06:09 src/
### 使用 Cargo 编译项目
    $ cargo build
    $ ll
    total 8
    drwxr-xr-x 1 root root  58 May 30 06:15 ./
    drwxr-xr-x 1 root root  86 May 30 06:23 ../
    -rwxr-xr-x 1 root root 155 May 30 06:15 Cargo.lock*
    -rwxr-xr-x 1 root root  82 May 30 06:09 Cargo.toml*
    drwxr-xr-x 1 root root  14 May 30 06:09 src/
    drwxr-xr-x 1 root root  66 May 30 06:15 target/

    编译出来的二进制位于: target/debug/hello_cargo

### 使用 Cargo 编译并运行项目
    $ cargo run

### 使用 Cargo 发布构建
    $ cargo build --release
    当项目最终准备好发布时，可以使用 cargo build --release 来优化编译项目。这会在 target/release 而不是 target/debug 下生成可执行文件。