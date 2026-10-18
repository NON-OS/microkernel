# epd1_proofs

Host tests for the aarch64 EPD1 trampoline (`src/arch/aarch64/security/epd1/`,
`src/arch/aarch64/mmu/trampoline/`, `src/arch/aarch64/asm/epd1*.{S,h}`). The
pure Rust is mounted by `#[path]`; the assembly and the layout constants are
read with `include_str!`.

- Classification: the speculation decision calls a CPU exposed exactly when
  it is a Cortex-A75 claiming neither CSV3 nor E0PD, over every Arm part and
  ten implementers; on every mix of up to four of five core kinds, exactly
  the exposed cores turn the trampoline on, and user roots carry it exactly
  when one does.
- Order: every sequence of up to eight build, exposed-CPU and new-root
  events. No CPU uses the trampoline while some user root lacks it, an
  exposed CPU in time is never refused, nothing switches before the tables.
- Footprint: a seeded user root, walked level by level, maps exactly two
  kernel pages, the code at EL1 only and read-only, the data read-only and
  never executable, every table on the way closed to EL0.
- Layout: the slot lies above user space, inside TTBR0's 48 bits, at the
  address the vectors test (and that address is an encodable `eor`
  immediate), and no kernel region in TTBR1 starts in or reaches into it.
- Assembly: EPD1 cleared before the first load on every entry from EL0, set
  before the TLB is emptied on the way out with no load in between, the
  targets in the vector table's order, every return to EL0 through the test.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory.
