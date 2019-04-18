# brainf

An interpreter for brainfuck programming language.

# Compiling

You will need rust to compile this project. Run following command to
compile the program in debug mode.

```
cargo build
```

Or if you wanna build the program in release mode, use this command,

```
cargo build --release
```

# Usage

The binary will be compiled into `target/debug/brainf` or `target/release/brainf`.
To run a brainfuck program, provide it as the argument to that binary. For example:

```
./target/release/brainf hello.bf
```

# LICENSE

MIT LICENSE, see [MIT-LICENSE.txt](MIT-LICENSE.txt)
