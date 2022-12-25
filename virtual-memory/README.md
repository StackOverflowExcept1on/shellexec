### virtual-memory

[![Build Status](https://github.com/StackOverflowExcept1on/shellexec/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/shellexec/actions)
[![Latest Version](https://img.shields.io/crates/v/virtual-memory.svg)](https://crates.io/crates/virtual-memory)
[![Documentation](https://docs.rs/virtual-memory/badge.svg)](https://docs.rs/virtual-memory/)

Library for allocating RWX memory on Unix and Windows.

Also see https://github.com/StackOverflowExcept1on/shellexec for more details

### Example

```rust
use virtual_memory::*;

let buf = &[
    //mov eax, 1337
    0xb8, 0x39, 0x05, 0x00, 0x00,
    //ret
    0xc3,
];

let mut memory = VirtualMemory::new(buf.len()).expect("failed to allocate rwx memory");
memory.copy_from_slice(buf);

let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(memory.as_ptr()) };
assert_eq!(f(), 1337);
```
