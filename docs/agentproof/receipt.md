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

How a receipt is checked, the trust file it is checked against, and what a pass does not prove: [the receipt verifier](receipt-verify.md).
