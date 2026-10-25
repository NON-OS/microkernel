# Checking the security claims yourself: What you need

Part of [Checking the security claims yourself](checking-the-claims.md).

## What you need

You need a checkout of this commit and `python3`. The tools that read only the
source tree need nothing else. Outside the flake shell, a `make nonos-mk-*`
target runs itself again inside `nix develop` when `NONOS_IN_FLAKE` is unset
(`Makefile:144-147`). A tool that needs a built kernel, a QEMU boot or the
[trust set](../overview/glossary-stark-proof.md#trust-set) says so below.

| Claim | Tool | Who runs it | At this commit |
|---|---|---|---|
| Ring 0 stays within its budget | `tools/nonos-tcb` | `nonos-verify build` in the `ci` workflow | exits 2 here: no built kernel |
| Every [capability](../overview/glossary-boot-root-record.md#capability) is gated by some code | `tools/nonos-cap-audit` | nothing | fails on `IO` |
| Every control runs | `scripts/check_unenforced.py` | flake check `static-abi` | fails on one new site |
| Kernel and userland agree on every bit | `scripts/check_cap_parity.py` | flake check `static-abi`, `make nonos-mk-check-caps` | passes |
| Each [capsule](../overview/glossary-boot-root-record.md#capsule)'s mask matches its code | `scripts/cap_audit.py --strict` | flake check `static-tree`, `make nonos-mk-static` | passes |
| Every assumption is in the register | `tools/nonos-assumptions` | flake check `static-abi`, `make nonos-mk-check-assumptions` | passes |
| A removed control is noticed | `tools/nonos-mutant` and the hostile [Linux guests](../overview/glossary-dma-pool.md#foreign-process) | `--check` only, in `static-abi` | `--check` passes; no mutant was booted |
| A boot sends nothing unexpected | `tools/nonos-pcap-egress`, `make nonos-mk-no-telemetry-capture` | the make target `ci-soak` runs the capture; nothing runs `nonos-pcap-egress` | not run on a boot |
| An [amnesic boot](../overview/glossary-amnesic-boot.md#amnesic-boot) writes nothing | `tools/nonos-amnesic-check` | nothing | reads at most 64 rows, fewer than the QEMU disk holds |
| Signed artifacts verify and tampered ones are refused | `nonos-verify trust-chain` and `adversarial`, `make nonos-mk-attest-refusal-run`, `tools/nonos-flip-byte` | the `trust-chain` and `adversarial` CI jobs | not run here; the tamper targets are broken |
| What the image holds and who holds authority | `tools/nonos_console.py`, `tools/nonos_system_map.py` | nothing | partly run |

The last column is what each tool did when run by hand against this commit,
outside the flake. Nothing that needs a build, a boot or the trust set was run.
The `verify` workflow runs on `pull_request` and on a push to `main` or
`develop` (`.github/workflows/verify.yml:14-16`). Its `flake-check` job runs
`nix flake check` on a Linux and a macOS runner
(`.github/workflows/verify.yml`), so a flake check is a CI check too. When the
flake checks ran on this commit, `static-abi` stopped at
`scripts/check_prebuilt.py`, before any check this page names, and `static-tree`
reported its `cap_audit --strict` gate as passed while failing on other gates.
[CI](../build/ci.md) lists those failures.

`nonos-cap-audit`, `nonos-assumptions` and `nonos-mutant --check` read only the
source tree. `nonos-tcb` needs a built kernel. The egress and amnesic checks
need a QEMU boot. `nonos-verify trust-chain`, `nonos-verify adversarial` and
`nonos_system_map.py` read the trust set.
