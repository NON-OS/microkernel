# The self-test: every attack refused, on a booted machine

Part of [Security](README.md).

The self-test boots NONOS three times. In each boot a program tries what an attacker
would try, and the kernel or the gate it attacks refuses. Every attempt prints one line
on the serial console, then `nonos-ci/attack_suite_check.py` reads all three logs, each
on its own. It passes only when every attack in the suite was attempted, every attempt
was refused and said how, and every isolation matrix row has a refusal of its own, with
the errno the gate answers with. It fails on any line that says `ESCAPED`, in whatever
shape, on a guest's `BROKEN` summary, and on an `[ATTACK]` line that is no readable
verdict, such as one cut or split on its way.

None of it ships. The attacker capsule exists only in the `selftest` build profile, which
boots with the development loader. The kernel refuses to compile the capsule beside
`nonos-release` (`src/userspace/capsule_attack/mod.rs`). The flake refuses its feature
beside any loader but `dev-qemu` (`tools/nix/config.nix`). And `CAPSULE_DEV_ONLY` keeps
it out of every make lane's signing and every release seal
(`userland/capsule_attack/Capsule.mk`).

## Running it

```
make selftest                                  # all three boots, then the verdict
make selftest SELFTEST_ARGS=--dry-run          # the commands, run nothing
nix develop --command tools/nonos-selftest --only capsule --iommu
```

| option | what it does |
|---|---|
| `--only NAME` | one boot, `capsule`, `attest` or `linux`; repeat for more. The checker then judges those boots' attacks alone, and the report's `boots` line names them |
| `--iommu` | the capsule boot behind an emulated Intel IOMMU, through `nix run .#qemu`'s `--lane iommu=intel` |
| `--no-build` | use the copy under `target/dev/tree`, its throwaway keys and its sealed image as they are; refused when any of them is missing |
| `--timeout S` | seconds each boot may take, make's own build included (default 3600) |
| `--build-timeout S` | seconds the selftest image's build and seal may take (default 7200) |
| `--out DIR` | where the logs and the report go (default `target/selftest`) |

Each boot runs in a process group of its own and is stopped as a whole: once the lines
it waits for are in its log and a settle time has passed, or at `--timeout`
(`tools/nonos_selftest/watch.py`). A boot that is stopped at the bound has not finished,
and the run fails.

Exit codes: 0 when every boot ran and every attack was refused. 1 when an attack escaped
or never ran, or when a boot did not finish. 2 when a build failed, a tool could not
start, or a log could not be read.

No boot reads or makes a key in this checkout. First, `tools/nonos-dev-image` brings
its copy of the tree under `target/dev/tree` up to date and makes throwaway keys there
and only there: every key the catalogue names, the attack capsule's publisher among
them. With the capsule boot planned it also seals the `selftest-dev` image; without it,
`--keys-only` makes the copy and its keys alone. Then:

- **capsule** boots that image with `nix run .#qemu`, on one CPU. The kernel's trap dump
  is several writes, and on more than one CPU another CPU's line can land inside it.
- **attest** is make's `nonos-mk-attest-refusal-run`, run inside the copy with the copy's
  signing key, so it signs with the copy's throwaway trust anchor and `proof_io` keys.
- **linux** is make's `nonos-mk-run-serial-log` with `NONOS_DEV=1 NONOS_LINUX_GUESTS=1
  LINUX_GUEST_SET=suite`, inside the copy. `userland/linux_guests/Guests.mk` mints its
  scratch guest publishers under the copy's `.keys/`.

Before anything runs, `tools/nonos_selftest/guard.py` refuses a plan with a make boot
outside the copy, or a signing key outside it (exit 2). No row needs the owner's seal.

## The three boots

| boot | attacker | attacks | ends when |
|---|---|---|---|
| capsule | `userland/capsule_attack`, the last capsule init spawns | `cap-escape`, `foreign-memory`, `service-squat`, `held-escape`, `fault-containment` | its `[ATTACK-FAULT]` line is in the log, plus 30 s |
| attest | the probe kernel's spawn gate, against four tampered trailers (`src/userspace/attest_refusal`) | `trailer-flip`, `trailer-extra-cap`, `trailer-kernel-kind`, `trailer-stale-epoch` | make's lane ends; it stops QEMU at 240 s |
| linux | `/bin/suite` under the Linux personality (`userland/linux_guests`) | `linux-fs-escape`, `linux-foreign-memory`, `linux-raw-syscall` | all three attacks are in the log, plus 10 s |

### A panic inside a box

The selftest image's `capsule_linux` is built with the `selftest-panic` feature, and no other
image's is. In that image, `linux --box strict --selftest-panic` makes the personality panic on
purpose, right after the box is armed and before any program runs. The Terminal then has no
report, and the run ends with 134. On screen it says `linux: the box sent no report:
capsule_linux stopped on an internal error (exit 134), so what the program tried is not known`.
On serial it writes `[BOX-MISSING] v=1 report=none capsule_failed=yes exit=134 source=terminal`.
This is typed in the selftest image's Terminal: no attacker in the three boots above types it.

`tools/nonos-dev-image --profile selftest` turns the feature on, in its own copy of the tree
only. `scripts/check_copy_only_features.py`, in the static checks and the flake's `static-abi`
check, fails any committed build that names it: `tools/nix/capsules.json`, a `Capsule.mk`, or
`capsule_linux`'s default features. It also fails if the tool turns it on outside that profile.
Its `--self-test` makes each of those four mistakes and must catch every one.

## The lines

Every attempt prints one line, written by the attacker after the kernel has answered:

```
[ATTACK] <attack> refused: <what was tried>, <call> <errno name> (<errno>)
[ATTACK] <attack> ESCAPED: <what was tried>, <call> returned <value>
```

A line that is neither says why an attempt could not be made, for example when the
service it attacks is not running:

```
[ATTACK-NOTE] cap-escape: a network service is not running, its send was not tried
```

The capsule boot, in order:

```
[SELFTEST] attack capsule spawned
[ATTACK-START] capsule pid 41
[KLOG WARN] [CAP-DENY] pid=41 ipc to net.sockets needs caps 0x4, holds 0x119
[ATTACK] cap-escape refused: a socket through net.sockets without Network, MkIpcSend EPERM (1)
...
[ATTACK-DONE] capsule
[ATTACK-FAULT] pid 41 writes address 0x0
[TRAP PF] cpl=3 rip=... pid=0x0000000000000029 err=0x0000000000000006 cr2=0x0000000000000000
```

The pid and the capability words are examples; the format is the kernel's. When the
spawn gate refuses the capsule, the boot prints `[ERROR] SELFTEST attack capsule spawn
failed` instead, and every one of its attacks fails the run as never attempted.

The capsule cannot report its own fault, so the checker writes that verdict from the
kernel's lines in the capsule boot's own log (`nonos-ci/attack_suite/fault.py`). The
fault is refused when a trap line, at the start of its line, names the capsule's pid at
the address it wrote, and that log goes on after it with no kernel panic. A log that
ends at the trap, or that shows a kernel fault after it, is an escape. The other boots'
logs count for neither. A `fault-containment refused` line printed into a log is
ignored: only the checker writes that refusal. A capsule holding `Debug` could still
print a line shaped like the kernel's trap at the start of a line (see below).

The attest boot prints its four lines from the kernel itself, with the gate's own reason:

```
[ATTACK] trailer-flip refused: <the verifier's reason> code <n>
```

The linux boot prints one line per refused attempt, beside the guest's own line:

```
[ATTACK] linux-fs-escape refused: fs <what>, errno <n>
```

## The kernel's line beside each refusal

The `selftest` profile's kernel feature also echoes the kernel's warnings to the serial
console, as `[KLOG WARN] ...` (`src/log/backend/serial.rs`). No other image does. The
report sets each capsule refusal beside the last `[CAP-DENY]` for the capsule's own pid
since its previous attempt (`nonos-ci/attack_suite/kernel_lines.py`). An attempt the
kernel refused with an errno and no log line says so, and borrows no other line.

That kernel line is evidence for a reader, not part of the verdict. `MkDebug` passes a
capsule's bytes to the console as they are (`src/syscall/microkernel/debug.rs`), so a
capsule holding `Debug` could print a line shaped like the kernel's, a trap line
included. The verdict rests on the errno the attacker's own call returned. Each matrix
row names that errno (`EPERM` for a capability or a name, `EFAULT` for memory that is
not the caller's), so a call refused for another reason, such as no such service or a
renumbered call, proves no row.

## The report

`--out` holds `report.txt` and `report.json`, the same records in two forms. The text's
first line names the format and its version, `nonos-attack-suite-report v1`
(`nonos-ci/attack_suite/report.py`). The version is raised when a record changes shape.

| record | fields |
|---|---|
| `format` | `name`, `version` |
| `scope` | `boots`: the boots whose attacks were judged |
| `log` | `path`, `sha256` of the log as read |
| `dropped` | `attack`, `since`, `reason` |
| `attempt` | `attack`, `verdict` (`refused` or `ESCAPED`), `how`, `kernel` (the kernel's line, or null) |
| `note` | `text`: an `[ATTACK-NOTE]`, an attack the suite does not know, or a dropped attack that was shown |
| `row` | `row`, `proven` |
| `fail` | `reason` |
| `result` | `pass` |

`attack_suite_check.py --list` prints every attack, row and dropped row. `--boot NAME`
judges the logs for that boot's attacks alone.

## Dropped rows

Two rows the suite once expected cannot be shown on any self-test boot. Each keeps its
date and reason in `nonos-ci/attack_suite/rules.py` and in every report:

- `kernel-replay`, since 2026-10-24. The loader's rollback floor needs a TPM counter, and
  under QEMU the floor reads Unprotected, so an older signed kernel would boot
  (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`). The floor rule is proven on the
  host instead, in `boot_proofs` (`floor_rule_tests`, `floor_seq_tests`,
  `floor_raise_tests`).
- `chainload`, since 2026-10-24. The loader has no chainload path to attack: its shell
  launcher starts no image (`nonos-bootloader/src/boot/shell/launch.rs`).

## How the checker is held to account

- **Its mutants.** `attack_suite_check.py --mutants --log ...` makes mutants of a passing
  log, applied to each log on its own, and every one must fail. Each attack's lines are
  removed, its first refusal is flipped to ESCAPED, its refusals are emptied of how, and
  its errnos are renamed. Each row's needed refusal is reworded. An escape is said
  without its colon, and an attack line is split mid-word. The fault's trap is moved to another pid, the log
  is cut at the trap, a kernel panic is appended, and the fault is replaced by a line
  claiming its refusal. A mutant that still passes is a rule that bites on nothing,
  and fails the run (`nonos-ci/attack_suite/mutants.py`).
- **Its tests.** `--self-test`, run by the flake's `static-abi` check, runs the unit tests
  in `nonos-ci/attack_suite/test_*.py`, then the mutants of a synthetic log written for
  that purpose (`sample.py`), which no report presents as a boot's.
- **Fuzzing.** The serial log is untrusted: a guest can print anything into it.
  `test_fuzz.py` feeds the checker seeded garbage, garbage spliced into a passing log,
  logs cut mid-line, and random deletions (`test_fuzz.py`, `test_fuzz_cuts.py`). It
  must never crash or pass on garbage. A log that lost a line the pass rests on must
  fail, whatever else it lost, and a capsule log cut at or before the end of its trap
  must fail.
- **The kernel's mutants.** `verification/mutants.json` holds three kernel builds, each
  with one rule the attacker tests taken out (`ipc-service-gate`, `ipc-inbox-gate`,
  `register-right`). Each names the `[ATTACK] cap-escape ESCAPED` line the capsule boot
  must then print ([A removed control is noticed](checking-the-claims-a-removed-control.md)).
