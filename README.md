# Megaton Hammer

**This is still a work in progress. Do not expect something full-featured,
stable or easy to use just yet. APIs will break in the blink of an eye.**

Megaton Hammer is a Rust toolchain to build all sorts of homebrew for the
Nintendo Switch. It is Homebrew ABI compliant, and as such can be loaded by
Ace Loader or the HBL without issues.

## Documentation

**TODO**: Host the documentation somewhere stable. Right now, if you're lucky,
you might get API docs at https://docs.roblab.la/megaton-hammer/megaton_hammer.

## Building

The easiest way to get started is through [rustup], as you'll need a very recent
nightly:

```
curl https://sh.rustup.rs -sSf | sh
rustup toolchain add nightly
rustup default nightly
```

You'll also want to use [xargo], a cargo wrapper that will automatically compile
libcore for esoteric targets.

```
cargo install xargo
```

Cloning this repo and building megaton-example will yield a binary:

```
git clone https://github.com/roblabla/megaton-hammer
cd megaton-hammer
env RUST_TARGET_PATH=$PWD xargo build --target aarch64-roblabla-switch -p megaton-example
```

The generated output will be an ELF. In order to run it on the switch, you'll
need to turn it into an NRO. You can use the elf2nxo python script for this.
First, install its dependencies, then run it:

```
pip install --user lz4 pyelftools
python target/aarch64-roblabla-switch/debug/megaton-example target/aarch64-roblabla-switch/debug/megaton-example.nro
```

[rustup]: https://rustup.rs
