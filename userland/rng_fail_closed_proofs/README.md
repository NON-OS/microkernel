# rng_fail_closed_proofs

Host proofs that the kernel's randomness callers fail closed, and that the
boot line naming a draw made before seeding is right. There are 20 tests.

**What is mounted.** The kernel's own files, by `#[path]` (`src/mount.rs`):

- `src/elf/aslr/manager.rs` and its five files;
- `src/capabilities/token/nonce.rs`;
- `src/crypto/random_api/{basic,entropy_check,hardware_mix}.rs`;
- `src/crypto/util/misc.rs`, `src/crypto/util/rng/{api,error}.rs`;
- `src/crypto/util/rng/global/{fallback,generate,range}.rs`;
- `src/crypto/core/api/random/api.rs`;
- `src/crypto/error.rs`.

Only these stand in (`src/stand_in*.rs`): the seeded RNG (its state, its
draw and whether it is seeded), the hardware entropy source, the CPU's
random instruction, the time counter and the serial line. "Not seeded"
stands for what the kernel's `ensure_initialized` returns when `init_rng`
cannot seed. The kernel's `init.rs` itself, which seeds on first use when
the hardware answers, is not mounted. Each source can
be turned off for a thread, and each working source tags its values, so a
test sees which one answered.

**What is tested.**

- ASLR takes the CPU's instruction, then the seeded RNG. When neither
  answers, every placement is refused and `reseed` says so; nothing is made
  from a constant. A slide that would carry a stack below zero, or a base
  or heap past the top, is refused, not wrapped.
  Turned-off randomisation needs no draw.
- A token's 128-bit nonce is refused before seeding, never zeros. The
  64-bit nonce draws when it can and counts when it cannot. The counter has
  no reset, so it does not repeat within a boot.
- `random_api::get_bytes_secure` and the `misc` helpers refuse when the RNG
  is not seeded, whatever the hardware answers, and write nothing.
- `[RNG] pre-seed draw by <file>:<line>` names the caller's own source
  line through every layer: the byte API, the RNG's `fill_bytes`,
  `get_random_bytes`, `random_u64` and `random_range`. It is
  said once, never for a seeded draw, and as one write. A path too long is
  cut short and the line number kept. With no hardware the weak line
  follows it, once.

- The core API's `fill_random` and `secure_random_u64/u32/u8` refuse before
  seeding, whatever the hardware answers, write nothing and never reach the
  fallback, so neither boot line is said; once seeded they answer from it.
- `random_api::get_vec_secure` refuses before seeding, and draws from the
  seeded RNG after.

Each pre-seed test is its own file under `tests/`, because the line is said
once a process.

**Reviewed, not proved.** The ELF loader's refusal (`base_addr.rs` and its
two callers) and `security/policy/capability/api.rs` are not mounted: they
reach most of the kernel. Each is a `?` on a refusal shown above.

**Mutants.** 36 mutants of the kernel's sources, applied one at a time, were
all caught. They include the ASLR constant seed restored, the source order
swapped, each placement ignoring a failed draw, `reseed` claiming success,
each slide wrapping, the 128-bit nonce taking zeros, the 64-bit nonce
repeating, the secure byte call falling back, the `misc` helpers falling
back to the clock, and each layer's `#[track_caller]` removed.
