# capsule_process_manager

## Role

`capsule_process_manager` is the desktop's Processes window,
`app.process_manager`, a 1240 by 780 app on `nonos_app_skeleton`. It reads
the kernel's process table with `mk_proc_stat` and shows CPU, memory,
capabilities and state per process, with filtering, search, sorting and a
security view, and it can end a process with `mk_kill`. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
process manager (App trait)
    |
    | MkProcStat (table)        MkKill (pid, signal)
    v                           v
kernel process table          kernel, checks ProcessControl
```

## Microkernel contract

- The window, input and frame loop come from `nonos_app_skeleton::run`.
- `MkProcStat` reads every process's row (`src/pm/state/refresh.rs`). Because
  the capsule holds ProcessControl, the kernel shows it every field of every
  row; a caller without ProcessControl or AttestRead sees only identity and
  state for processes other than itself (`src/syscall/microkernel/procstat_redact.rs`
  in the kernel).
- `MkKill` sends SIGTERM. Ending a process is one action, End Process in the
  inspector or `k`, taken twice: the first press arms the exact pid, the
  second sends it (`src/pm/state/kill.rs`). The kernel ends a process at once
  for any signal it accepts (no capsule runs a handler), so there is no
  separate force-quit. It lets a caller signal an unrelated pid only when it
  holds ProcessControl or Admin, and the status strip says what it answered:
  "ended", "denied by the kernel: no authority over that process", or that it
  refused the request (`src/pm/state/notes.rs`).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x2001819`, and `CAPSULE_OPTIONAL_CAPS = 0x100`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | window services |
| 0x0010 | Memory | heap and window backing |
| 0x0800, 0x1000 | GraphicsDisplayQuery, GraphicsSurfaceCreate | its window |
| 0x2000000 | ProcessControl | read every row of the table, signal other processes |
| 0x0100 | Debug (optional) | granted only by a `capsule-serial-debug` build; the capsule makes no debug call |

Endpoints: `service:4736:app.process_manager`, reply `4737`. The kernel
mirror is `src/userspace/capsule_process_manager`.

## Operating rules

- The manager refuses outright to end anything in `CRITICAL`
  (`src/pm/critical.rs`), matched by the name the kernel gives each at spawn:
  init, login, the keyring, the entropy and crypto pools, policy, the input
  router, the VFS, `net.core`, the compositor, the window manager and the
  desktop shell. It refuses to end its own window too, matched by pid, since
  its service name is shared with its other windows, which may be ended.
- With no findings the status strip says "NO FINDINGS", not that the system
  is secure: the monitor checks that watched services keep running, that
  nothing stays pinned at full cpu and that only init holds Admin.
- CPU shares are whole percents, as the kernel reports them.
- Every key it answers to is listed once, and both the dispatcher and the
  help overlay walk that list.
- An empty table says why: still reading, refused by the kernel, or nothing
  matching the filter or search; a refused kill says so too
  (`src/pm/state/notes.rs`).

## Privacy and persistence

All state is in memory: the last table, the rate samples and the view.
Nothing is written anywhere.

## Verification

- Build: `make nonos-mk-process-manager`; sign:
  `make nonos-mk-process-manager-sign`.
- `userland/apps_proofs` includes `src/pm/state/notes.rs`, `src/pm/critical.rs`
  and `src/pm/format.rs` and checks the empty-table and kill lines, which
  processes are protected, and the percent text.
