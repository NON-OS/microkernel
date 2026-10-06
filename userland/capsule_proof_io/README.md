# capsule_proof_io

## Role

`capsule_proof_io` is the boot proof capsule: the smallest signed userland
process, spawned at boot through the verified path, which checks a handful of
syscall refusals from CPL 3 and exits. It depends on no storage, graphics,
network or driver capsule. The capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md).

```text
kernel init
    |
    | verified spawn
    v
proof_io -- raw syscalls --> expected errnos
    |
    `-- MkDebug line, MkExit status
```

## What it checks

`src/main.rs`, in order, exiting with the step's number at the first that
fails:

1. `MkTimeMillis` answers 1024 times without an error (exit 1).
2. An unknown syscall number returns `-ENOSYS` (exit 2).
3. `MkDebug` with a bad pointer returns `-EFAULT` (exit 3).
4. `MkDebug` with a 257-byte line, one past its limit, returns `-EINVAL` (exit 4).
5. Four retired syscall tags (`CryptoSign`, `DebugLog`, `AdminModLoad` and the
   old `GraphicsSurfaceCreate`) return `-ENOSYS` (exit 5).

It then writes `[SYSCALL-PROOF] PASS ...` with `MkDebug` and exits 0. A failure
writes a `[SYSCALL-PROOF] FAIL <step>` line first.

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x18`: IPC and Memory, and
  `CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, for the line that is its output.
  The kernel mirror (`src/userspace/capsule_proof_io`) requests the two bits
  and adds Debug only in a kernel built with `capsule-serial-debug`.
- Service `service:4500:proof_io`, reply `reply:4501:endpoint.proof_io.reply`,
  declared for the manifest; the capsule serves no IPC.
- The feature `nonos-capsule-proof-io` is in `microkernel-desktop-offline` and
  in most single-capsule and smoke profiles, so nearly every image spawns it.
- `mk/25-attest-refusal.mk` builds broken variants of it from its honest
  enrollment for the attestation refusal test profile, which boots the spawn
  gate against them.

## Limits

The capsule holds no Debug, in its manifest or its spawn grant, and the
syscall contract checks Debug before `MkDebug` reads its arguments. So step 3
gets `-EPERM`, not `-EFAULT`, and the run ends with exit 3; its FAIL line,
also written with `MkDebug`, is refused too. No test or smoke script reads the
`[SYSCALL-PROOF]` line.

## Privacy and persistence

The capsule emits fixed lines only. It reads no user data, persists nothing
and holds no secrets.
