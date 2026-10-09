# Checking the security claims yourself

Run the tools in the NONOS tree that test its security claims: what each one checks, the command, what a pass and a failure print, and whether CI runs it.

## What you need

You need a checkout of this commit and `python3`. The tools that read only the source tree need nothing else. Outside the flake shell, a `make nonos-mk-*` target runs itself again inside `nix develop` when `NONOS_IN_FLAKE` is unset (`Makefile:144-147`). A tool that needs a built kernel, a QEMU boot or the [trust set](../overview/glossary.md#trust-set) says so below.

| Claim | Tool | Who runs it | At this commit |
|---|---|---|---|
| Ring 0 stays within its budget | `tools/nonos-tcb` | `nonos-verify build` in the `ci` workflow | exits 2 here: no built kernel |
| Every [capability](../overview/glossary.md#capability) is gated by some code | `tools/nonos-cap-audit` | nothing | fails on `IO` |
| Every control runs | `scripts/check_unenforced.py` | flake check `static-abi` | fails on one new site |
| Kernel and userland agree on every bit | `scripts/check_cap_parity.py` | flake check `static-abi`, `make nonos-mk-check-caps` | passes |
| Each [capsule](../overview/glossary.md#capsule)'s mask matches its code | `scripts/cap_audit.py --strict` | flake check `static-tree`, `make nonos-mk-static` | passes |
| Every assumption is in the register | `tools/nonos-assumptions` | flake check `static-abi`, `make nonos-mk-check-assumptions` | passes |
| A removed control is noticed | `tools/nonos-mutant` and the hostile [Linux guests](../overview/glossary.md#foreign-process) | `--check` only, in `static-abi` | `--check` passes; no mutant was booted |
| A boot sends nothing unexpected | `tools/nonos-pcap-egress`, `make nonos-mk-no-telemetry-capture` | the make target `ci-soak` runs the capture; nothing runs `nonos-pcap-egress` | not run on a boot |
| An [amnesic boot](../overview/glossary.md#amnesic-boot) writes nothing | `tools/nonos-amnesic-check` | nothing | reads at most 64 rows, fewer than the QEMU disk holds |
| Signed artifacts verify and tampered ones are refused | `nonos-verify trust-chain` and `adversarial`, `make nonos-mk-attest-refusal-run`, `tools/nonos-flip-byte` | the `trust-chain` and `adversarial` CI jobs | not run here; the tamper targets are broken |
| What the image holds and who holds authority | `tools/nonos_console.py`, `tools/nonos_system_map.py` | nothing | partly run |

The last column is what each tool did when run by hand against this commit, outside the flake. Nothing that needs a build, a boot or the trust set was run. The `verify` workflow runs on `pull_request` and on a push to `main` or `develop` (`.github/workflows/verify.yml:14-16`). Its `flake-check` job runs `nix flake check` on a Linux and a macOS runner (`.github/workflows/verify.yml`), so a flake check is a CI check too. When the flake checks ran on this commit, `static-abi` stopped at `scripts/check_prebuilt.py`, before any check this page names, and `static-tree` reported its `cap_audit --strict` gate as passed while failing on other gates. [CI](../build/ci.md) lists those failures.

`nonos-cap-audit`, `nonos-assumptions` and `nonos-mutant --check` read only the source tree. `nonos-tcb` needs a built kernel. The egress and amnesic checks need a QEMU boot. `nonos-verify trust-chain`, `nonos-verify adversarial` and `nonos_system_map.py` read the trust set.

## Ring 0 stays within its budget

`tools/nonos-tcb` counts what runs in ring 0. It takes the kernel source files that rustc's dep-info for the newest kernel build lists, and counts their non-comment lines under `src/` (`kernel_files` and `code_lines`, `tools/nonos-tcb:52-60`). A file in that set counts whole, code a `cfg` removes included, so the number is an upper bound; generated files, `include_bytes!` payloads and path crates are left out (`tools/nonos-tcb:17-29`).

```sh
make nonos-mk-capsules
make nonos-mk-tcb
```

Not tested in this release.

`nonos-mk-tcb` holds the count to `TCB_BUDGET`, by default `nonos-ci/baselines/tcb-x86_64-capsules.txt`, then runs `tools/nonos-proof-coverage` (`mk/40-run.mk:358-364`). Each budget is a [baseline](../overview/glossary.md#baseline) that moves one way: it is lowered when ring 0 shrinks, and a change that raises it has to say why (`nonos-ci/baselines/README.md`). `tcb-x86_64-capsules.txt` holds 132664 lines for the `microkernel-capsules` kernel, and CI gates on it. `tcb-x86_64-desktop-prod.txt` holds 132739 for the `nonos-mk-desktop-gui-prod` kernel, the desktop built to boot under QEMU, and no target or workflow reads it; to check it, build that kernel and pass `TCB_BUDGET=nonos-ci/baselines/tcb-x86_64-desktop-prod.txt`. The full hardware kernel, `nonos-mk-zerostate`, which the make files call the image that ships, has no budget (`nonos_kernel_build`, `mk/20-build.mk:1187-1197`).

A pass prints `[tcb] <files> files, <lines> lines of ring 0 code` with the dep-info file's name, one line per top-level module, then `[tcb] baseline 132664, delta` and the difference, and exits 0. A count above the budget prints `::error::tcb grew past its baseline` and exits 1 (`held`, `tools/ratchets/budget.py:21-35`). Without a built kernel it prints `no kernel dep-info; build the kernel first` and exits 2 (`kernel_files`, `tools/ratchets/kernel_files.py:26-31`), which is what it printed in this tree.

`nonos-verify build` runs `make nonos-mk-capsules`, then this count against the capsules budget, as its `tcb-budget` check (`run_logged`, `nonos-verify/src/build.rs:41-57`). The `verify` job of `ci-build.yml` runs `nonos-verify build` (`.github/workflows/ci-build.yml:133-166`), and the `ci` workflow calls it on every pull request and every push to `main` and `develop` (`.github/workflows/ci.yml:7-10`). The flake does not run it. No count from a built kernel is recorded for this commit.

## Every capability and control is enforced

`tools/nonos-cap-audit` reads the one list in `src/capabilities/types/defs.rs`. It fails for a capability no code consults, and for an `Mk` system call missing from the ABI registry or from the capability table (`capabilities` and `syscalls`, `tools/nonos-cap-audit:124-162`). A capability named only inside a checker that nothing calls does not count as consulted (`consulted`, `tools/nonos-cap-audit:78-112`). `IO` and `Hardware` are on its `UNENFORCED` list, known to enforce nothing, and a third one fails it (`tools/nonos-cap-audit:115-121`).

```sh
python3 tools/nonos-cap-audit
```

At this commit it fails and exits 1:

```text
on the unenforced list but now enforced, remove it: IO
capabilities: 36 declared, 35 consulted, 2 known unenforced
syscalls: 114 Mk numbers, all gated and named

1 problem(s); a right nothing enforces is not a right
```

The scan counts `IO` as consulted because the attest refusal probe builds its `with_io` mask from it (`src/userspace/attest_refusal/probe.rs:30`). `IO` still gates nothing, and the kernel's own `capability_table` says so (`src/capabilities/types/defs.rs:21-24`). `can_read` and `can_write`, the checkers that read it, are called from nowhere (`src/syscall/caps/checks/fs.rs:21-27`). A clean run prints the same count lines with no problem line and exits 0. No make target, workflow or flake check runs this tool.

`scripts/check_unenforced.py` asks the same of every control. It lists three shapes: a gate-shaped function nothing calls, a policy constant in the security source trees nothing reads, and a line that logs what it would refuse (`uncalled_gates`, `unread_policy` and `logs_not_refuses`, `scripts/unenforced_scan.py:40-66`). The list is held to `scripts/baselines/unenforced.txt`, which may only lose rows, and a new site fails it (`run`, `scripts/gate.py:58-91`).

```sh
python3 scripts/check_unenforced.py
python3 scripts/check_unenforced.py --list
```

At this commit the first fails and exits 1:

```text
unenforced: a new control that does not run at src/arch/x86_64/amd_vi/dte.rs:62 allows_write
unenforced: 77 sites, 2 closed since the baseline, 1 new
```

`allows_write`, in the AMD-Vi device table code, is called from nothing in `src` (`src/arch/x86_64/amd_vi/dte.rs:62`). `--list` prints all 77 sites, `can_read`, `can_write` and `can_hardware` among them.

`scripts/check_cap_parity.py` compares the kernel's bits with the two userland tables a capsule names them through (`KERNEL_BITS`, `USER_BITS` and `MANIFEST_BITS`, `scripts/check_cap_parity.py:36-38`).

```sh
python3 scripts/check_cap_parity.py
```

It printed `cap-parity: 36 capabilities, kernel and 2 mirrors agree` and exited 0. A difference prints `cap-parity: the capability tables disagree`, one line per mismatch, and exits 1.

`scripts/cap_audit.py --strict` holds each capsule's mask to what its code can be shown to need. It fails a capsule whose own code makes a call its mask has no bit for, a capsule that requires `Debug` outside `DEBUG_REQUIRED`, and a crates.io tool whose manifest lacks part of the tool sandbox or that is spawned without the bits the [file store](../overview/glossary.md#file-store) asks for; the `attack` and `toolkit` capsules are exempt (`strict_failures`, `scripts/cap_audit.py:310-335`). It can miss a use reached through a function pointer, a computed system call number or a kernel check not listed in `cap_audit_kernel`, so a bit with no evidence is one to read, not one to drop (`scripts/cap_audit.py:40-49`).

```sh
python3 scripts/cap_audit.py --strict
```

It ended with `cap-audit: 107 capsules, 8 granted bits with no evidence, 1 capsules missing a bit a call needs, 0 strict failures (attack, toolkit exempt)` and exited 0. The one capsule missing a bit is `toolkit`, which is exempt.

`make nonos-mk-check-caps` and `make nonos-mk-static` run `check_cap_parity` before anything is built (`mk/40-run.mk:345-356`). The flake check `static-abi` runs it, `check_unenforced` and the next two tools in one list after `check_prebuilt` (`tools/nix/checks.nix:221-232`). `cap_audit.py --strict` is one gate of `run-static-checks.sh` (`cap_audit_out`, `nonos-ci/run-static-checks.sh:4825-4830`), which the flake check `static-tree` runs.

## The assumption register is complete

[`verification/ASSUMPTIONS.md`](../../verification/ASSUMPTIONS.md) has one row per thing NONOS trusts rather than proves. `tools/nonos-assumptions` finds what it can in the tree: every third-party crate linked into the kernel or the bootloader, the in-tree cryptography, the hardware features the kernel relies on, the toolchains, and any Lean axiom or Verus assumption. It fails for a found assumption with no row and for a found-kind row nothing matches (`unlisted` and `stale`, `tools/nonos-assumptions:55-63`). Rows of kind `stated` have no detector.

```sh
python3 tools/nonos-assumptions
python3 tools/nonos-assumptions --list
```

The first printed `[assumptions] 123 found, 10 stated, 0 unlisted, 0 stale` and exited 0; `--list` printed the 123 found entries. A failure prints `::error::assumption <id> is not in verification/ASSUMPTIONS.md`, or that the register lists an entry nothing in the tree still rests on, and exits 1. `make nonos-mk-check-assumptions`, `make nonos-mk-static` and the flake check `static-abi` run it. [Threat model](../overview/threat-model.md) states the assumptions in prose.

## A removed control is noticed

A test that cannot fail proves nothing. Seven controls each have a mutant, a build with that one control taken out: [`verification/mutants.json`](../../verification/mutants.json) names each by one exact substitution and the line that must then appear. `tools/nonos-mutant` has three modes: `--check` confirms each substitution still applies exactly once, `--apply NAME` rewrites the tree under `--root`, and `--verdict NAME LOG` looks for the line (`stale` and `seen`, `tools/nonos-mutant:50-68`).

| Mutant | File it changes | What the mutant breaks | Line that must appear |
|---|---|---|---|
| `pid-namespace` | `userland/capsule_linux/src/linux/serve/pid_out.rs` | a guest is handed the kernel's pids, not its family's own numbers | `[GUEST] fingerprint ESCAPED the machine's pid count` |
| `family-clock` | `userland/capsule_linux/src/linux/call/epoch.rs` | the monotonic clocks read machine uptime, not time since the family started | `[GUEST] clock ESCAPED the monotonic clock` |
| `statfs-constant` | `userland/capsule_linux/src/linux/file/system/space/room.rs` | `statfs` on a private mount reports one block more than the fixed quota | `[GUEST] fingerprint ESCAPED the store's size` |
| `private-tmp` | `userland/capsule_linux/src/linux/file/private/names.rs` | `/tmp` and the other private prefixes stop being private to a family | `[GUEST] reader ESCAPED the sibling's /tmp/nonos-sibling` |
| `read-only-tree` | `userland/capsule_linux/src/linux/file/root.rs` | a guest may write the shared Linux tree | `[GUEST] reader ESCAPED the sibling's /nonos-sibling` |
| `trap-frame` | `src/process/foreign/trap.rs` | a parked guest's registers also go into the scheduler's resume slot | `[GUEST] exec ESCAPED` |
| `amnesic-gate` | `userland/capsule_vfs/src/server/handlers/persist_gate.rs` | the file store keeps files on an amnesic boot | `[amnesic] the boot left` |

The kernel's `keep` puts a parked guest's frame aside, out of the resume slot, because a guest switched back from that slot would return to user mode with its system call number as the answer (`src/process/foreign/trap_frame.rs:19-32`).

The `[GUEST]` lines come from the hostile Linux guests in `userland/linux_guests`, which attack the [Linux personality](../overview/glossary.md#linux-personality) from inside. A Rust guest prints each attempt as `[GUEST] <guest> refused <what> errno=<n>` or `[GUEST] <guest> ESCAPED <what>: <how>`, and the guest ends with `held` or `BROKEN` and a nonzero exit status on any escape (`check` and `finish`, `userland/linux_guests/src/report.rs:44-74`). With every control in place no `ESCAPED` line appears. With one removed, its line must.

```sh
python3 tools/nonos-mutant --check
```

It printed `[mutant] 7 of 7 apply` and exited 0. A mutant whose code has moved prints `[mutant] <name>: its substitution no longer applies once` and fails. The flake check `static-abi` runs `--check` near the end of its list.

One mutant, end to end:

```sh
git worktree add ../nonos-mutant HEAD
python3 tools/nonos-mutant --root ../nonos-mutant --apply private-tmp
python3 tools/nonos-mutant --verdict private-tmp serial.log
```

Not tested in this release.

Between the second and third commands, build a guest image from the copy and boot it with its [serial console](../overview/glossary.md#serial-console) written to `serial.log`. `--apply` rewrites files in place, so run it on a copy, never on your working tree. The guests are built only when `NONOS_LINUX_GUESTS` is 1 (`mk/20-build.mk:569-570`), and that needs `NONOS_DEV` set to 1 too, because it mints scratch [publisher](../overview/glossary.md#publisher) keys (`userland/linux_guests/Guests.mk:9-10`). `--verdict` prints `[mutant] <name>: caught` and exits 0 when the line is in the log, or `NOT caught, the guest did not notice` and exits 1. For `amnesic-gate` the log is the output of `nonos-amnesic-check --against`, and the boot must try to keep a file; that tool's row limit, below, applies.

No make target, workflow or flake check runs `--apply` or `--verdict`, and no CI workflow or flake derivation builds the guests. None of the seven was booted for this release.

## A boot sends nothing unexpected

What a boot sent is read off the wire. A QEMU boot with a network card, `QEMU_NET_MODE` set to `nat` or `hostfwd`, adds a `filter-dump` of that card writing to `QEMU_NET_CAPTURE` (`mk/10-qemu.mk:124-143`). The mode defaults to `nat` and the capture to `target/qemu-net.pcap`; `QEMU_NET_MODE=off` attaches no card, and an empty `QEMU_NET_CAPTURE` boots without a capture (`mk/10-qemu.mk:35-39`).

`tools/nonos-pcap-egress` reads a capture. It lists each IPv4 destination with a packet count, marked `UNEXPECTED` unless given with `--allow`, then every DNS name queried, and exits 1 when any destination is outside the allow list (`dests` and `allow`, `tools/nonos-pcap-egress:50-70`). It reads IPv4 frames only, so IPv6 and ARP pass unseen, and takes DNS names only from UDP packets to port 53.

```sh
make nonos-mk-run-serial-nat
python3 tools/nonos-pcap-egress target/qemu-net.pcap --allow ADDRESS
```

Not tested in this release.

Each destination prints as `[egress] <address> <count> packets`, and each name as `[egress] dns <name> x<count>`. On scratch captures, one holding only the 24-byte file header printed nothing and exited 0, and one packet to an address outside `--allow` exited 1.

`make nonos-mk-no-telemetry-capture` is the strict form (`no_telemetry_qemu_capture`, `mk/50-ci.mk:26-27`). It boots `make QEMU_NET_CAPTURE=<pcap> nonos-mk-run-serial-net` for 120 seconds unless `NONOS_NO_TELEMETRY_TIMEOUT` says otherwise, and passes only when the capture holds no packet at all, so a DHCP request fails it as surely as any other packet (`timeout_secs` and `bytes`, `scripts/no_telemetry_qemu_capture.sh:22-68`). It prints `no-telemetry capture: PASS no packets captured`, or `FAIL captured <n> bytes` and exits 1, or `GAP` and exits 2 when the boot failed or wrote no capture. It writes `report.json` to `target/no-telemetry`. `NONOS_NO_TELEMETRY_BOOT_CMD` replaces the boot command.

`make nonos-mk-qemu-net-audit` checks the QEMU command lines instead, read from five `make -n` runs of the serial boot targets (`default_cmd`, `scripts/audit_qemu_network_mode.sh:30-43`). Its first check expects the default serial boot to attach no network card, but `QEMU_NET_MODE` defaults to `nat` (`mk/10-qemu.mk:35`), so by the code that check fails at this commit.

Only make runs them: the target `ci-soak` runs the capture and `ci-security` the audit (`mk/50-ci.mk`). No GitHub workflow calls either target, and the flake runs neither. None was run for this release.

## An amnesic boot writes nothing

Every boot is amnesic until first-boot setup records the choice to keep data. On such a boot the file store refuses to keep a file with `EACCES` and writes `[VFS] refused persist: amnesic boot` through `mk_debug`; a write of all zeros, which withdraws a record, is let through (`require_persistent`, `userland/capsule_vfs/src/server/handlers/persist_gate.rs:33-41`).

`tools/nonos-amnesic-check` reads the [package store](../overview/glossary.md#package-store) table of a raw disk image: the `NONOSTR1` container at LBA 256, one row per entry with name, offset, length and digest (`STORE_LBA` and `rows`, `tools/nonos-amnesic-check:28-48`). `--record FILE` saves the rows before a boot. `--against FILE` compares after it, names every row the boot added or changed, and exits 1 when there is one (`added`, `tools/nonos-amnesic-check:62-71`).

The tool reads at most 64 rows and stops with `<image>: <n> entries, above 64` when the table holds more (`MAX`, `tools/nonos-amnesic-check:29-40`). The store format allows 512, and the comment beside that limit puts the image's store at some 220 entries (`MAX_ENTRIES`, `userland/nonos_disk_map/src/container.rs:35-41`). The QEMU disk packs the Linux userland: 19 programs at four files each, 76 rows before Perl's and Tcl's library files (`LINUX_USERLAND_STORE_ENTRIES`, `userland/linux_userland/Userland.mk:135-138`, `userland/linux_userland/Userland.mk:153-181`), so by the code the check stops at its first command on that disk. A scratch store of 70 rows gave exactly that error here.

The QEMU boots use `target/qemu-virtio-blk.img`, with the store packed at LBA 256 (`QEMU_BLK_STORE_STAMP`, `mk/40-run.mk:88-89`). The intended use: boot once so make packs the disk, record it, boot again, stop QEMU, compare.

```sh
python3 tools/nonos-amnesic-check target/qemu-virtio-blk.img --record target/amnesic-before.txt
make nonos-mk-run-serial-log
python3 tools/nonos-amnesic-check target/qemu-virtio-blk.img --against target/amnesic-before.txt
```

Not tested in this release.

A pass ends with `[amnesic] 0 rows added or changed, <n> in all` and exits 0. A failure first prints `[amnesic] the boot left <name> on disk` for each row. A disk with no store gives `no NONOSTR1 container at LBA 256` and exit 1. The pass line and the no-store error both showed on scratch images here. The tool reads the table, not the data behind it: a write that leaves every row as it was is not seen, and a row the boot removed is not reported. No make target, workflow or flake check runs it.

## Signed artifacts verify and tampered ones are refused

`nonos-verify trust-chain` verifies every certificate and manifest in the trust set against the [trust anchor](../overview/glossary.md#trust-anchor)'s policy, and records a `Fail` for any that does not chain (`verdict`, `nonos-verify/src/trust_chain.rs:121-150`). It always records one gap as well: the manifest fields are not yet compared with each `Capsule.mk` (`rpt.gap`, `nonos-verify/src/trust_chain.rs:166-171`). The `trust-chain` job of the `ci` workflow runs it (`.github/workflows/ci.yml`).

`nonos-verify adversarial` takes the first capsule in the trust set whose [NONOS ID certificate](../overview/glossary.md#nonos-id-certificate) and [manifest](../overview/glossary.md#manifest) verify, then attacks copies: a flipped byte in the certificate, in the manifest and in the policy, the certificate checked after its window and before it, the certificate cut in half, and a second capsule's certificate under the first one's manifest (`attacks`, `nonos-verify/src/adversarial.rs:107-179`). Each must be denied. `trust-chain` and `adversarial` both check with the verifier of the `nonos_capsule_sign` crate on the build machine, not the kernel's own code (`nonos-verify/src/adversarial.rs:7-11`, `nonos-verify/src/trust_chain.rs:7-11`). Certificate windows are checked at a fixed time, `NOW_MS`, midnight UTC on 6 May 2026 (`nonos-verify/src/adversarial.rs:16`).

```sh
cargo run --release --manifest-path nonos-verify/Cargo.toml -- adversarial
```

Not tested in this release.

It counts the refusals in `denied`, prints `redteam: <denied>/<total> attacks denied` and writes `security-attestation.json` in its report directory (`nonos-verify/src/adversarial.rs:211-223`), under the directory `NONOS_VERIFY_REPORTS` names (`nonos-verify/src/main.rs:24`). An accepted attack is a `Status::Fail` and the process exits 1 (`nonos-verify/src/main.rs:60-62`). Without the policy, or with no capsule that verifies, it records a gap and exits 0; with only one, the cross-binding attack becomes a gap (`rpt.gap`, `nonos-verify/src/adversarial.rs:63-102`, `nonos-verify/src/adversarial.rs:178`). The fused CI attestation fails on a blocking module's `fail` or a missing module, not on a gap (`blocking_fail`, `nonos-verify/src/attest.rs:37-61`), so read the report as well as the exit status. The `adversarial` job runs it (`.github/workflows/ci-adversarial.yml:41-42`), called from the `ci` workflow (`.github/workflows/ci.yml:77-78`) and from `nightly` and `release`. Neither job's result for this commit is recorded here.

`make nonos-mk-attest-refusal-run` boots a test kernel that embeds deliberately broken copies of the `proof_io` capsule, each of which the [attestation](../overview/glossary.md#attestation) check must refuse; the kernel refuses to compile it beside `nonos-production` (`compile_error`, `src/lib.rs:36-40`). Its probe spawns `flip`, `extra_cap`, `kernel_kind` and `stale_epoch`, then the honest capsule, through the [spawn gate](../overview/glossary.md#spawn-gate), with one `[ATTEST-PROBE]` line each (`run`, `src/userspace/attest_refusal/probe.rs:25-49`). The check passes only with exactly four `[ZK-ATTEST] FAIL proof_io:` refusals, one carrying `STARK proof refused code`, five `as expected` lines and `[ZK-ATTEST] ok proof_io`, and prints `attest refusal: PASS` or `FAIL` (`CASES`, `nonos-ci/attest_refusal_check.py:26-50`). QEMU gets 240 seconds (`ATTEST_REFUSAL_LOG`, `mk/25-attest-refusal.mk:53-62`). The build signs a probe certificate with the trust anchor's seeds (`NONOS_TA_ED25519_SEED`, `mk/25-attest-refusal.mk:25-34`), so it runs only in a tree that holds them. No workflow runs it. [STARK attestation](stark-attestation.md) lists it with the other attestation tests.

```sh
make nonos-mk-attest-refusal-run
```

Not tested in this release.

`tools/nonos-flip-byte` copies a file with one byte inverted, the middle one unless `--at` names another, and keeps the length, so a size check alone cannot catch it (`main`, `tools/nonos-flip-byte:29-45`).

```sh
python3 tools/nonos-flip-byte original.bin flipped.bin
```

On a scratch file it exited 0 with one byte changed. An offset past the end prints `nonos-flip-byte: offset <n> outside <size> bytes` and exits 1. The guest image uses it to place a tampered copy of the guest suite at `/linux/bin/tampered` and of `libprobe.so` at `/linux/lib/libprobe_bad.so`, each beside the good file's proofs, which the personality must refuse however they are reached (`LINUX_GUEST_TAMPERED` and `LINUX_GUEST_BAD_LIB`, `userland/linux_guests/GuestFiles.mk:23-43`). The `dyn` guest prints `[GUEST] dyn ESCAPED: the tampered library loaded` if the bad library loads (`userland/linux_guests/c/dyn.c:26`).

The tamper targets are broken at this commit. `nonos-mk-tamper`, `nonos-mk-tamper-restore` and `nonos-mk-tamper-status` run `tamper_kernel.sh` from `nonos-utils` on the kernel in `ESP_DIR` (`mk/40-run.mk:237-245`), and that script is not in the tree, so each fails before it touches anything. `nonos-mk-tamper-run` still boots the current [ESP](../overview/glossary.md#esp) under `QEMU` without rebuilding it (`mk/40-run.mk:247-253`). By hand:

```sh
cp target/esp/EFI/nonos/kernel.bin target/kernel.bin.good
python3 tools/nonos-flip-byte target/kernel.bin.good target/esp/EFI/nonos/kernel.bin
make nonos-mk-tamper-run
```

Not tested in this release.

What the loader does with a changed kernel is on [Boot chain and signatures](boot-chain-and-signatures.md). Copy `target/kernel.bin.good` back afterwards.

## The whole-image report and the authority map

`tools/nonos_console.py` prints counts and checks over the tree and the built image, one section at a time, in the order `SECTIONS` gives (`tools/nonos_console.py:1532-1547`). Its counts are lines that contain a string under `src/`, found in pure Python by its `rg` helper, which skips `target` directories (`tools/nonos_console.py:125-141`).

```sh
python3 tools/nonos_console.py --fast safety
```

It reported 0 lines with `unwrap()`, `expect()` or `panic!()` under `src/`, 1,498 lines with `unsafe {` and 653 with `SAFETY:`, and exited 0.

Read its numbers as text counts. Its `syscalls` figure, 236 here, counts every line holding `tag4(b"` under `src/syscall`, not the published list (`section_syscalls`, `tools/nonos_console.py:1297-1303`). Its `tcb` figure sums the lines of a fixed list of kernel directories, every architecture included (`trusted`, `tools/nonos_console.py:509-512`), which is a different measure from `nonos-tcb`. Its capability names stop at bit 31 (`CAPABILITIES`, `tools/nonos_console.py:57-68`), so bits 32 to 35 print as a raw `bit:0x` value.

Its `attack` section copies an attested capsule's manifest to a scratch directory and runs `capsule-sign verify-manifest` on it. The untouched copy must be accepted; a flipped byte, another capsule's certificate and a manifest granting `ProcessControl` must be refused; it also checks a trailer's magic and looks for a foreign-architecture ELF (`section_attack`, `tools/nonos_console.py:1450-1527`). A case that is not refused prints `NOT BLOCKED`, and the console exits 1 (`FAILURES`, `tools/nonos_console.py:1577-1583`). Two things make that section unreliable at this commit:

- Without `nonos-sign/target/release/capsule-sign`, `verify_manifest` returns `None` (`tools/nonos_console.py:1425-1427`). The control line still reads `ACCEPTED`, and every case that calls the signing tool reads `NOT BLOCKED`. Build the signing tool first.
- The trailer line compares a trailer's first eight bytes with `TRAILER_MAGIC`, `NZKPATH1` (`tools/nonos_console.py:1508-1511`). A release kernel takes only a v4 [attestation trailer](../overview/glossary.md#attestation-trailer), which starts with `MAGIC_V4` (`parse_v4`, `nonos-attest-path/src/v4/parse.rs:33`); a bare `NZKPATH1` path is taken only by a [development image](../overview/glossary.md#development-image)'s kernel (`dev_path`, `src/security/capsule_attest/path.rs:41-45`). Against a sealed trailer that line reads `NOT BLOCKED`.

`tools/nonos_system_map.py` prints the authority each built capsule holds: an image summary, one row per capsule with its size, port and capabilities, how many capsules hold each capability, and the holders of the twelve capabilities it calls scarce, which it describes as acting on something the holder does not own (`SCARCE`, `tools/nonos_system_map.py:62-70`).

```sh
python3 tools/nonos_system_map.py --fast
python3 tools/nonos_system_map.py --rare
python3 tools/nonos_system_map.py --tsv
```

Not tested in this release.

It reads each mask from `CAPSULE_REQUIRED_CAPS` in the capsule's `Capsule.mk`, and marks a capsule `[not attested]` when its certificate, manifest or trailer in the trust set is missing or empty (`Capsule`, `tools/nonos_system_map.py:163-197`). Its tables count a capsule only when its binary is built under `userland/<capsule>/target/x86_64-nonos-user/release/`; `--tsv` lists every `Capsule.mk`, built or not. It does not decode the signed manifest the spawn gate checks, so it shows the mask the build declares. It knows all 36 capabilities (`CAPABILITIES`, `tools/nonos_system_map.py:43-60`). It is a report, not a gate: it exits 0 whenever it finds a `Capsule.mk` (`main`, `tools/nonos_system_map.py:312-339`).

## What a running machine shows about itself

Two of About's seven sections test claims while you watch: Proofs and Verify (`nav_label`, `userland/capsule_about/src/about/section.rs:43-53`).

Proofs shows whether the running system is attested and whether its network route is anonymous. Each half is Holds only when everything under it holds, Broken when any piece is broken, and Unknown otherwise, and Unknown is never drawn as a pass (`all`, `userland/capsule_about/src/about/data/proofs/session.rs:103-111`). The boot part is the bootloader's record as the kernel reports it, which the kernel does not verify again (`recorded`, `userland/capsule_about/src/about/data/verify/mod.rs:24-28`). The [proof crate](../overview/glossary.md#proof-crate) `attest_doc_proofs` tests the verdict rule on the host; its 46 tests passed in the flake checks of this commit.

Verify runs four checks against one read of the process table, and each can fail (`checks`, `userland/capsule_about/src/about/data/verify/scan.rs:38-48`):

- `no running capsule can write a log`: no live process holds `Debug`.
- `only init holds admin authority`: no process but `init` holds `Admin`.
- `every running capsule carries a capability mask`.
- `this window holds exactly what its manifest declares`: the [capability word](../overview/glossary.md#capability-word) About holds equals the mask it was built to ask for.

By the code, the first two read broken on a running desktop in this release. The policy capsule, which the kernel starts at boot on every desktop image, holds `Admin` (`REQUIRED_CAPS`, `src/userspace/capsule_policy/spawn.rs:32-34`). The file service holds `Debug` on a standard, qemu or dev image, through `serial_debug_cap` (`src/fs/vfs_capsule/spawn.rs:51-53`), and the Linux personality holds it on every image while it runs (`LINUX_CAPS`, `src/userspace/capsule_linux/spawn.rs:39-43`). A broken verdict on those two lines is expected, not a sign of tampering.

Beside them a census counts the capsules, the holders of `FileSystem`, and the holders of any of `Hardware`, `Driver`, `Mmio`, `Irq`, `Dma` and `Pio` (`RAW_HARDWARE`, `userland/capsule_about/src/about/data/caps.rs:54`). Those are counts, not checks.

The `attest` service, `systems.nonos.attest`, listens on port 4444 (`CAPSULE_SERVICE_ENDPOINT`, `userland/capsule_attest/Capsule.mk:8`) and answers any capsule that holds `IPC`, the bit a service [endpoint](../overview/glossary.md#endpoint) asks of senders by default (`required_caps`, `src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:104`). `OP_PROOF_INVARIANTS` returns seven invariants, each with a verdict (`INVARIANTS`, `userland/capsule_attest/src/state/invariants/table.rs:29-72`). Three are checked against the live process table: `NO LOGS`, `PRIVACY MICROKERNEL` and `ADMIN IS INIT ONLY`. `NO LOGS` and `ADMIN IS INIT ONLY` count the same holders as About's first two checks, so by the code they fail for the same reasons. The other four, `NO TRACES`, `EPHEMERAL`, `NOT LINUX` and `HYBRID-PQ SIGNATURES`, are settled where the service cannot look, and come back unchecked, never as a pass (`NotAtRuntime`, `userland/capsule_attest/src/state/invariants/probe.rs:32-36`). The service holds `CoreExec`, `IPC`, `Memory` and `AttestRead` and nothing else (`CAPSULE_REQUIRED_CAPS`, `userland/capsule_attest/Capsule.mk:17`), and nothing in a reply is signed. `OP_PROOF_BOOT` returns the time and a fixed bootloader label, not the loader's verdict (`bootloader`, `userland/capsule_attest/src/server/handlers/proof_boot.rs:27-28`). Its verdicts are a report from the kernel's own table, not a proof to anyone else.

In the Terminal, `log [word ...]` prints the serial console lines the kernel keeps in memory: the newest 200, or every line that names one of the words, ignoring case (`NEWEST`, `userland/capsule_terminal/src/command/builtin/log.rs:29-55`). The kernel keeps the first 64 KiB of the boot and the last 64 KiB (`HEAD` and `CAPACITY`, `src/sys/serial/tail.rs:27-32`), and only in an image built with `capsule-serial-debug`, which the hardened and airgapped [build profiles](../overview/glossary.md#build-profile) drop (`debugFeatures`, `tools/nix/config.nix:84`, `tools/nix/config.nix:92`). The call needs `AttestRead` (`can_attest_read`, `src/syscall/contract/cap_table/mk.rs:54`), which the Terminal holds (`CAPSULE_REQUIRED_CAPS`, `userland/capsule_terminal/Capsule.mk:17-22`). The spawn gate writes one line for each capsule whose trailer it checks: `[ZK-ATTEST] ok <name> <authority>`, `[ZK-ATTEST] FAIL <name>: <reason>` or `[ZK-ATTEST] none <name>` (`print`, `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:28-59`).

```sh
log zk-attest
```

Not tested in this release.

`receipt` prints what the kernel recorded about every running capsule, read with `mk_attest_entries` (`userland/capsule_terminal/src/command/builtin/receipt/run.rs:36`), which needs `AttestRead` too (`src/syscall/contract/cap_table/mk.rs:53`). [Terminal](../using/terminal.md) has the other commands.

## See also

- [Protections and limits](protections-and-limits.md)
- [STARK attestation](stark-attestation.md)
- [Capsule isolation](capsule-isolation.md)
- [Threat model](../overview/threat-model.md)
- [CI](../build/ci.md)
- [Make targets](../build/make-targets.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
- [Kernel logging](../kernel/logging.md)
- [The Linux personality](../userland/linux-personality.md)
