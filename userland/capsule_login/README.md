# capsule_login

## Role

`capsule_login` is the pre-desktop authentication gate capsule. It validates
session start and end requests before userland shell flow continues, and it
paints the lock screen as a compositor overlay. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

## Microkernel contract

The capsule uses IPC, memory and its own lock screen surface:

- `MkIpcRecv` receives requests on `service:4416:login`.
- `MkIpcSend` returns responses on `reply:4417:endpoint.login.reply`.
- `MkServiceLookup` discovers `keyring`, `desktop_shell`, and `compositor`.
- `MkIpcCall` delegates key validation and emits shell/compositor signals.
- `GraphicsDisplayDimensions` sizes the lock screen, and `MkSurfaceRegister`,
  `MkSurfaceShare` and `MkSurfaceRelease` give it a surface to submit.

The kernel does not keep login session state. It routes IPC and enforces the
signed capsule manifest.

## Interface contract

Ops:

- `HEALTHCHECK`
- `START_SESSION`
- `END_SESSION`
- `GET_STATE`

## Authority

The manifest grants `IPC`, `Memory`, `GraphicsDisplayQuery` and
`GraphicsSurfaceCreate` through `CAPSULE_REQUIRED_CAPS = 0x1818`: the lock
screen asks the display its size and registers its own surface. No device,
MMIO, IRQ, DMA, PIO, network, filesystem, admin, or debug authority is
requested.

## Runtime lifecycle

On boot the capsule discovers dependencies, starts locked, and serves IPC. A
successful `START_SESSION` opens the session for one owner pid and a single
key id. `END_SESSION` relocks state and clears ownership metadata.

## Current implemented surface

- Bounded, deterministic lock/session state machine.
- Keyring calls `OP_UNLOCK` and `OP_LOCK` on session start and end.
- Desktop shell notify signal and compositor damage ping on transitions.
- Deterministic errors for malformed payloads, invalid ownership, and busy
  session transitions.

## Wire format

Request/response envelopes use fixed 20-byte headers and explicit payload
length. Payloads are little-endian:

- `START_SESSION`: `key_id:u32`
- `END_SESSION`: empty body
- `GET_STATE`: empty body, response body `state:u32 owner_pid:u32 session_serial:u32`

## State ownership

The capsule owns lock state, owner pid, key id, and session serial. The kernel
and peers do not mutate session state directly.

## Operating rules

- Only one active session exists at a time.
- `END_SESSION` requires caller pid ownership.
- A session whose owner ended without ending it is locked again before the next request is
  served, and the shell told, as the owner's own end would have (`src/server/reap.rs`,
  `src/state/context/owner_ended.rs`, held by `userland/login_proofs`).
- Key validation is delegated to keyring; no credential bytes are stored in the
  capsule.
- Transition notifications are best-effort IPC calls bounded by reply status.

## Release target

A production-ready login capsule provides deterministic session transitions,
keyring delegation, shell/compositor signaling on session changes, signed
artifacts, and CI/static-gate coverage.

## Release evidence

- Build checks on x86_64/aarch64/riscv64 user targets.
- `make nonos-mk-login` and `make nonos-mk-login-sign`.
- Static checks run with known unrelated blockers documented in the plan.

## Release checklist

- Session start/end/state operations are deterministic.
- Ownership guard on end-session is enforced.
- Sign artifacts and matrix row are updated.

## What does not work today

- `START_SESSION` passes the requesting pid to the keyring as the key's
  owner, but the keyring accepts an owner pid only when it is the pid of the
  message's sender, which is login itself. A start relayed for any other
  capsule's key is refused with `EACCES`.
- No capsule in the tree sends `START_SESSION`; only `userland/login_proofs`
  drives the session state.

## Explicit non-goals today

No password UI rendering, multi-factor auth, persistent login storage, or
kernel-resident auth policy.
