# Mellow

**_Mellow_** is a hobby-compiler written in Rust that designed to be simple, intuitive and fast. The language is inspired by Rust, Lua and Python.

Note that this is a **work-in-progress** project, so there might be some bugs in the code.

# How to run?

## 0. Requirements

- Cargo
- GCC
- Nasm

Install them, if you haven't already.

## 1. Clone

```sh
git clone https://github.com/georgiyozhegov/mellow.git
```

Then, go to the `mellow` directory.

```sh
cd mellow
```

## 2. Compile & execute

And run the compiler.

```sh
make run
```

First, it will compile the **_sl_** (standard library) and then, it will link the assembly object file with it.
