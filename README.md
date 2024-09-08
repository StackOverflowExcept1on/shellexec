### shellexec

[![Build Status](https://github.com/StackOverflowExcept1on/shellexec/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/shellexec/actions)
[![Latest Version](https://img.shields.io/crates/v/shellexec.svg)](https://crates.io/crates/shellexec)

Cross-platform shellcode executor in rwx memory

```
Usage: shellexec <binary>

Executes shellcode

Positional Arguments:
  binary            path to binary file

Options:
  --help            display usage information
```

### Implementation

[virtual-memory](https://github.com/StackOverflowExcept1on/shellexec/blob/master/virtual-memory/src/lib.rs) is used to execute shellcode.
This helper crates allocates rwx memory using the system functions that is described here:

- Unix
    - [`mmap(NULL, len, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANON, -1, 0)`](https://man7.org/linux/man-pages/man2/mmap.2.html)
    - [`munmap(addr, len)`](https://man7.org/linux/man-pages/man2/munmap.2.html)

- Windows
    - [`VirtualAlloc(NULL, len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE)`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc)
    - [`VirtualFree(addr, 0, MEM_RELEASE)`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree)

Then the contents of the file are copied into this memory,
the address of this memory is converted into a function and called

### Installing from [crates.io](https://crates.io/crates/shellexec)

```
cargo install shellexec
```

### Building

```bash
cargo build --release
```

### Usage

```bash
# linux
cargo run --release -- shellexec/test_input/linux

# windows
cargo run --release -- shellexec/test_input/windows

# output
hello from shellcode!
```
