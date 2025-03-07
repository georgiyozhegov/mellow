# Mellow

**_Mellow_** is a hobby-compiler written in Rust that designed to be simple, intuitive and fast. The language is inspired by Rust, Lua and Python.

Note that this is a **work-in-progress** project, so there might be some bugs in the code.

# Getting Started

## 0. Requirements

Ensure you have the following installed on your system.

- Cargo
- GCC
- Nasm

## 1. Clone

```sh
git clone https://github.com/georgiyozhegov/mellow.git
```

Then, go to the `mellow` directory.

```sh
cd mellow
```

## 2. Build & Run

And run the compiler.

```sh
make run
```

First, it will compile the **_sl_** (standard library) and then, it will link the assembly object file with it. The resulting binary is able to run on any x86-64 machine.
