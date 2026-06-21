# 构建

在构建之前，请确保您已安装 Rust。
如果您尚未安装，可以参考 [Rust 官方 安装指南](https://www.rust-lang.org/tools/install) 进行安装。

## 构建项目

安装 Rust 后，您可以使用以下命令构建项目：

```shell
cargo build
```


## 构建文档

文档是使用 [mdBook](https://rust-lang.github.io/mdBook/) 构建的，它也是由 Rust 编写的。

如果您尚未安装，可以使用以下命令安装：

```shell
cargo install mdbook
```

安装 mdBook 后，您可以使用以下命令构建文档：

```shell
cd docs/en
mdbook build
```
或
```shell
cd docs/zh_cn
mdbook build
```
