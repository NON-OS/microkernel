# Checking the security claims yourself: Signed artifacts verify and tampered ones are refused

Part of [Checking the security claims yourself](checking-the-claims.md).

## Signed artifacts verify and tampered ones are refused

`nonos-verify trust-chain` verifies every certificate and manifest in the trust
set against the [trust
anchor](../overview/glossary-stark-proof.md#trust-anchor)'s policy, and records
a `Fail` for any that does not chain (`verdict`,
`nonos-verify/src/trust_chain.rs:121-150`). It always records one gap as well:
the manifest fields are not yet compared with each `Capsule.mk` (`rpt.gap`,
`nonos-verify/src/trust_chain.rs:166-171`). The `trust-chain` job of the `ci`
workflow runs it (`.github/workflows/ci.yml`).

`nonos-verify adversarial` takes the first capsule in the trust set whose [NONOS
ID certificate](../overview/glossary-linux-personality.md#nonos-id-certificate)
and [manifest](../overview/glossary-linux-personality.md#manifest) verify, then
attacks copies: a flipped byte in the certificate, in the manifest and in the
policy, the certificate checked after its window and before it, the certificate
cut in half, and a second capsule's certificate under the first one's manifest
(`attacks`, `nonos-verify/src/adversarial.rs:107-179`). Each must be denied.
`trust-chain` and `adversarial` both check with the verifier of the
`nonos_capsule_sign` crate on the build machine, not the kernel's own code
(`nonos-verify/src/adversarial.rs:7-11`,
`nonos-verify/src/trust_chain.rs:7-11`). Certificate windows are checked at a
fixed time, `NOW_MS`, midnight UTC on 6 May 2026
(`nonos-verify/src/adversarial.rs:16`).

```sh
cargo run --release --manifest-path nonos-verify/Cargo.toml -- adversarial
```

Not tested in this release.

It counts the refusals in `denied`, prints `redteam: <denied>/<total> attacks
denied` and writes `security-attestation.json` in its report directory
(`nonos-verify/src/adversarial.rs:211-223`), under the directory
`NONOS_VERIFY_REPORTS` names (`nonos-verify/src/main.rs:24`). An accepted attack
is a `Status::Fail` and the process exits 1 (`nonos-verify/src/main.rs:60-62`).
Without the policy, or with no capsule that verifies, it records a gap and exits
0; with only one, the cross-binding attack becomes a gap (`rpt.gap`,
`nonos-verify/src/adversarial.rs:63-102`,
`nonos-verify/src/adversarial.rs:178`). The fused CI attestation fails on a
blocking module's `fail` or a missing module, not on a gap (`blocking_fail`,
`nonos-verify/src/attest.rs:37-61`), so read the report as well as the exit
status. The `adversarial` job runs it
(`.github/workflows/ci-adversarial.yml:41-42`), called from the `ci` workflow
(`.github/workflows/ci.yml:77-78`) and from `nightly` and `release`. Neither
job's result for this commit is recorded here.

`make nonos-mk-attest-refusal-run` boots a test kernel that embeds deliberately
broken copies of the `proof_io` capsule, each of which the
[attestation](../overview/glossary-amnesic-boot.md#attestation) check must
refuse; the kernel refuses to compile it beside `nonos-production`
(`compile_error`, `src/lib.rs:36-40`). Its probe spawns `flip`, `extra_cap`,
`kernel_kind` and `stale_epoch`, then the honest capsule, through the [spawn
gate](../overview/glossary-rollback-index.md#spawn-gate), with one
`[ATTEST-PROBE]` line each (`run`,
`src/userspace/attest_refusal/probe.rs:25-49`). The check passes only with
exactly four `[ZK-ATTEST] FAIL proof_io:` refusals, one carrying `STARK proof
refused code`, five `as expected` lines and `[ZK-ATTEST] ok proof_io`, and
prints `attest refusal: PASS` or `FAIL` (`CASES`,
`nonos-ci/attest_refusal_check.py:26-50`). QEMU gets 240 seconds
(`ATTEST_REFUSAL_LOG`, `mk/25-attest-refusal.mk:53-62`). The build signs a probe
certificate with the trust anchor's seeds (`NONOS_TA_ED25519_SEED`,
`mk/25-attest-refusal.mk:25-34`), so it runs only in a tree that holds them. No
workflow runs it. [STARK attestation](stark-attestation.md) lists it with the
other attestation tests.
