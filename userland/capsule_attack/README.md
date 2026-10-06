# capsule_attack

`capsule_attack` is the attacker's side of release gate 4: a
capsule that tries what a hostile program would, and prints one `[ATTACK]` line per attempt
with the kernel's answer. It is test only.

It is not wired into any build yet. `Capsule.mk` names the feature `nonos-capsule-attack` and the
kernel mirror `src/userspace/capsule_attack`, but the root `Cargo.toml` declares no such feature,
the mirror does not exist, and nothing under `mk/` includes this `Capsule.mk`. So no profile and
no image embeds it, and no boot has run it. The attack suite it plans,
which `lib.rs` would refuse beside `nonos-production`, is not written. What runs today is the
checker's self-test, `nonos-ci/attack_suite_check.py --self-test`, in the flake's `static-abi`
check. The isolation model it attacks is described in
[the capabilities page](../../docs/handbook/kernel/capabilities.md).

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS      := 0x119
```

0x119 is CoreExec, IPC, Memory and Debug, the least a capsule runs on. Every attempt is one it
has no right to: it holds no Network, Mmio, Hardware, Admin or RegisterService.

## Attempts

Each is a raw syscall (`src/attempts.rs`), so nothing in libc stands between the attempt and the
kernel's answer.

| Group | Attempt | Refused because |
| --- | --- | --- |
| cap-escape | a send to each traffic service, `net.sockets`, `net.tcp`, `net.dns`, `net.nym`, `net.anon` and `net.socks5`, with MkIpcSend | each takes Network |
| cap-escape | an MMIO window with MkMmioMap | no Mmio or Admin |
| cap-escape | twenty gated calls with no right to them (`src/codes.rs` GATED): claim a device, map DMA, bind an interrupt, read PCI config, take I/O ports, read the attestation registry, ask the TPM to sign, read the device secret or the TPM identity keys, grant itself Network, set the clock, write the store, open another app's window, install an app, host a guest, copy another process's memory, mint a trailer, enrol a signing root, inject input, drain keystrokes | the capability table, EPERM, logged `[CAP-DENY]` |
| cap-escape | kill init with MkKill | not its child or guest, and no ProcessControl |
| foreign-memory | a kernel address, then another address space's, to MkDebug | neither is this capsule's memory |
| foreign-memory | a kernel address as an IPC message, MkIpcSend | the same |
| foreign-memory | a fixed mapping at a kernel address, MkMmap | the kernel half is never the user's |
| foreign-memory | a length that wraps the address space, MkDebug | the user-copy bounds |
| foreign-memory | the process table and the boot record written to a kernel address, MkProcStat and MkAttestStatus | the same |
| foreign-memory | a kernel page unmapped, MkMunmap | outside user space |
| foreign-memory | a page of the broker's DMA window unmapped, MkMunmap, and a fixed mapping in its MMIO window, MkMmap | those pages go back through the broker only (`src/hardware/broker/windows.rs`) |
| cap-escape | MkServiceRegister of `net.dns` on its own port | no RegisterService: `net.dns` is a name the kernel lets a holder of the right claim at run time, so the right is all that is missing (`src/syscall/microkernel/ipc/register_allowed.rs`) |
| cap-escape | a read of the first raw sector and a write past the end of the disk, MkIpcCall to each of `driver.nvme0`, `driver.ahci0` and `driver.virtio_blk0` that runs | no StoreWrite: each disk driver serves its medium to the kernel's client and a StoreWrite holder alone, and answers EACCES in its status word (`server/medium_rule.rs`, virtio-blk's `server/acl/rule.rs`). A boot needs a disk attached for these lines; with none, an `[ATTACK-NOTE]` says so |
| service-squat | MkServiceRegister of `net.sockets` and `app.file_manager` | other manifests own those names |
| fault-containment | a loop that never yields for 3000 ms, then a write to the null page | the scheduler takes the CPU back, and the kernel ends this capsule alone (`src/containment.rs`) |

The Hardware bit gates no call of its own (`src/capabilities/types/defs.rs`). The hardware
operations are gated by Driver, Mmio, Irq, Dma and Pio, and the capsule tries each of them, so the
isolation matrix's Hardware row is the five refusals for device claim, PCI config, I/O ports,
interrupts and DMA.

## Fault containment

The two halves run after `[ATTACK-DONE] capsule`, since the second ends the capsule.

- **The loop.** It spins for 3000 ms and makes no syscall but a clock read every 2^20 turns.
  Before and after, it reads the machine's syscall total from MkProcStat. When the others made
  syscalls meanwhile, it prints `[ATTACK] fault-containment refused: ...` with their count. When
  none did, it prints an `[ATTACK-NOTE]` instead: nothing else wanted the CPU, so the boot shows
  nothing either way.
- **The fault.** It prints `[ATTACK-FAULT] pid N writes address 0x0`, then writes there. The
  checker, `nonos-ci/attack_suite_check.py`, looks for the kernel's `[TRAP PF] cpl=3` line for
  that pid at that address and for more of the log after it, with no `[PANIC ...]` or
  `KERNEL PANIC`. Only then does it count `[ATTACK] fault-containment refused`. If the write
  returns, the capsule itself prints `ESCAPED`.

## What a boot must show

The checker gates each row of the isolation matrix on its own refusal line, so a row with no line
fails the run even when its attack name has passed. Two rows are not this capsule's:

- **DMA escape** needs a device. The broker confines each driver's device to its own IOMMU
  domain and the kernel prints a `[VT-D] IOMMU fault` line for each fault record when a device reaches outside it
  (`src/arch/x86_64/iommu/unit/fault/drain.rs`). The local session proves it with the vIOMMU on.
- **Personality escape** is the Linux guests' rows, `linux-fs-escape`, `linux-foreign-memory`
  and `linux-raw-syscall`.

`src/line.rs` formats the verdict lines, `src/errno.rs` names the errnos in them, and `src/codes.rs` spells each syscall number the way the ABI registry does.
