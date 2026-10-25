# AgentProof receipt v0: the format and the offline verifier

A receipt says: this NONOS kernel, measured into this TPM's PCR 9, sealed this run of this
program in this box, and the TPM signed it. It trusts the kernel, the TPM and the release.
It does not trust whoever hands the receipt over.

**Status in 0.9.3.** The format, the writer and the verifier are built, and proven on the
host with receipts signed by a test attestation key. No NONOS image writes a receipt yet.
The job that reads a run's sealed record and asks for the attestation document belongs in
`nonos.prove` (letter agent-02, gap 3), and has not been built. So nothing on this page
has been checked against a receipt from a booted machine, and no AgentProof report is
signed.

## What is sealed today

A boxed run's family is sealed by the kernel, in a 240-byte NFSR v1 record. Its slot 0 binds
the program to the box it was held in:

```
BLAKE3("NONOS.agentproof.app.v1" | app_id | box_hash)
```

`app_id` is the program's (`nonos_linux_policy::app_id`), and `box_hash` is the box's policy
hash, the one the report shows. Outside a box, slot 0 is the app_id as before. The
personality's own BusyBox has no app_id. In a box its note binds the box alone, so its run
is sealed, and its receipt names no program.

## The format: NAPR v0

Big-endian. The head is `NAPR` and version 0 (two bytes). Then five fields, each a `u32`
length and its bytes:

| field | bytes | what |
|---|---|---|
| record | 240 | the NFSR v1 record of the run |
| box | 1 to 255 | the box's `--box` word |
| app policy | 256 | the program's NLXPOL01 policy |
| launcher | 32 | the capsule that asked for the run; zero until it can be named |
| doc | up to 1088 KiB | the NONOSATT v5 attestation document, with the two parts its quote does not sign cleared |

- The document answers the receipt's challenge:
  `BLAKE3("NONOS-AGENTPROOF-RECEIPT-v0" | every byte before the doc's length)`.
- The two parts the quote does not sign are the capsule count and the firmware event log.
  A receipt carries the count as 0 and the log empty. Left in, they would be bytes anyone
  could change without the receipt failing.
- One receipt has one encoding. A reader refuses another version, a field of the wrong
  length, and any byte after the doc.

## The verifier

```
agentproof-receipt verify <receipt> <trust>
```

It builds on the host: `cargo build --release` in `userland/agentproof_receipt`. It exits 0
when the receipt verifies, 1 when it is refused, naming the check (`agentproof-receipt:
refused: <name>`), and 2 when a file cannot be used. It checks, in order:

1. `format`: the receipt is one NAPR v0 receipt.
2. `doc`: the document is one v5 document (nsh_attest's own reader).
3. `challenge`: the document answers this receipt's challenge.
4. `unsigned`: the document carries no part its quote does not sign.
5. `doc`: its TPM quote verifies under the AK it carries (nsh_attest's own check).
6. `ak`: the AK is the one the trust file pinned.
7. `release`: PCR 9 names a trusted release, and PCR 8 and 14 are that release's.
8. `dma`: the IOMMU was enforcing, with 0 unconfined grants. Then `registry`: the capsule
   registry was complete.
9. `record`, `unfinished`, `supervisor`: the record is one NFSR v1 record, finished, with no
   count saturated and no overflow, sealed under the vendor authority by a trusted
   supervisor with its trusted capability mask.
10. `box`, `policy`, `binding`: the box word is one `capsule_linux` takes, the policy is one
    NLXPOL01 policy, and slot 0 is that app in that box.

It then prints the release, the program and its app_id, the box and its hash, the family
it was sealed as, and `mode LEARN, not enforced` for a learn run. A v0 receipt carries no
counts.

## The trust file

```
nonos-receipt-trust 1
release <version> <pcr8> <pcr9> <pcr14>
supervisor <measurement> <caps>
ak <attestation key>
```

All values are hex, and `#` starts a comment. Name at least one release and one
supervisor, and one AK. v0 pins the AK on first use and checks no EK certificate chain, so
a receipt never says the TPM's maker vouched for the key.

## What it does not prove

- A record is genuine if every capsule allowed to ask for an attestation document is release
  code that asks only for a challenge it computed from a record it read. A later version
  should have the kernel compute the challenge from the record itself.
- The launcher is not named until the capsule that asks for a run can be (gap 5).
- No counts: v0 carries no opening of the record's histogram.

## Where this comes from

- The format and the verifier: `userland/agentproof_receipt`.
- The proofs: `userland/agentproof_receipt_proofs`, with receipts signed by the
  `nsh_attest_proofs` builders. They cover each refusal, a flip of every byte, and 20,000
  mutated receipts. The fuzz target `napr` is in `userland/agentproof_receipt/fuzz`, and the
  mutants are in `verification/host_mutants/receipt.json` (21 of 21 killed).
- The slot-0 rule: `userland/capsule_linux/src/linux/seal_slot.rs`.
