# Design: the saved networks' generation counter

Status: proposed, before code (Lead's ruling on W8 item 2).

## What it stops

The saved-networks file is sealed under the machine key, so it cannot be
forged. It can still be swapped for an older sealed copy from the same disk,
which brings back a forgotten network or one later pinned to WPA3. A TPM
counter that only rises makes every older copy stale.

## The index

- `0x01000022`, beside the rollback counter `0x01000020`, its base
  `0x01000021` and the legacy `0x01000010`, and listed with them in
  [rollback protection](../security/rollback-protection.md).
- The loader's own template (`DEFINE_COUNTER_CMD`) with only the index
  changed: `TPM_NT_COUNTER`, 8 bytes, SHA-256, AUTHREAD | AUTHWRITE |
  NO_DA, empty authValue.
- The kernel defines it on the first increment, under the owner hierarchy
  with an empty password. If the owner has a password (another OS took
  ownership), the define is refused and the call answers ENODEV, as with no
  TPM.
- No base index: only "below the counter" matters, so a high start is fine.

## Auth and wear

- Only the kernel talks to the TPM, so the capability below is the gate.
  Another OS could raise the counter; that makes the list stale (fail
  closed) but never brings an old file back. Nothing can lower it. NO_DA
  keeps a wrong guess from locking the TPM.
- It is raised only when the list changes (saved, forgotten, pinned to
  WPA3), never on a load or a join. Tens of writes a year are far inside a
  counter's rated wear.

## The call

- `MkSavedGeneration(op, out_ptr)`, tag `MNVG`, in `abi/syscalls.toml`.
  - `op` 0 reads; 1 increments, then reads.
  - `*out_ptr` receives the u64. An index never defined reads 0.
  - Returns 0, EINVAL (other op), EFAULT, ENODEV (no TPM or no owner), or
    EIO (any other TPM answer).
  - Only `0x01000022` (no argument names an index); no DIAG line, no pid.
- Capability: a new bit `SavedState`, for net.core (autojoin reads) and
  settings (saves, forgets); Crypto is too broad. Both manifests re-signed.

## The record, NWF2

- Sealed as today (ChaCha20-Poly1305, `machine_key("wifi/saved-networks")`).
- Plaintext: `"NWF2"` ‖ version 1 ‖ generation u64 LE ‖ the NWF1 slots. The
  generation is inside the seal.
- Save: increment, then seal with the new value and write (your order). A
  crash between the two leaves the file one behind, so it loads as stale and
  the person saves again. The reverse order would allow one step of rollback
  after a crash.
- Load: open, then accept only generation ≥ counter; anything else is
  `Stale`.
- NWF1 is migrated once, and only while the counter reads 0. An NWF1 that
  appears later is stale.
- No TPM: nothing is sealed or saved, as now (`NoTpm`), said plainly.

## Failing closed

- Every load error except "no store" loads no networks. That covers Stale,
  BootChanged, KeyFailed, a bad seal and a bad record. Joins become
  WPA3-only, with the new code `SAVED_UNREADABLE` (-16).
- The panel offers "Forget all", with: "The saved networks were sealed to
  an earlier kernel or firmware, or replaced; forget them to save again."

## Proofs, a mutant per rule

- Kernel host tests: each op and each refusal; the fixed index.
- Client: stale refused, equal and newer accepted, increment before seal,
  NWF1 only at 0, and every load error failing closed except no store.
- Swtpm (`tpm_enroll_proofs`): define, increment, read; a high start.
