# asid_proofs

Host tests for the aarch64 hardware ASID pool, mounted by `#[path]` from
`src/arch/aarch64/mmu/asid/pool.rs`: every live user root gets its own ASID,
never 0 (the kernel's) or the top one (shared), and keeps it across switches;
a retired root's ASID is handed out again; once all are taken, roots share
the top ASID. Checked over every ASID of an 8-bit and a 16-bit core.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
