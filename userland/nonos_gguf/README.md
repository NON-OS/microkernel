# nonos_gguf

A strict reader of GGUF model headers, the format the local Qwen models ship
in. A model file is hostile input until it is checked: its counts, string
lengths and tensor shapes size everything an engine allocates. `parse` reads
the header, the metadata keys and the tensor table through a `ReadAt` source,
bounds every one of them against `Limits`, and refuses a file that breaks a
bound with the field and the value that broke it. It returns a `Summary` of
the metadata and tensors.

`DEFAULT` allows 16,384 tensors and 16,384 keys, strings up to 1 MiB, arrays
up to 2^24 elements, a dimension up to 2^31 and a tensor up to 16 GiB.

It is a library with no capsule, service or capability word, and nothing in
the tree depends on it: `qwenchat`, the Linux guest that runs the models,
reads them with llama.cpp's own loader. The local model is described in
[docs/handbook/apps/local-ai.md](../../docs/handbook/apps/local-ai.md).

## Tests and tools

- 12 host tests under `src/tests/`.
- `examples/check.rs` prints what a GGUF file's header says:
  `cargo run --release --example check -- model.gguf`.
- `fuzz/fuzz_targets/parse.rs` is a cargo-fuzz target for `parse`.

```sh
cd userland/nonos_gguf
cargo test --release
```
