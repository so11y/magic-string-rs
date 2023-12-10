# magic-string-rs

magic-string-rs 是一个 Rust 实现的用于处理字符串的工具库，受 JavaScript MagicString 库启发而创建。

## 简介

MagicString 是一个工具，用于在不修改原始字符串的情况下执行字符串操作。它允许添加、删除和修改字符串内容，而不会直接修改原始字符串。本库旨在提供类似 JavaScript MagicString 库的功能，并通过 Rust 语言进行实现。




## 📃 文档

详细的方法说明和示例请查阅 [MagicString 文档](https://github.com/Rich-Harris/magic-string)。


## 支持的 API

- [x] toString
- [x] prepend
- [x] append
- [x] appendLeft
- [x] appendRight
- [x] overwrite
- [x] remove
- [ ] trimLines
- [ ] prependLeft
- [ ] prependRight
- [ ] trim
- [ ] trimStart
- [ ] trimEnd
- [ ] isEmpty
- [ ] generateDecodedMap
- [ ] move
- [ ] indent
- [ ] addSourcemapLocation
- [ ] clone
- [ ] slice
- [ ] snip