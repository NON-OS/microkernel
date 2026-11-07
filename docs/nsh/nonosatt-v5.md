# NONOSATT version 5

The attestation document a NØNOS kernel produces to answer "what is this
machine running", signed by its TPM. nsh/1 carries it in `HOST_PROOF`. The
client reads it in step 1 of [nsh-v1-attest.md](nsh-v1-attest.md).

- Big-endian throughout, to match the TPM structures it carries.
- Every variable field is length-prefixed with a `u32`.
- A reader that does not recognise the version must refuse the document.
  nsh/1 reads version 5 only; versions 3 and 4 quote PCRs 0, 1, 2 and 7
  alone, which names the firmware and not the NØNOS chain.

Implemented in: `src/security/attest_doc/document.rs`, `src/security/attest_doc/document_consts.rs`, `userland/nsh_attest/src/consts.rs`.

## Layout

| Offset | Field | Size | Rule |
|---|---|---|---|
| 0 | `magic` | 8 | `NONOSATT` |
| 8 | `version` | `u32` | 5 |
| 12 | `challenge` | 32 | |
| 44 | `registry_root` | 32 | |
| 76 | `capsule_count` | `u32` | |
| 80 | `registry_complete` | `u8` | 0 or 1 |
| 81 | `iommu_vendor` | `u8` | 0 to 3 |
| 82 | `iommu_enforcing` | `u8` | 0 or 1 |
| 83 | `unconfined_grants` | `u32` | |
| 87 | `identity_devices` | `u32` | |
| 91 | `pcr_count` | `u8` | 8 |
| 92 | PCR entries | 8 × 33 | each `index: u8`, `value: 32`; indices 0, 1, 2, 4, 7, 8, 9, 14 in that order |
| 356 | `log_len`, `log` | `u32` + 0 to 1 048 576 | the TCG event log |
| | `attest_len`, `attest` | `u32` + 1 to 1024 | `TPMS_ATTEST`, byte for byte as the TPM signed it |
| | `sig_len`, `signature` | `u32` + 1 to 72 | `TPMT_SIGNATURE`, whole |
| | `ak_len`, `ak_public` | `u32` + 64 | the AK's uncompressed P-256 point, x then y; `ak_len` is exactly 64 |

- Nothing may follow `ak_public`.
- The whole document is at most 1 114 112 bytes (1 MiB of log and 64 KiB
  for the rest).
- A reader judges each length against its cap before it takes the bytes.
- The minimum for `attest` and `signature`, the maximum of each, the fixed
  `ak_len` and the document cap are the reader's (`nsh_attest`); the
  kernel's encoder writes each field's length as it is.

Implemented in: `src/security/attest_doc/document_encode.rs`, `userland/nsh_attest/src/doc/parse.rs`, `userland/nsh_attest/src/consts.rs`.

## Field meanings

| Field | Meaning | Bound by the quote |
|---|---|---|
| `challenge` | The 32 bytes the requester asked the document to carry. In nsh/1, the attestation challenge. | yes, through `extraData` |
| `registry_root` | The registry's root over every capsule that passed the spawn gate and has not exited. | yes, through `extraData` |
| `capsule_count` | How many entries the registry holds. | no |
| `registry_complete` | 1 while every running capsule is in the registry. The kernel refuses to produce a document otherwise, so it always writes 1. | no |
| `iommu_vendor` | 0 none, 1 Intel VT-d, 2 AMD-Vi, 3 ARM SMMUv3. Any other value is refused. | yes |
| `iommu_enforcing` | The IOMMU translates with this kernel's tables. | yes |
| `unconfined_grants` | Device mappings no IOMMU domain confines. | yes |
| `identity_devices` | Devices found and not claimed that the IOMMU still lets reach all of memory. | yes |
| PCR values | Read just before the quote and matched against its digest. | yes, through `pcrDigest` |
| `log` | The firmware's TCG event log as the loader carried it, with the kernel's PCR 14 events after it. Empty when the loader carried none. | through the replay to the quoted PCRs |
| `attest`, `signature` | The quote. | |
| `ak_public` | The key `signature` verifies under. | |

Device DMA is confined only when an IOMMU is present and enforcing, and
both `unconfined_grants` and `identity_devices` are zero.

Implemented in: `src/security/attest_doc/document.rs`, `src/security/attest_doc/posture.rs`, `userland/nsh_attest/src/doc/mod.rs`.

## Quoted PCRs

| PCR | Extended by | How a client checks it |
|---|---|---|
| 0, 1, 2, 7 | firmware | Replay of the log; equal to the enrolment certificate. |
| 4 | firmware: boot manager and loader Authenticode digests | Replay of the log; applications held to the release loader and the enrolled boot applications. |
| 8 | the loader | Equal to the accepted release's value. |
| 9 | the loader: the admitted kernel | Picks the accepted release. |
| 14 | the kernel: capsule events | Replay of the log; every capsule kind 1. |

The quote selects these eight in the SHA-256 bank: selection bitmap
`97 43 00`. `pcrDigest` is SHA-256 of the eight values in the order above.

Implemented in: `src/security/attest_doc/document_consts.rs`, `src/security/attest_doc/produce_judge.rs`, `userland/nsh_attest/src/quote/select.rs`.

## The binding

The kernel hands the TPM, as `qualifyingData`, a value that folds the
challenge, the registry root and the DMA posture:

```
qualifying_data = BLAKE3(
    "nonos.attest.bind.v3"
    || challenge                  (32)
    || registry_root              (32)
    || iommu_vendor               (u8)
    || iommu_enforcing            (u8, 0 or 1)
    || unconfined_grants          (u32 BE)
    || identity_devices           (u32 BE)
)                                 = 32 bytes
```

The quote's `extraData` is this value. A verifier recomputes it from the
document's own fields and compares. Replacing the challenge, the root or
any posture value gives a different value than the one the TPM signed.

Implemented in: `src/security/attest_doc/binding.rs` (compiled unchanged into `userland/nsh_attest/src/steps/mod.rs`).

## Production

The kernel produces a document as follows:

1. It refuses if the registry is incomplete.
2. It reads the registry root and the DMA posture once, and computes the
   binding from them.
3. Under the lock every PCR 14 extend takes, it copies the log and takes
   the quote, so the log carried is the one whose events the quoted PCR 14
   holds.
4. It reads the eight PCRs, then quotes them, then checks the quote: the
   quote must answer the binding, select exactly the eight PCRs in the
   SHA-256 bank, and sign the digest of the values read. If a PCR moved
   between the read and the quote, it takes the pair once more. A second
   disagreement is refused.
5. It refuses if the registry became incomplete before the quote.
6. It writes `registry_complete` = 1 and `capsule_count` = the registry's
   count.

- A log over 1 048 576 bytes is refused, never cut.
- When the loader carried no log, the document carries an empty log, and
  a verifier that needs the log refuses it.

Implemented in: `src/security/attest_doc/produce.rs`, `src/security/attest_doc/produce_quote.rs`, `src/security/attest_doc/produce_judge.rs`, `src/security/attest_doc/log.rs`.

## The event log

The log is the TCG crypto-agile format (TCG PC Client Platform Firmware
Profile). Its structures are little-endian, unlike the TPM's.

The first event is a `TCG_PCClientPCREvent` in the SHA-1 shape:

| Field | Size |
|---|---|
| PCR | `u32` = 0 |
| type | `u32` = `EV_NO_ACTION` (3) |
| digest | 20, all zero |
| body size | `u32`, at most 65 536 |
| body | `TCG_EfiSpecIDEvent` |

`TCG_EfiSpecIDEvent`: signature `Spec ID Event03\0` (16), platform class
`u32`, version minor `u8` = 0, major `u8` = 2, errata `u8`, `uintnSize`
`u8` (1 or 2), bank count `u32` (at most 8), each bank `alg: u16`,
`size: u16` (1 to 64, each algorithm once), vendor info size `u8`, vendor
info. Nothing may follow. A 32-byte SHA-256 bank (`0x000B`) must be
declared.

Every later event is a `TCG_PCR_EVENT2`:

| Field | Size |
|---|---|
| PCR | `u32`, below 24 |
| type | `u32` |
| digest count | `u32`, equal to the number of declared banks |
| digests | each `alg: u16` and the bank's digest size; each declared bank once, in any order |
| body size | `u32`, at most 65 536 |
| body | |

The log holds at most 4096 events after the header and 1 048 576 bytes.

Implemented in: `userland/nsh_attest/src/log/header.rs`, `userland/nsh_attest/src/log/event.rs`, `userland/nsh_attest/src/log/mod.rs`.

## PCR 14

PCR 14 holds only the kernel's capsule events, appended to the firmware's
log. There is one event per distinct admitted capsule.

| Field | Value |
|---|---|
| PCR | 14 |
| type | `EV_NONOS_CAPSULE` = `0x4E4F4E01` |
| digests | one per bank the firmware log declares, each the bank's digest of the body |
| body size | 65 |
| body | `cert: 32`, `verifier: 32`, `kind: u8` |

| Kind | Name | Admitted by |
|---|---|---|
| 1 | `vendor` | its STARK proof, under the vendor's root |
| 2 | `path-only` | its Merkle path alone, on a development image |
| 3 | `developer` | its STARK proof, under a root a person enrolled on this machine |
| 4 | `local-build` | this machine's own tag on a local build, with no proof |

- `cert` is the capsule certificate's digest.
- `verifier` is `SHA-256("nonos capsule path-only")` for a kind 2 capsule,
  and SHA-256 of the capsule's trailer for any other kind.
- The event's SHA-256 digest is SHA-256 of the body, so the kind is under
  what the TPM extended.
- A path or a local tag sets kind 2 or 4 whatever root took the capsule.
  Only the vendor's root makes kind 1.

Implemented in: `src/security/capsule_measure/event.rs`, `src/security/capsule_measure/kind.rs`, `userland/nsh_attest/src/log/pcr14.rs`, `userland/nsh_attest/src/log/admitted.rs`.

## Registry entries

The registry lists every capsule that passed the spawn gate and has not
exited. The document carries only its root. nsh/1 carries the entries in
`HOST_PROOF`.

Each entry is 45 bytes:

| Offset | Field | Size | Encoding |
|---|---|---|---|
| 0 | `pid` | 4 | `u32` BE |
| 4 | `measurement` | 32 | the value the capsule's proof was checked against |
| 36 | `caps` | 8 | `u64` BE capability mask, bit *i* = kernel capability *i* |
| 44 | `authority` | 1 | 0 vendor; 1 + slot for a developer root; 255 publisher |

```
registry_root = BLAKE3("nonos.attest.registry.v1" || count (u32 BE) || entries)
```

- Entries are folded in ascending `pid` order, and sent in that order.
- `count` is the number of entries. A verifier takes it from the byte
  length divided by 45, never from a field it was handed.
- An empty registry hashes the domain and a zero count.
- The authority byte is inside the fold, so the same measurements under
  different authorities give different roots.
- Bit 2 of `caps` is `Network`.

Implemented in: `src/security/attest_registry/format.rs`, `src/security/attest_registry/entry.rs`, `src/security/attest_registry/root.rs`, `src/security/attest_registry/order.rs`, `src/security/attest_registry/dump.rs`, `userland/nsh_attest/src/steps/registry.rs`.
