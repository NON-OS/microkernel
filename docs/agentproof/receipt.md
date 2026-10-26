# AgentProof receipt v0: the format and the offline verifier

A receipt says: this NONOS kernel, measured into this TPM's PCR 9, sealed this run of this
program in this box, and the TPM signed it. It trusts the kernel, the TPM and the release.
It does not trust whoever hands the receipt over.

**Status in 0.9.3.** Built, and booted on a development image under QEMU with a software
TPM and test keys. A headless Gauntlet run of `linux --box strict python3 ...` gets its
receipt from `nonos.prove`'s RECEIPT job, logs it, acknowledges it, and the record is
dropped. On that boot the job's rule held: asked again, the same bytes; asked anything
else, EBUSY; after the ack, the record gone. A development image extends no capsule
measurement into PCR 14, so the verifier refuses its receipts at `release`, as
nsh_attest's own check does (letter server/14). Every other check passed on that receipt,
in a build that takes PCR 14 when it is zero, which no released verifier does. Nothing has
been run on hardware, and no release image has made a receipt.

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
| launcher | 32 | the measurement of the capsule that asked, from the kernel's registry |
| doc | up to 1088 KiB | the NONOSATT v5 attestation document, its unsigned parts cleared |

- The document answers the receipt's challenge:
  `BLAKE3("NONOS-AGENTPROOF-RECEIPT-v0" | every byte before the doc's length)`.
- The three parts the quote does not sign are the capsule count, the registry's complete
  flag and the firmware event log. A receipt carries the count as 0, the flag clear and
  the log empty. Left in, they would be bytes anyone could change without the receipt
  failing, so a receipt claims nothing of them.
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
8. `dma`: the IOMMU was enforcing, with 0 unconfined grants.
9. `record`, `unfinished`, `supervisor`: the record is one NFSR v1 record, finished, with no
   count saturated and no overflow, sealed under the vendor authority by a trusted
   supervisor with its trusted capability mask.
10. `box`, `policy`, `binding`: the box word is one `capsule_linux` takes, the policy is one
    NLXPOL01 policy, and slot 0 is that app in that box.

It then prints the release, the program and its app_id, the box and its hash, the family
it was sealed as, the capsule that asked, and `mode LEARN, not enforced` for a learn run. A
v0 receipt carries no counts.

```
agentproof-receipt pin <receipt>
```

`pin` writes the trust file a receipt verifies under, taken from that receipt: its AK, its
PCR 8, 9 and 14 as release `first-use`, and the supervisor that sealed its record. It
writes the file only when the receipt verifies under it, so every check has passed except
the one the pin makes: that you trust this AK, this release and this supervisor. The file
says so in its comments. Pin once, keep the file, and verify later receipts against it.

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
- The launcher is the measurement nonos.prove read from the kernel's registry for the pid
  that asked. A receipt does not carry the registry's entries, so the verifier cannot check
  that the measurement is in the signed registry. It takes nonos.prove's word for it, as it
  does for the record.
- No counts: v0 carries no opening of the record's histogram.
- Not the registry: v0 carries none of its entries, so it says nothing of which capsules
  ran, or whether the list was whole.

## Where this comes from

- The format and the verifier: `userland/agentproof_receipt`.
- The proofs: `userland/agentproof_receipt_proofs`, with receipts signed by the
  `nsh_attest_proofs` builders. They cover each refusal, a flip of every byte, and 20,000
  mutated receipts. The fuzz target `napr` is in `userland/agentproof_receipt/fuzz`, and the
  mutants are in `verification/host_mutants/receipt.json` (51 of 51 killed).
- The slot-0 rule: `userland/capsule_linux/src/linux/seal_slot.rs`.
- The job: `userland/capsule_prove/src/job/receipt*.rs`. The Terminal's side:
  `userland/capsule_terminal/src/term/terminal/agentproof/receipt*.rs`. The Gauntlet's:
  `tools/agentproof_gauntlet/receipt.py`.
