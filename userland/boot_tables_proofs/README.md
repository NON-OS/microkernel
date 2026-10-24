# boot_tables_proofs

Host tests for the aarch64 boot page tables. The builder, the level 2 table
pool, the RAM, direct and register maps, the image's page tables, the walker
and `map_page` / `unmap_page` are the kernel's own files, mounted by
`#[path]`. Only the boot info struct, the console and the TLB flush are
stand-ins.

Each test is its own binary, since the tables are statics built once:

- `pi4_8gb`: three RAM regions over eight gigabytes, the second's top sharing
  a gigabyte with the peripherals, a GICv2 with no redistributors. Every
  2 MiB of RAM is identity and direct mapped as RAM, the image's blocks as
  pages with text alone executable, the registers as Device, and nothing
  at physical 0 is taken for a register window.
- `pool_spent`: 64 GiB, more gigabytes than tables. The direct map stops where
  the pool does and reports it, for the allocator to stop there too.
- `map_page`: a page lands in its level 3 table; one inside a 2 MiB block,
  past the level 3 span or outside level 0's first entry is refused.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
