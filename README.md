# Native Extensions with Rust

Recently I've been through a lot of interview processes and in one of them the challenge was to
implement a simple feature (image conversion from PNG to JPG) in two different languages and write
about it.

One of the languages had to be JS and while researching libraries to do that in JS most libraries
were a mess or barely maintained. While I eventually found a nice one
([sharp](https://github.com/lovell/sharp)) it got me thinking: what if I could use things outside of
the JS ecossystem?

From my time doing Elixir, a common thing to do would be to write a NIF (Native Implemented
Function) and most of the time the community opted to use Rust for that using a library called
[Rustler](https://github.com/rusterlium/rustler). So I wanted to do something similar in JS and
other languages.

This is then an experiment with multiple tools for writing native extensions targetting different
runtimes using Rust. But first, let's get to the basics.

## C ABI as the Lingua Franca

ABI stands for Application Binary Interface and you can think of it as the "protocol" to which data
is accessed and represented in memory and how functions are called. This is higly specific to the
compiler and the target platform.

While barely a well defined standard, C compilers do some effort to make sure the ABI is stable,
which means if you get a module that was compiled a long time ago and link it to a new program
compiled with a recent compiler, it should still work.

Since C ended up being the main language used for low level programming, it's ABI became the "lingua
franca", meaning that even other languages (C++, D, Rust, etc) can be called from C and call C
functions, that is, have a way to adhere to the C ABI.

This is also how runtimes like Node.js, Ruby MRI, CPython, Erlang VM and others have a way to call
C-ABI-compatible functions. The name of that will vary and has a lot of nuance of what it actually
means, but you will hear terms like:
- Native Extension
- Foreign Function Interface (FFI)
- Native Implemented Functions (NIF) for the BEAM (Erlang VM)
- Java Native Interface (JNI) for the JVM
- Node Addons for Node.js

## Why Rust?

While the most straightforward choice would be C or C++ since most of these runtimes even provide a
C header to interface with you don't need to limit yourself to that.

Pretty much any language that can generate a C-ABI-compatible dynamically linked library can be
used, the list also include some new languages like Zig and Rust.

Rust has been gaining a lot of traction in that space recently due to it's memory safety features
and great type system and ergonomics. However, there's still a challenge in compiling and linking
the native extension and also having idiomatic bindings.

## Using Rust

While you could use [`rust-bindgen`](https://github.com/rust-lang/rust-bindgen) to generate the Rust
bindings from the runtime provided C headers, it might become a challenge once you want to package
that and distribute (for example, how do you find the appropriate header file for the current
running runtime version, how/when you compile, how to provide pre-compiled binaries, etc).

This is where some projects come in place to make the developer experience better. In some cases
(like Ruby MRI) this is even being integrated into the "standard tooling" (Bundle can now generate a
native gem project using Rust, as described
[here](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html)).

## The Tools

The goal of this experiment was to play a little bit with different tools for writing extensions in
Rust for different runtimes. So here is a list of the tools we'll be looking at:

| Runtime | Tool                                                                                                                  |
|---------|-----------------------------------------------------------------------------------------------------------------------|
| Node.js | [Neon](https://neon-bindings.com/)                                                                                    |
| Ruby    | [Bundler](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html) / [rb_sys](https://oxidize-rb.github.io/rb-sys/) |
| Python  | [PyO3](https://pyo3.rs/) / [maturin](https://www.maturin.rs/)                                                         |
| Elixir  | [Rustler](https://github.com/rusterlium/rustler)                                                                      |

These are some other tools that could be used for other runtimes:
- PHP: [ext-php-rs](https://github.com/davidcole1340/ext-php-rs) or
  [PHPER](https://github.com/phper-framework/phper?tab=readme-ov-file)
- JVM: [jni-rs](https://github.com/jni-rs/jni-rs)


### Multi-target generation tools

Finally, there are some tools that I definitely will look into in the future, but not right now:
tools that generate bindings for multiple runtimes from a single Rust codebase.

While each of the other tools have their own bindings for each language, these tools here provide a
way to define an interface and automatically generate the bindings for each runtime.

One is [UniFFI](https://github.com/mozilla/uniffi-rs) which works with a custom interface
description language (UDL) and use that to both generate the Rust scaffolding and the bindings for
each runtime. In a way this is similar to the idea of gRPC and Protobufs, without the network part.

The other is [diplomat](https://github.com/rust-diplomat/diplomat/) which the main difference is
that the interface is described in Rust itself using macros.

### Note on WASM

Another strategy for interop with Rust would be to compile WASM modules and then using a WASM
runtime (like Wasmtime) in the target runtime. Funny enough, in the case of Elixir the Wasmtime is
implemented as a NIF using Rustler, so you would be running a Rust-coded WASM module through a
Rust-coded NIF in Elixir: recursive feelings.

Maybe you could compile Wasmtime to a WASM module and go as deep as you wish.