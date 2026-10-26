# The AgentProof receipt verifier

What [an AgentProof receipt](receipt.md) is checked for, in order, and what a pass does not prove.

## The verifier

```
agentproof-receipt verify [--release] <receipt> <trust>
agentproof-receipt pin <receipt>
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
7. `release`: PCR 9 names a trusted release, and PCR 8 is that release's (nsh_attest's rule;
   PCR 14 is not pinned, it is replayed).
8. `dma`: an IOMMU was present and enforcing, with no grant around it and no device left
   in a domain that reaches all of memory (nsh_attest's `steps::dma`). Then `registry`: the
   receipt's entries fold to the root the quote signed. Then `capsules`: the capsules
   replay to the quoted PCR 14 from zero, each kind is one of nsh_attest's four, and a
   capsule names the path-only tag exactly when its path alone admitted it.
9. `record`, `unfinished`, `supervisor`: the record is one NFSR v1 record, finished, with no
   count saturated and no overflow, sealed under the vendor authority by a trusted
   supervisor with its trusted capability mask.
10. `box`, `policy`, `binding`: the box word is one `capsule_linux` takes, the policy is one
    NLXPOL01 policy, and slot 0 is that app in that box.
11. `launcher`: a launcher the receipt names is one of the capsules the entries list.

Then the release verdict, apart from verification, as nsh_attest gives it: `kind release`
when PCR 14 records at least one capsule and every one was admitted on its STARK proof
under the vendor's root; otherwise `kind not a release: 28 path-only capsules` (each kind a
release does not admit, with how many), or `PCR 14 records no capsule`. A development
image admits every capsule by its path alone, so its receipts verify and are not from a
release. With `--release` such a receipt exits 1; without it, 0.

It then prints the release and its verdict, the program and its app_id, the box and its
hash, the family it was sealed as, the capsule that asked, and `mode LEARN, not enforced`
for a learn run. Then `ran`, how many
capsules the machine was running, and a `network` line for each that held Network: pid,
measurement and authority, or `no capsule held Network`. A v0 receipt carries no counts.

## The trust file

```
nonos-receipt-trust 1
release <version> <pcr8> <pcr9>
supervisor <measurement> <caps>
ak <attestation key>
```

All values are hex, and `#` starts a comment. Name at least one release and one
supervisor, and one AK. v0 pins the AK on first use and checks no EK certificate chain, so
a receipt never says the TPM's maker vouched for the key.

`pin` writes the trust file a receipt verifies under, taken from that receipt: its AK, its
PCR 8 and 9 as release `first-use`, and the supervisor that sealed its record. It writes
the file only when the receipt verifies under it, so every check has passed except the one
the pin makes: that you trust this AK, this release and this supervisor. The file says so
in its comments. Pin once, keep the file, and verify later receipts against it.

## What it does not prove

- A record is genuine if every capsule allowed to ask for an attestation document is release
  code that asks only for a challenge it computed from a record it read. A later version
  should have the kernel compute the challenge from the record itself.
- The launcher is the measurement nonos.prove read from the kernel's registry for the pid
  that asked. The verifier checks it is one of the capsules the signed registry lists, not
  that it was that pid that asked.
- The registry's complete flag is not under the quote, so a receipt carries it clear and
  claims nothing of it: the entries are the ones the signed root names.
- No counts: v0 carries no opening of the record's histogram.

## Where this comes from

- The format and the verifier: `userland/agentproof_receipt`.
- The proofs: `userland/agentproof_receipt_proofs`, with receipts signed by the
  `nsh_attest_proofs` builders. They cover each refusal, a flip of every byte, and 20,000
  mutated receipts. The fuzz target `napr` is in `userland/agentproof_receipt/fuzz`, and the
  mutants are in `verification/host_mutants/receipt.json` (67 of 67 killed).
- The fuzz run, 26 October 2026: 541 s, 1,704,331 inputs grown from one fixture receipt,
  201 new units, no crash, peak RSS 28 MB. Built without cargo-fuzz, with the flags it
  passes: in `userland/agentproof_receipt/fuzz`, `RUSTFLAGS="-Cpasses=sancov-module
  -Cllvm-args=-sanitizer-coverage-level=4 -Cllvm-args=-sanitizer-coverage-inline-8bit-counters
  -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-trace-compares
  --cfg fuzzing -Cdebug-assertions -Coverflow-checks" cargo build --release --target
  x86_64-unknown-linux-gnu --bin napr`, then `napr -max_total_time=540 -max_len=8192 corpus/`.
- The slot-0 rule: `userland/capsule_linux/src/linux/seal_slot.rs`.
- The job: `userland/capsule_prove/src/job/receipt*.rs`. The Terminal's side:
  `userland/capsule_terminal/src/term/terminal/agentproof/receipt*.rs`. The Gauntlet's:
  `tools/agentproof_gauntlet/receipt.py`.
