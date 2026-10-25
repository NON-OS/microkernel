# relocate_proofs

Host tests for the aarch64 kernel's self-relocation
(`src/arch/aarch64/boot/relocate/{rela,apply}.rs`, mounted by `#[path]`).

`fixtures/pie.elf` is a static PIE that ld.lld links from `fixtures/pie.s` with
the kernel's flags (`-pie --no-dynamic-linker -z text`) and the kernel's layout:
linked at 0x4008_0000, writable data on its own 2 MiB block. Its sha256 is
recorded in `fixtures/pie.elf.sha256`:

    4368e879d381b1ec92d4ce4289724c391c51d881a0ce09b0aa3a112b76f6b405  pie.elf

`fixtures/regen.sh` rebuilds it in a scratch directory, prints the result's
sha256 and fails unless it equals the recorded one, so anyone can show the
checked-in binary is exactly what `pie.s` and `pie.ld` produce. Built and
checked with llvm-mc from LLVM 18.1.3 and the toolchain's rust-lld (LLD
21.1.8); `--update` takes a new build, for when either source changes. Its data holds pointers to code, to read-only data, into an object,
to itself and past an object's end, and one plain number.

- The PIE is loaded as the arm64 boot protocol places an Image, at two
  different 2 MiB aligned bases plus text_offset, and relocated at each: every
  pointer is its target's link address moved by that image's bias, and every
  other byte is the same in both.
- The pointers read through reach the bytes they name; before relocation the
  places are empty, as ld.lld leaves them.
- A relocation that is not R_AARCH64_RELATIVE, lies outside the image or is
  misaligned refuses the whole image with nothing written; a bias that is not a
  multiple of 2 MiB is refused.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
