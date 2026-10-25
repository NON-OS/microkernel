# Checking the security claims yourself: Every capability and control is enforced

Part of [Checking the security claims yourself](checking-the-claims.md).

## Every capability and control is enforced

`tools/nonos-cap-audit` reads the one list in `src/capabilities/types/defs.rs`.
It fails for a capability no code consults, and for an `Mk` system call missing
from the ABI registry or from the capability table (`capabilities` and
`syscalls`, `tools/nonos_cap_audit_checks.py:35-72`). A capability named only
inside a checker that nothing calls does not count as consulted, and neither
does a `.bit()` that builds the mask a capsule is given, under `src/userspace`
or in a `spawn.rs` (`consulted`, `tools/nonos_cap_audit_consulted.py`). Nor
does a constant made from a bit, such as `CAP_ENTROPY`, until some line other
than a `use` reads it (`used_elsewhere`, `tools/nonos_cap_audit_alias.py`).
`IO`, `Hardware` and `Entropy` are on its `UNENFORCED` list, known to enforce
nothing, and a fourth one fails it (`UNENFORCED`, `tools/nonos_cap_audit_checks.py`).

```sh
python3 tools/nonos-cap-audit
```

At this commit it passes and exits 0:

```text
capabilities: 37 declared, 34 consulted, 3 known unenforced
syscalls: 117 Mk numbers, all gated and named
```

`IO` gates nothing, and the kernel's own `capability_table` says so
(`src/capabilities/types/defs.rs:20-21`). `can_read` and `can_write`, the
checkers that read it, are called from nowhere
(`src/syscall/caps/checks/fs.rs:21-27`). The attest refusal probe builds its
`with_io` mask from it (`src/userspace/attest_refusal/probe.rs:30`), which once
counted as consulting it. `--self-test` runs the audit on fixtures that tell
such a mask from a gate. A failing run prints a problem line for each finding
and exits 1. No make target, workflow or flake check runs
this tool.

`scripts/check_unenforced.py` asks the same of every control. It lists three
shapes: a gate-shaped function nothing calls, a policy constant in the security
source trees nothing reads, and a line that logs what it would refuse
(`uncalled_gates`, `unread_policy` and `logs_not_refuses`,
`scripts/unenforced_scan.py:40-66`). The list is held to
`scripts/baselines/unenforced.txt`, which may only lose rows, and a new site
fails it (`run`, `scripts/gate.py:58-91`).

```sh
python3 scripts/check_unenforced.py
python3 scripts/check_unenforced.py --list
```

At this commit the first fails and exits 1:

```text
unenforced: a new control that does not run at src/arch/x86_64/amd_vi/dte.rs:62 allows_write
unenforced: 77 sites, 2 closed since the baseline, 1 new
```

`allows_write`, in the AMD-Vi device table code, is called from nothing in `src`
(`src/arch/x86_64/amd_vi/dte.rs:62`). `--list` prints all 77 sites, `can_read`,
`can_write` and `can_hardware` among them.

`scripts/check_cap_parity.py` compares the kernel's bits with the two userland
tables a capsule names them through (`KERNEL_BITS`, `USER_BITS` and
`MANIFEST_BITS`, `scripts/check_cap_parity.py:36-38`).

```sh
python3 scripts/check_cap_parity.py
```
