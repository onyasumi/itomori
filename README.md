<div align="center">

# 糸守 itomori

[description coming soon]

[![GPLv2](https://img.shields.io/badge/license-GPLv2-green)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html#SEC1)

</div>

## Compiling

```sh
rustup target add aarch64-unknown-none
```

then

```sh
cargo rustc --target aarch64-unknown-none -- -C link-arg=--script=./linker.ld
```