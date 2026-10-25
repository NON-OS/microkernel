# K1: `NONOSATT` version 5, a quote that names the NONOS chain

Status: design, for the lead's review. Nothing lands before the 0.9.3 seal.
The kernel has since used version 4 for the count of devices the IOMMU
still lets reach all of memory, so K1's document is version 5. The file
keeps its name so links to it hold.

## The gap today

`MkAttestDoc` (`src/security/attest_doc/produce.rs`) quotes these PCRs:

```
QUOTED_PCRS = [0, 1, 2, 7]
```

Those are firmware, firmware configuration, option ROMs and Secure Boot
state. The PCRs NONOS itself extends are left out:

| PCR | What extends it | Source |
|---|---|---|
| 4 | the boot manager and the loader, measured by firmware (Authenticode) | firmware |
| 8 | the loader's own measurement | `nonos-bootloader/src/security/tpm_extend/components.rs`, `check/tpm.rs` |
| 9 | the admitted kernel: `SHA-256(kernel_blake3 \|\| kernel_attest_root)` | `boot/attestation/kernel_gate.rs` |
| 14 | the capsule and ZK-proof measurements | `security/enforce/checks/measurements.rs`, `tpm_extend/components.rs` |

The result is that a version 3 or 4 document signed by a genuine TPM says which
firmware booted, but not which loader, kernel or proof. A machine running
any kernel produces an equally valid version 4 document.

The registry root inside it describes the capsules the running kernel says
it started. That claim comes from the kernel's own memory. A changed kernel
could report the right registry.

## Version 5

The document keeps its version 4 layout (big-endian, length-prefixed) and
adds two fields after `identity_devices`.

| Field | Size |
|---|---|
| `"NONOSATT"`, version u32 = 5 | 12 |
| challenge, registry_root, capsule_count, registry_complete, iommu_vendor, iommu_enforcing, unconfined_grants, identity_devices | as in version 4 |
| **pcr_count** (u8), then each **index** (u8) and its **SHA-256 value** (32) | 1 + 33 × n |
| **TCG event log**, as the firmware handed it over (`MODULE_KIND_TCG_LOG`) | u32 length, then the bytes; at most 1048576, the loader's own cap |
| attest, signature, ak_public | as in version 4 |

- `QUOTED_PCRS_V4 = [0, 1, 2, 4, 7, 8, 9, 14]`. The quote's selection must
  equal this list, and the values must be in this order.
- **The binding.** The qualifying data stays `nonos.attest.bind.v3`, which
  already folds in `identity_devices`. The
  PCR selection and its digest are inside `TPMS_ATTEST`, which the TPM
  signs, so the binding does not need to change. Leaving it alone keeps
  the change small.
- **The kernel checks itself before it answers.** It works in this order:
  1. It reads the eight values with `TPM2_PCR_Read`. That call is repeated
     until every value has come back, because a TPM may return fewer
     digests than were asked for.
  2. It quotes.
  3. It recomputes `SHA-256(value 0 || value 1 || …)` and compares it with
     the quote's `pcrDigest`.

  A mismatch means a PCR moved between the read and the quote. The kernel
  then reads and quotes once more, and refuses if the second attempt also
  disagrees. It never hands out values that the quote does not sign.
- **The event log.** It lets a verifier replay PCR 4 and see the loader's
  Authenticode digest in it, because PCR 4 also holds firmware events that
  differ between machines. The kernel already replays this log for its own
  loader check (`src/security/boot/loader_check`). A log over the cap is
  refused, not cut short.

## What a verifier checks (nsh-v1-attest.md, step 5, made exact)

1. The quote's selection is exactly `QUOTED_PCRS_V4`, in the SHA-256 bank.
2. The digest of the document's values equals the quote's `pcrDigest`.
3. PCR 8, PCR 9 and PCR 14 equal the release's expected values. A new host
   tool computes those from the signed image. A boot that extended nothing
   leaves zeros, and zeros are refused.
4. Replaying the log reproduces PCR 4, and the log contains the release
   loader's Authenticode digest.
5. PCRs 0, 1, 2 and 7 equal the machine's enrolled values.

## The code it touches

- **Kernel:**
  - `src/security/attest_doc/`, in `produce.rs` and `document.rs`;
  - the PCR read in `src/security/tpm/boot_reads/` or `tpm/quote/`, turned
    into a general read for up to eight PCRs.
- **Readers:** `capsule_about`'s `doc_parse` moves to version 5 in the same
  change.
- **Proofs:**
  - `attest_doc_proofs` and `attest_key_proofs` move to version 5;
  - `tpm_enroll_proofs/live` gets a swtpm quote over eight PCRs, with a
    move between the read and the quote forced to show the retry;
  - a fuzz target covers the version 5 document parser: `attest_doc` in
    `nsh_attest_proofs`.
- **Mutants:**
  - the selection list;
  - the digest comparison;
  - the retry count;
  - the log cap.

## Open points

- **PCR 8 and PCR 14 are extended in more than one place.** Before setting
  the expected values I will trace, on the lead's first version 5 boot,
  which of those extends actually run on a booted image. The document
  carries the values, so the boot shows them directly.
- **The expected-value tool must not need the signing keys.** It reads the
  signed image and its trailer and nothing else.

## PCR 14: one extend per admitted capsule

Nothing extended PCR 14 on any boot. The loader's two extenders,
`extend_boot_measurements` and `measure_boot_components`, have no caller,
and `kernel_gate` extends PCR 9 only. A version 5 document therefore
carried a PCR 14 of zeros. The kernel now extends it itself, once for each
distinct capsule it admits this boot, at admission, before the capsule
runs. A capsule spawned again with the same body adds nothing.

- **The event.** Type `0x4E4F4E01`.
  - Table 27 "Events" of the TCG PC Client Platform Firmware Profile
    (Version 1.06 Revision 52, section 10.4.1, page 114) assigns
    `0x00000000` to `0x00000012`, and UEFI types in `0x80000000` to
    `0x800000FF`.
  - It says: "Any event type value not defined in this table SHALL NOT be
    used by PC Client Platform Firmware and is reserved for application
    and OS usage." The value lies outside both ranges.
- **The body**, 65 bytes:
  - the capsule's certificate digest (32);
  - the digest of what verified it (32): the trailer's, or SHA-256 of the
    fixed tag `nonos capsule path-only` for a path-only capsule, and for
    no other;
  - a kind byte, from what admitted it:
    - 1, its STARK proof under the vendor's root;
    - 2, its Merkle path alone, on a development image;
    - 3, its STARK proof under a root a person enrolled;
    - 4, this machine's local-build tag, with no proof.
- **The digests.** For every bank the firmware log's header declares, the
  digest is that bank's hash of the same 65 bytes, SHA-1 included when the
  header declares it. The extend carries exactly those banks.
- **One log.** After ExitBootServices the firmware cannot log, so the
  kernel keeps its events and appends them, as `TCG_PCR_EVENT2`, to the
  log copy the document carries. The extend and the append are taken
  under one lock, so the PCR and the log cannot drift. A capsule admitted
  after boot, an install, extends then, and the next document carries
  it.
- **A cap.** At 1024 distinct capsules the kernel stops, as for a failed
  extend: a client replays 4096 events, the firmware's among them.
- **A failed extend** sets a sticky flag. Every later document is refused;
  the boot is not. Only a machine whose TPM was never found skips the
  extend; a command that fails on a TPM that is there is a failure. The reason is `kernel_gate`'s for PCR 9: a TPM fault
  must not stop a machine running, but it must stop the machine from
  claiming what it cannot show.
- **The client** (`nsh_attest`) replays PCR 14 from the log as well. It
  reports each capsule's kind, and the release check requires at least
  one PCR 14 event and every one kind 1, not just a nonzero value. The
  first that is not is named: `path-only`, `developer` or `local-build`.
  A capsule names the path-only tag exactly when it is kind 2, or the
  log is refused.

What PCR 14 does not say:

- A capsule is measured when the gate admits it, so one refused later in
  its spawn is still recorded. That over-reports; it never hides one.
- An image a supervisor checks through `MkLocalVerify` or
  `MkCapsuleVerify` and runs as a foreign guest is not a capsule, and is
  not in PCR 14.
- The client does not tie PCR 14's capsules to the registry's entries, by
  count or by identity.
- A firmware that logs anything of its own in PCR 14 cannot pass: the
  client takes capsule events alone there.
