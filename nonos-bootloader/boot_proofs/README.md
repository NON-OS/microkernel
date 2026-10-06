# boot_proofs

Host-runnable proofs for the bootloader's security-critical logic. The real
rollback floor code and the real `image_format` footer parser are included
through `#[path]` and run on the host, so the invariants are proved about the
code that gates a kernel boot.

## Rollback floor

The floor is a TPM monotonic counter at NV index 0x01000020; it stops an
attacker booting an older signed kernel with a known vulnerability. The real
`security::tpm_nv` read and raise sequences run against a TPM scripted to the
specification (`scripted_tpm.rs`). The tests establish:

- A new TPM starts the floor at 1; a held counter is read and not incremented.
- A counter the owner undefines reads above its old floor on the next boot,
  never 0 (REVIEW R20), because an uninitialized counter is incremented first.
- Any other read answer, or an increment that fails, is no floor.
- Raising reaches the target and never lowers; a failed increment is reported.
- The read is a value only in the shape of one, byte by byte, and the three
  commands name the rollback counter.
- Without a readable counter Hardened and Air-Gapped refuse; every other
  profile boots and says so (`floor_rule`).

Kani harnesses hold the read's mapping total for every answer and length. The
same files run against swtpm in `userland/tpm_enroll_proofs`.

## Image footer parser

The footer names the byte ranges of the kernel, signature, and proof regions and
is attacker-controlled. Over roughly 125,000 crafted footers with adversarial
region offsets and sizes, and a range of degenerate inputs, the parser never
panics and never returns a region slice that escapes the input buffer. A Kani
harness proves the parse is total for every input of the checked size.

## Run

```sh
cd nonos-bootloader/boot_proofs
cargo test --release
cargo kani                # all-input totality (requires Kani)
```
