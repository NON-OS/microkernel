# nonos-proof-verify test fixture

Real enrollments made by `nonos-stark-enroll` 0.2.0, with real STARK proofs
from the STARK project at the revision the enroll tool's `Cargo.lock` pins. Nothing
here is a release: the roots are development roots, and the verifier says so.

| Files | What |
|---|---|
| `kernel.image`, `kernel_root.bin`(`.transcript`), `kernel.trailer` | one kernel slot: `nonos-stark-enroll kernel kernel.image kernel_root.bin kernel.trailer` |
| `a.elf`, `b.elf`, `caps_root.bin`(`.transcript`), `a.trailer`, `b.trailer` | two capsule slots, capabilities 0x18 and 0x59: `nonos-stark-enroll capsules caps_root.bin 0x18:a.elf:a.trailer 0x59:b.elf:b.trailer` |
| `dev_root.bin`(`.transcript`), `a.dev`, `b.dev` | the same two capsules enrolled as a development image is, paths only: `NONOS_ENROLL_PATHS=1 nonos-stark-enroll capsules dev_root.bin 0x18:a.elf:a.dev 0x59:b.elf:b.dev` |

The images are short text files: a gate measures bytes, not ELF structure,
and the contexts are built from their BLAKE3 alone. The bootloader case uses
`nonos-boot-measure/src/tests/fixture` (a PE, its root and a v4 trailer).

Regenerating draws a new pad seed, so every root and trailer changes; the
tests read the files and hold no root of their own. A proof takes about a
minute and a half on eight cores.
