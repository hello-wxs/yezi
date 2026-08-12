<div align="center">

# Yezi
**极速终端词条学习 TUI 工具**

[![版本](https://img.shields.io/crates/v/yezi)](https://crates.io/crates/yezi)
[![下载量](https://img.shields.io/crates/d/yezi)](https://crates.io/crates/yezi)
[![许可证](https://img.shields.io/crates/l/yezi)](https://codeberg.org/hello_wxs/yezi/src/branch/main/LICENSE.md)
[![作者](https://img.shields.io/badge/author-hello_wxs-white.svg)](https://codeberg.org/hello_wxs)

[en](https://codeberg.org/hello_wxs/yezi/src/branch/main/README.md) | [简体中文](https://codeberg.org/hello_wxs/yezi/src/branch/main/README-zh_cn.md)
</div>

Yezi 是一款**使用 Rust 编写的终端条目管理器**，基于异步 I/O 构建。它旨在提供一个**高效、用户友好且极速**的条目管理体验。它具有 **Vim 风格**的输入/选择/确认组件。

# 文档
文档在[此](docs/zh_cn)。如果您想查看渲染后的文档，可以使用 mdbook 在本地构建：
```shell
cd docs/zh_cn
mdbook build
```
另外，如果你有 `cargo-mdbook`（我的另一个项目），你可以使用它更方便地构建文档： **:)**
```shell
cargo mdbook build
```

## 许可证
本项目采用 **GNU General Public License v3.0 或更高版本** 许可 - 详情请参阅 **[LICENSE.md](LICENSE.md)**。
