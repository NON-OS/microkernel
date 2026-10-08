# capsule_about

## Role

`capsule_about` is the "About" window: the machine's account of itself. It is a
GUI app on `nonos_app_skeleton`, a 1000 by 680 surface (`WIN_W`/`WIN_H` in
`src/about/ui/metrics.rs`), with seven sections: Overview, Proofs, System,
Trust, Verify, Display and Licenses (`src/about/section.rs`). It reads the
kernel's own records and draws conclusions from them; it reaches no transport.
The capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md),
and the system apps page is
[docs/handbook/apps/system-apps.md](../../docs/handbook/apps/system-apps.md).

```text
nonos_app_skeleton::run  (window, input, frame loop)
    |
    v
About -- MkCapCheck / MkProcStat / MkGetPid      --> Trust, System, Runtime
      -- MkAttestStatus / MkAttestEntries        --> boot verdicts, measured capsules
      -- MkAttestDoc (fresh challenge, on open)   --> TPM attestation document
      -- MkServiceLookup + MkIpcCall (policy, attest) --> Proofs board, route
      -- MkServiceLookup driver.virtio_gpu0        --> Display present path
      -- MkTimeMillis / MkTimeMonotonic            --> clock, uptime
```

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x80001819`: CoreExec (`0x1`), IPC (`0x8`), Memory
  (`0x10`), GraphicsDisplayQuery (`0x800`), GraphicsSurfaceCreate (`0x1000`) and
  AttestRead (`0x80000000`). `CAPSULE_OPTIONAL_CAPS = 0x100`, Debug, is folded in
  only by a `capsule-serial-debug` kernel through `serial_debug_cap()` for its
  `[APP]` log lines; it is not in the required mask. The kernel mirror is
  `src/userspace/capsule_about`.
- Service `service:4710:app.about`, reply `reply:4711:endpoint.app.about.reply`,
  with instance endpoints `service:4846/4848:app.about.1/2` for up to two more
  windows. The window itself serves no IPC; these are its skeleton endpoints.
- The window, input and frame loop come from `nonos_app_skeleton::run`
  (`src/main.rs`). The feature is `nonos-capsule-about` (in `microkernel-about`,
  which builds on `microkernel-desktop-gui`, and in `microkernel-desktop-offline`).

## Interface contract

The window serves no IPC. It is a client that reads kernel records and two
services:

- `MkCapCheck` on its own pid compares the word it holds with the one it was
  built to ask for (`src/about/data/verify/own_mask.rs`).
- `MkAttestStatus` gives the boot verdicts and `MkAttestEntries` the attested
  capsules with their measurements; `MkProcStat` and `MkGetPid` feed System,
  Trust and the live process board.
- On open, `MkAttestDoc` asks once for a TPM-signed attestation document over a
  fresh challenge (`src/about/data/attest_doc.rs`); a machine without a TPM shows
  the refusal as a refusal.
- `MkServiceLookup` with `MkIpcCall` reaches `policy` and `attest` for the Proofs
  board and the chosen route; a lookup of `driver.virtio_gpu0` (no call) tells
  Display which present path is live. `MkTimeMillis` and `MkTimeMonotonic` give
  the clock and uptime.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x80001819` is the whole authority the window asks for.
CoreExec runs user code; IPC reaches policy, attest and the window services;
Memory is the heap; GraphicsDisplayQuery reads the framebuffer size for Display;
GraphicsSurfaceCreate makes its window surface; AttestRead reads the attestation
entries and every live capsule's mask from `MkProcStat`. No Network, FileSystem,
Crypto, Driver, Admin or RegisterService capability is requested: the window
draws conclusions from what it reads and never reaches a transport. The kernel
installs the mask from the verified manifest at spawn; the capsule cannot widen
it.

## Privacy and persistence

The capsule reads no user data and writes nothing. The attestation challenge is
fresh each time the window opens. Nothing is persisted between runs.

## Runtime lifecycle

Launched as a desktop app through `nonos_app_skeleton::run`, which owns the
window, input and frame loop. On open it registers its surface, asks once for
the attestation document, and thereafter reads the kernel records each frame to
repaint the active section. Closing the window ends the process; it is not a
long-lived service and holds no server endpoint.

## Failure model

Every read that can fail is drawn as its failure, never as a pass. A process
table that does not list the window says so; an unanswered clock reads
"unavailable" on every Uptime row; a refused `MkAttestDoc` is shown as a
refusal. The Proofs board marks each half Holds only when everything under it
holds, Broken when any piece is broken, and Unknown otherwise, and Unknown is
never drawn as a pass (`src/about/data/proofs/session.rs`). The admission badge
shows "Verified" only for an admission under a proof;
"Signed, no proof", "Not admitted" and "Unknown" never read as a pass.

## Current implemented surface

The seven sections in `src/about/section.rs`, painted by the screens under
`src/about/ui/screens/`: Overview (product, version, admission badge, headline
tiles), Proofs (`src/about/data/proofs/`), System (build, runtime, memory,
uptime from `MkProcStat`), Trust (the capability word the kernel recorded for
this pid, decoded, plus the trust chain), Verify (the self-checks under
`src/about/data/verify/`), Display (framebuffer size and present path,
`src/about/data/present.rs`) and Licenses (AGPL-3.0 and third-party texts).

## Wire format

The window defines no IPC message layout of its own. It speaks the `policy` and
`attest` service protocols as a client through `MkIpcCall`, and reads kernel
records through the attestation and `MkProcStat` syscalls. Its only output is
the painted surface; on a `capsule-serial-debug` kernel it also emits fixed
`[APP]` log lines through the optional Debug capability.

## State ownership

The window owns its UI state in its own memory: the active section, scroll
offsets and the snapshots it read this frame. The process table is read whole
(`src/about/data/verify/procs.rs`), sized from the kernel's live count, so the
Runtime count, the live checks and the proof board cover every process. It owns
no persistent state and no shared memory beyond its surface.

## Operating rules

- Count the word the process table holds for this pid, not the mask the build
  declares; the declared `MASK` is only Verify's yardstick (`src/about/data/own.rs`).
- Never draw Unknown or a refusal as a pass.
- Ask no transport anything; read the route and proofs from `policy` and the
  `attest` board, where only the transports post.
- Keep the attestation challenge fresh on every open.

## Release target

0.9.2.

## Release evidence

`userland/attest_doc_proofs` holds the attestation-document format the kernel
signs and the Proofs board's verdict rule on the host. `userland/apps_proofs`
(`about_tests`) holds the badge rule, the capability count and the present-path
choice.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x80001819`, Debug optional only.
- [ ] `about_tests` passes: badge rule, capability count, present path.
- [ ] `attest_doc_proofs` passes: document format and verdict rule.

## Explicit non-goals today

The window settles only the claims it can read directly. It reaches no
transport, runs no network or filesystem access, and the claims it cannot settle
are named on their own card with where each is settled instead
(`src/about/data/proofs/`). It is not a configuration UI and changes nothing.

## Verification

- Build: `make nonos-mk-about`; sign: `make nonos-mk-about-sign`. The kernel
  mirror is `src/userspace/capsule_about`.
- Host proofs: `userland/apps_proofs` (`about_tests`) and
  `userland/attest_doc_proofs`.
- Static gate: `bash nonos-ci/run-static-checks.sh`.
