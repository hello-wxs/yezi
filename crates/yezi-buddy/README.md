<div align="center">

# yezi-buddy

[![Version](https://img.shields.io/crates/v/yezi-buddy)](https://crates.io/crates/yezi-buddy)
[![Downloads](https://img.shields.io/crates/d/yezi-buddy)](https://crates.io/crates/yezi-buddy)
[![License](https://img.shields.io/crates/l/yezi-buddy)](https://codeberg.org/hello_wxs/yezi/src/branch/main/LICENSE.md)
[![Author](https://img.shields.io/badge/author-hello_wxs-white.svg)](https://codeberg.org/hello_wxs)

The `yezi-buddy` crate provides a buddy that can generate random states.
You can create a buddy with a custom name and buddy_name.

</div>

## Examples

You can both provide a user name and a buddy name.

```
use yezi_buddy::Buddy;
let mut buddy = Buddy::new("hello_wxs".to_string(), Some("Biu".to_string()), 8);
```

If you don't provide a buddy name, the type name will be used as the buddy name.

```
use yezi_buddy::Buddy;
let mut buddy = Buddy::new("hello_wxs".to_string(), None, 8);
```
