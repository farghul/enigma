# Enigma

Create and modify Certificate Signing Requests (CSR) for your web deployment needs.

## Prerequisites

Rust language installed to allow building from source.

## Build

Before building the application, change the value of the `PLACE` constant to reflect your environment:

```rust
const PLACE: &'static str = "enigma/key/";
```

From the root project folder run:

```zsh
cargo build --release
   Compiling libc v0.2.150
   Compiling rustix v0.38.25
   Compiling bitflags v2.4.1
   Compiling lazy_static v1.4.0
   Compiling errno v0.3.8
   Compiling is-terminal v0.4.9
   Compiling colored v2.0.4
   Compiling enigma v0.1.0
    Finished `release` profile [optimized] target(s) in 5.11s
```

## Run

From the _target/release_ folder run:

```zsh
./enigma [flag] [cert name] [mmddyyyy]
```

## Flags

```zsh
h, help      Help information
k, key       Create CSR with a new key
n, nokey     Create CSR with an existing key
```

## License

Code is distributed under [The Unlicense](https://github.com/ekiuts/enigma/blob/main/LICENSE.md) and is part of the Public Domain.
