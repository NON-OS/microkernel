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

Big-endian. The head is `NAPR` and version 0 (two bytes). Then six fields, each a `u32`
length and its bytes:

| field | bytes | what |
|---|---|---|
| record | 240 | the NFSR v1 record of the run |
| box | 1 to 255 | the box's `--box` word |
| app policy | 256 | the program's NLXPOL01 policy |
| launcher | 32 | the measurement of the capsule that asked, from the kernel's registry |
| doc | up to 1088 KiB | the NONOSATT v5 attestation document, its unsigned parts cleared |
| entries | 45 each, 1 to 4096 | what `MkAttestEntries` gave with the document: every capsule running, with its pid, measurement, capabilities and authority |

- The document answers the receipt's challenge:
  `BLAKE3("NONOS-AGENTPROOF-RECEIPT-v0" | every byte before the doc's length)`.
- The three parts the quote does not sign are the capsule count, the registry's complete
  flag and the firmware event log. A receipt carries the count as 0, the flag clear and
  the log empty. Left in, they would be bytes anyone could change without the receipt
  failing, so a receipt claims nothing of them.
- One receipt has one encoding. A reader refuses another version, a field of the wrong
  length, an entries field that is not whole entries, and any byte after the entries.
- The entries come after the doc, outside the challenge: the quote binds them through
  the registry root they must fold to.

How a receipt is checked, the trust file it is checked against, and what a pass does not prove: [the receipt verifier](receipt-verify.md).
