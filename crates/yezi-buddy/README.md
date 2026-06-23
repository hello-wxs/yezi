<div align="center">

# yezi-buddy

[![LICENSE: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL%20v3%20or%20later-blue.svg)](LICENSE.md)
[![VERSION: 0.1.0](https://img.shields.io/badge/version-0.1.0-fedcba.svg)]()
[![AUTHOR: hello_wxs](https://img.shields.io/badge/author-hello_wxs-white.svg)]()

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
