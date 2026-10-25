# Checking the security claims yourself: A removed control is noticed

Part of [Checking the security claims yourself](checking-the-claims.md).

## A removed control is noticed

A test that cannot fail proves nothing. Ten controls each have a mutant, a
build with that one control taken out:
[`verification/mutants.json`](../../verification/mutants.json) names each by one
exact substitution and the line that must then appear. `tools/nonos-mutant` has
three modes: `--check` confirms each substitution still applies exactly once,
`--apply NAME` rewrites the tree under `--root`, and `--verdict NAME LOG` looks
for the line (`stale` and `seen`, `tools/nonos-mutant:50-68`).

| Mutant | File it changes | What the mutant breaks | Line that must appear |
|---|---|---|---|
| `pid-namespace` | `userland/capsule_linux/src/linux/serve/pid_out.rs` | a guest is handed the kernel's pids, not its family's own numbers | `[GUEST] fingerprint ESCAPED the machine's pid count` |
| `family-clock` | `userland/capsule_linux/src/linux/call/epoch.rs` | the monotonic clocks read machine uptime, not time since the family started | `[GUEST] clock ESCAPED the monotonic clock` |
| `statfs-constant` | `userland/capsule_linux/src/linux/file/system/space/room.rs` | `statfs` on a private mount reports one block more than the fixed quota | `[GUEST] fingerprint ESCAPED the store's size` |
| `private-tmp` | `userland/capsule_linux/src/linux/file/private/names.rs` | `/tmp` and the other private prefixes stop being private to a family | `[GUEST] reader ESCAPED the sibling's /tmp/nonos-sibling` |
| `read-only-tree` | `userland/capsule_linux/src/linux/file/root.rs` | a guest may write the shared Linux tree | `[GUEST] reader ESCAPED the sibling's /nonos-sibling` |
| `trap-frame` | `src/process/foreign/trap.rs` | a parked guest's registers also go into the scheduler's resume slot | `[GUEST] exec ESCAPED` |
| `amnesic-gate` | `userland/capsule_vfs/src/server/handlers/persist_gate.rs` | the file store keeps files on an amnesic boot | `[amnesic] the boot left` |
| `ipc-service-gate` | `src/syscall/microkernel/ipc/send_caps.rs` | a send by name reaches a service without the capabilities it requires | `[ATTACK] cap-escape ESCAPED: a socket through net.sockets without Network, MkIpcSend returned` |
| `ipc-inbox-gate` | `src/syscall/microkernel/ipc/send_caps.rs` | a send by pid reaches a process's inbox past the gates of what it serves | `[ATTACK] cap-escape ESCAPED: net.sockets' inbox by its pid without Network, MkIpcSendToPid returned` |
| `register-right` | `src/syscall/microkernel/ipc/register_allowed.rs` | a runtime name is claimed without RegisterService | `[ATTACK] cap-escape ESCAPED: register net.dns without RegisterService, MkServiceRegister returned` |

The kernel's `keep` puts a parked guest's frame aside, out of the resume slot,
because a guest switched back from that slot would return to user mode with its
system call number as the answer (`src/process/foreign/trap_frame.rs:19-32`).

The `[GUEST]` lines come from the hostile Linux guests in
`userland/linux_guests`, which attack the [Linux
personality](../overview/glossary-linux-personality.md#linux-personality) from
inside. A Rust guest prints each attempt as `[GUEST] <guest> refused <what>
errno=<n>` or `[GUEST] <guest> ESCAPED <what>: <how>`, and the guest ends with
`held` or `BROKEN` and a nonzero exit status on any escape (`check` and
`finish`, `userland/linux_guests/src/report.rs:44-74`). With every control in
place no `ESCAPED` line appears. With one removed, its line must.

The `[ATTACK]` lines come from the attack capsule in the [self-test](self-test.md)'s
capsule boot, which holds none of the capabilities those three rules check.

```sh
python3 tools/nonos-mutant --check
```

It printed `[mutant] 7 of 7 apply` and exited 0. A mutant whose code has moved
prints `[mutant] <name>: its substitution no longer applies once` and fails. The
flake check `static-abi` runs `--check` near the end of its list.

One mutant, end to end:

```sh
git worktree add ../nonos-mutant HEAD
python3 tools/nonos-mutant --root ../nonos-mutant --apply private-tmp
python3 tools/nonos-mutant --verdict private-tmp serial.log
```

Not tested in this release.

Between the second and third commands, build a guest image from the copy and
boot it with its [serial
console](../overview/glossary-rollback-index.md#serial-console) written to
`serial.log`. `--apply` rewrites files in place, so run it on a copy, never on
your working tree. The guests are built only when `NONOS_LINUX_GUESTS` is 1
(`mk/20-build.mk:569-570`), and that needs `NONOS_DEV` set to 1 too, because it
mints scratch [publisher](../overview/glossary-peer-list.md#publisher) keys
(`userland/linux_guests/Guests.mk:9-10`). `--verdict` prints `[mutant] <name>:
caught` and exits 0 when the line is in the log, or `NOT caught, the guest did
not notice` and exits 1. For `amnesic-gate` the log is the output of
`nonos-amnesic-check --against`, and the boot must try to keep a file; that
tool's row limit, below, applies.

No make target, workflow or flake check runs `--apply` or `--verdict`, and no CI
workflow or flake derivation builds the guests. None of the seven was booted for
this release.
