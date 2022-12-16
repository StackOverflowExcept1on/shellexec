### shellexec

[![Build Status](https://github.com/StackOverflowExcept1on/shellexec/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/shellexec/actions)

Cross-platform shellcode executor in rwx memory

```
Usage: shellexec <binary>

Executes shellcode

Positional Arguments:
  binary            path to binary file

Options:
  --help            display usage information
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
