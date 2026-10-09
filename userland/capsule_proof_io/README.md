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

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x18`: IPC (`0x8`) and Memory (`0x10`), and
  `CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, for the line that is its output.
  The kernel mirror (`src/userspace/capsule_proof_io/spawn.rs`) requests
  `Capability::IPC | Capability::Memory` and adds Debug only in a kernel built
  with `capsule-serial-debug`.
- Service `service:4500:proof_io`, reply `reply:4501:endpoint.proof_io.reply`,
  declared for the manifest; the capsule serves no IPC.
- The feature `nonos-capsule-proof-io` is in `microkernel-desktop-offline` and
  in most single-capsule and smoke profiles, so nearly every image spawns it.
- `mk/25-attest-refusal.mk` builds broken variants of it from its honest
  enrollment for the attestation-refusal test profile, which boots the spawn
  gate against them.

## Interface contract

The capsule serves no IPC. It is spawned once, makes a fixed sequence of
syscalls from CPL 3 (`src/main.rs`), and exits. In order:

1. `MkTimeMillis` (`mk_time_millis`) answers 1024 times without an error.
2. An unknown syscall tag (`0x21444142`) returns `-ENOSYS` (`-38`).
3. `MkDebug` (`mk_debug`) with a bad pointer returns `-EFAULT` (`-14`).
4. `MkDebug` with a 257-byte line, one past its 256-byte limit, returns
   `-EINVAL` (`-22`).
5. Four retired tags (`CryptoSign`, `DebugLog`, `AdminModLoad`, the old
   `GraphicsSurfaceCreate`) each return `-ENOSYS`.

It then writes `[SYSCALL-PROOF] PASS ...` with `MkDebug` and exits 0.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18` is the whole authority the capsule asks for: IPC
and Memory. Debug is optional (`0x100`) and present only on a
`capsule-serial-debug` kernel. The kernel installs the mask from the verified
manifest at spawn; the capsule cannot widen it, and it holds no storage,
network, graphics or device capability.

## Privacy and persistence

The capsule emits fixed lines only. It reads no user data, persists nothing and
holds no secrets.

## Runtime lifecycle

Spawned once at boot by the verified path. It takes no input, runs the checks in
`_start`, writes one `[SYSCALL-PROOF]` line and exits. It is not restarted and
leaves no open endpoint.

## Failure model

At the first failing check it writes `[SYSCALL-PROOF] FAIL <step>` with `MkDebug`
and exits with the step number (1 to 5). Because Debug is optional, on a kernel
without it `MkDebug` is refused before it reads its arguments: step 3 then gets
`-EPERM`, not `-EFAULT`, the run ends at exit 3, and that step's FAIL line is
refused too.

## Current implemented surface

The five checks above, in order, in `src/main.rs`. Nothing else: no IPC handler,
no second code path.

## Wire format

None. The capsule exposes no IPC endpoint and defines no message layout. Its
only output is the fixed `[SYSCALL-PROOF]` text line written through `MkDebug`.

## State ownership

Stateless. No heap use beyond the static byte strings; no files, no sessions, no
shared memory.

## Operating rules

- Built from its honest enrollment. `mk/25-attest-refusal.mk` builds
  deliberately broken variants from the same source for the attestation-refusal
  profile; those must be refused by the spawn gate.
- No test or smoke script reads the `[SYSCALL-PROOF]` line; the exit status is
  the signal.

## Release target

0.9.2.

## Release evidence

`boot_proofs` exercise the capsule under the verified spawn. The
attestation-refusal profile (`mk/25-attest-refusal.mk`) boots the spawn gate
against the broken variants and requires their refusal.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x18`.
- [ ] The five checks pass under the verified spawn on a `capsule-serial-debug`
      kernel (exit 0).
- [ ] The broken variants are refused by the spawn gate.

## Explicit non-goals today

It exercises the five refusals above, not every refusal path. It is not a
general syscall fuzzer and measures no timing or performance.

## Verification

`src/main.rs` is the whole capsule. The expected errnos (`-38`, `-14`, `-22`)
are fixed by the syscall ABI, and the retired tags resolve as unknown numbers.
`boot_proofs` and the attestation-refusal profile check the behavior end to end.
