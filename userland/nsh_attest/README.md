# nsh_attest

The nsh/1 client's check of a host, run before any credential is sent:
steps 1 to 7 of `docs/design/nsh-v1-attest.md` over a `NONOSATT` version 5
document (`docs/design/k1-attest-doc-v4.md`). The first failed step ends the
session with `ABORT(POLICY)`, shown to the user as `POLICY (<name>)`.
Proofs, mutants and fuzz targets are in `../nsh_attest_proofs`.

No version 5 producer exists yet: the kernel writes `DOC_VERSION` 4
(version 3 plus the identity-domain device count), and this checker
refuses it. Interoperating with the kernel's document, and a swtpm known-answer test, stay part of K1.

## Proposed amendment to k1-attest-doc-v4.md and nsh-v1-attest.md

**PCR 4.** nsh-v1-attest.md step 5 pins PCR 4 to a release value, and k1
says PCR 4 differs between machines. This checker keeps PCR 4 pinned, but
pins it through the log:

- the log's replay must equal the quoted PCR 4;
- PCR 4 may hold only `EV_NO_ACTION`, `EV_SEPARATOR`, `EV_EFI_ACTION` and
  `EV_EFI_BOOT_SERVICES_APPLICATION` events. The TPM extends digests and
  never types, so any other type could hide a measurement;
- a separator or action must carry the SHA-256 of its own body;
- every application digest in PCR 4 must be the release's loader or a boot
  application the machine's enrolment lists (its boot manager, pinned and
  signed at enrolment), and the loader must be present. "Last application" is not
  enough: an earlier application could load the loader and never start it.

A release therefore carries the loader's Authenticode digest, PCR 8, PCR 9
and the `nsh.front` measurement, not a PCR 4 or PCR 14 value. It carries no
rollback index: the document has none to compare, and the loader enforces
it against the TPM before PCR 9 can name the kernel.

**Replay.** PCRs 0, 1, 2, 4, 7 and 14 must equal the log's replay. PCRs 8
and 9 are extended by NONOS code outside the log and are held to the
release instead. PCR 14 holds the kernel's own events, after the
firmware's: one per distinct admitted capsule, a 65-byte body of
certificate digest, verifier digest and kind (1 vendor proof, 2 path-only,
3 developer root, 4 local build), with the body's SHA-256 as the event's
digest. A capsule names the path-only tag as its verifier exactly when it
is path-only. A release host must have admitted at least one capsule,
every one kind 1 (`pcr14`, `path-only`, `developer`, `local-build`). The header must be PCR 0, `EV_NO_ACTION`, a zero digest,
`Spec ID Event03`, version 2.0, a 32-byte SHA-256 bank and no trailing
bytes. Every event carries exactly the declared banks, each once. A
startup-locality event is honoured once, in PCR 0, before any extend, for
locality 0, 3 or 4. Caps, the loader's: 1048576 bytes, 4096 events, 65536 bytes per body.

**Layout.** Big-endian, every length capped, nothing trailing:
- flags are 0 or 1, and `iommu_vendor` is at most 3;
- `pcr_count` is exactly 8, with indices 0, 1, 2, 4, 7, 8, 9, 14 in order;
- log 0 to 1048576 bytes (an empty log is then refused as `log`);
- attest 1 to 1024, signature 1 to 72, `ak_public` length exactly 64;
- the whole document is at most 1114112 bytes.

**Quote.** The signature is `TPMT_SIGNATURE` ECDSA over SHA-256, with r and
s of 1 to 32 bytes each, left-padded. The selection is one SHA-256 bank,
`sizeofSelect` 3, bitmap `97 43 00`. `qualifiedSigner` is checked for shape
(a SHA-256 name) only, until a swtpm fixture fixes its exact value.

**Other checks.**
- PCR 9 picks the release; PCR 8 and 14 must then be that release's.
- A certificate is refused from `now_ms >= not_after_ms`.
- Every registry entry is under the vendor authority (byte 0), Network or
  not (`not-vendor`), as PCR 14's capsules must all be kind 1.
- Exactly one registry entry holds Network: the release's `nsh.front`.
- `capsule_count` must equal the number of entries sent. It is not bound
  into the quote.
- Host names are compared byte for byte; the client lowercases first.
- Evidence exists only through `Evidence::verify`, which checks both host
  signatures over `HOST_PROOF`.

**Not here.** The signed policy file format, step 8 (the device proof).
