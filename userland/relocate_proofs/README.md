# relocate_proofs

Host tests for the aarch64 kernel's self-relocation
(`src/arch/aarch64/boot/relocate/{rela,apply}.rs`, mounted by `#[path]`).

`fixtures/pie.elf` is a static PIE that ld.lld links from `fixtures/pie.s` with
the kernel's flags (`-pie --no-dynamic-linker -z text`) and the kernel's layout:
linked at 0x4008_0000, writable data on its own 2 MiB block. `fixtures/regen.sh`
rebuilds it. Its data holds pointers to code, to read-only data, into an object,
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
