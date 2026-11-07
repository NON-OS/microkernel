# Capabilities: Granting and revoking

Part of [Capabilities](capabilities.md).

## Granting and revoking

- At spawn, `install_spawn` installs the mask from the verified manifest, once;
  a second call is refused (`src/process/caps.rs:110-122`). The mask never comes
  from the spawn site's `requested_caps`, which is only an upper bound for
  optional bits
  (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:25-27`). The
  word is every required bit plus the optional bits the spawn site asked for, as
  `install_caps` computes, and both sets must lie under the [capability
  ceiling](../overview/glossary-boot-root-record.md#capability-ceiling), the
  `allowed_caps_ceiling` of the publisher's identity certificate, which
  `within_ceiling` checks
  (`src/security/capsule_manifest/verify/caps_bits.rs:26-47`). A spawn site that
  asks for a bit the manifest does not declare is refused with
  `GrantOutsideManifest` (`src/security/capsule_manifest/verify/caps.rs:31-39`).
- The [boot profile](../overview/glossary-amnesic-boot.md#boot-profile) trims that word
  before it is installed: `caps` removes `Network` when the profile runs no
  network
  (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:48-55`).
- Every new process first inherits its parent's bits limited to `AMBIENT_CAPS`,
  which is `CoreExec`, `IPC` and `Memory`
  (`src/process/core/table/ambient.rs:28-29`); a verified capsule then gets its
  manifest's mask, and a Linux guest an empty one. A compile time assertion
  keeps hardware, `Admin`, `Debug`, graphics, `SpawnBroker`, `DeviceSecret`,
  `InputConsumer`, `CameraCapture`, `MicrophoneCapture`, `ScreenCapture`,
  `UserFiles`, `AcpiOwner`, `PolicyPush` and `PowerControl` out of that set
  (`src/process/core/table/ambient.rs:34-61`).
- `MkCapGrant(pid, mask)` needs `Admin`, the caller must hold every bit it
  grants, and every bit must lie inside the target's envelope, its manifest's
  required and optional bits under the image ceiling, recorded at spawn
  (`grant_rule::grant_decision`, `src/process/caps_envelope.rs`); `grant` then
  mints a new token with the bits added (`src/process/caps.rs`).
- `MkCapRevoke(pid, mask)` lets a process drop its own bits with a valid
  token alone. Revoking from another process needs `Admin` and being init or
  the target's parent, and a process that owns the compositor, the input
  router, policy or a net stack service answers to init alone
  (`grant_rule::revoke_decision`, `capability/revoke_target.rs`). `revoke` raises the target's
  revocation epoch and mints a token without the bits, so any copy of the old
  token fails `check_revocation_epoch` (`src/process/caps.rs:99-108`).
- `MkCapCheck(pid, mask)`, served by `sys_cap_check`, returns 1 when that pid
  holds every bit of the mask and 0 otherwise; it needs only a valid token, so
  any capsule may ask about any pid
  (`src/syscall/microkernel/capability/handlers.rs:68-74`).

## Groups and delegation in the ABI file

`abi/caps.toml` also names four groups, `BASIC`, `SERVICE`, `OPER` and
`GRAPHICS_SERVICE`, which the file describes as the masks real capsules in this
tree are spawned with (`abi/caps.toml:43-56`), and a `[delegation]` table of
what one group may hand to another, such as `OPER_to_SERVICE`
(`abi/caps.toml:72-78`). These are published policy. The kernel does not read
them, and the syscall check calls only the five checks above.

## What capabilities do not cover

A capability says what kind of call a process may make, not whom it may talk to.
The [held endpoints](../overview/glossary-hardware-broker.md#held-endpoint) and the [peer
list](../overview/glossary-peer-list.md#peer-list) on [IPC](ipc.md) add that. `IO` and
`Hardware` gate nothing at this commit, and `Keyring` and `Entropy` gate only
kernel clients that nothing calls. A capability check is only as good as the
process isolation under it; [Capsule
isolation](../security/capsule-isolation.md) covers that side.
