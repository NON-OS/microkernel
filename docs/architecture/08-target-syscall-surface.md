# Target kernel: the system-call surface

Today: 196 system calls ([02-syscall-surface-today.md](02-syscall-surface-today.md)). Target: **46**, every one acting on an object named by a capability handle ([07-target-object-model.md](07-target-object-model.md)). The register convention stays (`rax` number, arguments in `rdi, rsi, rdx, r10, r8, r9`, negative errno in `rax`), and so does the FourCC numbering of `src/syscall/abi/tag.rs:20`, so the ABI checker scripts keep working.

## The calls

Types: `H` is a `CapHandle` (`u32`), `Va` a user virtual address, `Rights` a `u16`, `Badge` a `u64`. Every call returns `i64`: zero or a positive value on success, a negative errno on failure. A call made with a handle the caller does not hold, or without the right it needs, returns `EPERM` and has no other effect.

### Capabilities (6)

| Tag | Call | Effect |
|---|---|---|
| `KCPY` | `cap_copy(src: H, dst_proc: H, rights: Rights) -> H` | Copy into the caller's table, or into a process the caller holds with `Grant`; rights only shrink; refused if the class is outside the receiver's class mask |
| `KMNT` | `cap_mint(src: H, rights: Rights, badge: Badge) -> H` | Copy an endpoint or notification capability with a badge; only from a badge-0 source |
| `KMOV` | `cap_move(src: H, dst_proc: H) -> H` | Move without derivation |
| `KDEL` | `cap_delete(h: H)` | Remove one slot |
| `KRVK` | `cap_revoke(h: H)` | Delete every capability derived from `h`, in every process |
| `KIDN` | `cap_identify(h: H, out: Va)` | Write kind, rights and badge of `h` |

### IPC (6)

| Tag | Call | Effect |
|---|---|---|
| `KSND` | `send(ep: H, msg: Va, len: u32, caps: Va, ncaps: u32)` | Block until a receiver takes the message |
| `KCAL` | `call(ep: H, msg: Va, len: u32, caps: Va, ncaps: u32, reply: Va, rlen: u32) -> u32` | Send and wait for the one-shot reply |
| `KRCV` | `receive(ep: H, buf: Va, len: u32, caps_out: Va, info: Va, timeout_ns: u64) -> Badge` | Wait for a message; returns the sender's badge |
| `KRPY` | `reply_receive(ep: H, msg: Va, len: u32, buf: Va, blen: u32, info: Va) -> Badge` | Answer the last call and wait for the next |
| `KSIG` | `notify(n: H, bits: u64)` | OR bits into a notification |
| `KWAI` | `wait(n: H, timeout_ns: u64) -> u64` | Take and clear pending bits |

### Memory and address spaces (7)

| Tag | Call | Effect |
|---|---|---|
| `KMEM` | `mem_create(quota: H, pages: u32, flags: u32) -> H` | A zeroed MemoryObject charged to a quota |
| `KSEL` | `mem_seal(m: H)` | Make a MemoryObject immutable: every existing writable mapping is removed and no writable one can be made again |
| `KASN` | `as_create(quota: H) -> H` | A new, empty address space |
| `KMAP` | `as_map(as: H, va: Va, m: H, offset: u64, pages: u32, perms: u32)` | Map pages of a MemoryObject; `as` may be the caller's own |
| `KUNM` | `as_unmap(as: H, va: Va, pages: u32)` | Unmap |
| `KPRT` | `as_protect(as: H, va: Va, pages: u32, perms: u32)` | Change permissions, only downward on a started space unless the caller owns it |
| `KACP` | `as_copy(as: H, dst: Va, src: Va, len: u64)` | Copy bytes from the caller into another address space, page-safe; for a child under construction and for the foreign mechanism's peer copy |

### Processes and threads (12)

| Tag | Call | Effect |
|---|---|---|
| `KPRC` | `process_create(as: H, quota: H, request: Va) -> H` | A process around an unstarted address space; `request` names the requested class mask and the verdict ([migration/spawn-gate.md](migration/spawn-gate.md)) |
| `KSTA` | `process_start(p: H, entry: Va, sp: Va, arg: u64) -> i64` | Measure the address space, compare with the verdict's digest, record the measurement, start the first thread. Refused on mismatch |
| `KVRD` | `verdict_create(auth: H, fields: Va) -> H` | A single-use Verdict object; needs the `VerdictAuthority` capability, which only the pinned verifier holds ([09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md)) |
| `KKIL` | `process_kill(p: H, code: i32)` | End a process the caller holds with `Kill` |
| `KTHC` | `thread_create(p: H, entry: Va, sp: Va, tls: Va) -> H` | A new thread in a started process |
| `KTRR` | `thread_read_regs(t: H, out: Va)` | Registers of a suspended or parked thread |
| `KTWR` | `thread_write_regs(t: H, regs: Va)` | Write registers (sanitised RFLAGS, canonical RIP) |
| `KTRS` | `thread_resume(t: H)` | Resume a suspended or parked thread |
| `KTSU` | `thread_suspend(t: H)` | Suspend |
| `KTPR` | `thread_set_priority(t: H, band: u8)` | Up to the caller's own maximum |
| `KEXT` | `exit(code: i32) -> !` | End the calling process |
| `KYLD` | `yield_to(t: H or 0)` | Yield, optionally to a named thread |

### Futex (2)

| Tag | Call | Effect |
|---|---|---|
| `KFTW` | `futex_wait(addr: Va, expected: u32, timeout_ns: u64)` | Within the caller's own address space |
| `KFTK` | `futex_wake(addr: Va, n: u32) -> u32` | Within the caller's own address space |

### Devices (8)

| Tag | Call | Effect |
|---|---|---|
| `KDCR` | `dev_config_read(d: H, off: u16, width: u8) -> u32` | Allow-listed config read |
| `KDCW` | `dev_config_write(d: H, off: u16, width: u8, val: u32)` | Allow-listed config write; `BusMaster` and memory-space enable need those rights |
| `KDBR` | `dev_bar(d: H, bar: u8) -> H` | The BAR as a device MemoryObject (mapped with `KMAP`) or a PortRange |
| `KDRS` | `dev_reset(d: H)` | FLR or the device's reset method |
| `KIOA` | `iommu_attach(dom: H, d: H)` | Attach a device to a domain |
| `KIOM` | `iommu_map(dom: H, iova: u64, m: H, offset: u64, pages: u32, perms: u32)` | Map a MemoryObject for DMA |
| `KIRB` | `irq_bind(src: H, n: H, bit: u8) -> H` | Bind an interrupt source (INTx, MSI, MSI-X entry) to a notification bit |
| `KIRA` | `irq_ack(irq: H)` | Unmask after handling |

`iommu_unmap` is `iommu_map` with zero permissions. A held PortRange takes effect through the TSS I/O bitmap when the holder runs; it needs no call per access.

### Time, power, randomness, debug (5)

| Tag | Call | Effect |
|---|---|---|
| `KTMR` | `timer_arm(n: H, bit: u8, deadline_ns: u64)` | One-shot timer onto a notification |
| `KTAD` | `time_adjust(clock: H, delta_ns: i64)` | Needs the TimeSet class |
| `KPWR` | `power(pwr: H, op: u32)` | Reset, S5 or sleep, after the RAM wipe |
| `KRND` | `random(buf: Va, len: u32)` | The kernel's RNG, at most 256 bytes a call |
| `KDBG` | `debug_write(dbg: H, buf: Va, len: u32)` | Serial console |

The clock page is a read-only MemoryObject every process receives at start; reading time needs no call.

**Total: 6 + 6 + 7 + 12 + 2 + 8 + 5 = 46.**

## From today's surface to the target

| Today's family (count) | In the target |
|---|---|
| IPC and services (11) | `KSND`, `KCAL`, `KRCV`, `KRPY`; service lookup and registration move to the names service |
| Memory (2) | `KMEM`, `KMAP`, `KUNM`, `KPRT` |
| Process, thread, spawn, session, stdio (24) | `KPRC`, `KSTA`, `KVRD`, `KKIL`, `KTHC`, `KEXT`, `KYLD`, futex; capsule load and verify go to init and verifier; process statistics, terminals and stdio go to procmgr |
| Foreign process (20) | Thread register and resume calls, `KACP`, `KMAP` and `KPRT` on the guest's address space; the syscall-redirect endpoint replaces `MkForeignWait` and `MkForeignReply`; `MkLinuxRunNet` is deleted ([12-target-network-authority.md](12-target-network-authority.md)) |
| Device broker (27) | Eight device calls plus `KMAP`; per-access port I/O and the hypervisor pass-throughs become PortRange and MemoryObject capabilities |
| ACPI, platform, NUMA (33) | The acpi service, using MemoryObjects of firmware tables, PortRanges and the power capability |
| Storage, data volume, user files (15) | The volume and storage services |
| Trust, install, sign (17) | The verifier and the installer |
| Attestation, TPM, device secret (10) | The tpm and attest services; the measurement table is a MemoryObject |
| Crypto (12) | The crypto capsule, called directly; only `KRND` stays |
| Graphics, surface, input (12) | The display and input services over shared MemoryObjects and notifications |
| Capability management (3) | The six capability calls |
| Time (5) | The clock page, `KTMR`, `KTAD` |
| Admin (3) | `KPWR`; policy push to the settings service |
| Battery, debug (2) | acpi; `KDBG` |

## How a call is checked

The resolver chain of today (`src/syscall/contract/resolver/resolve.rs:37-41`) becomes one lookup: the handle indexes the caller's table, the slot's kind must match the call, the call's right must be in the slot's rights. There is no allowlist, no token and no per-call predicate table. For transfers (`KCPY`, `KMOV`, IPC with capabilities) the kernel also checks the receiver's class mask, which is where the image ceiling is enforced.

## Compatibility during the migration

The 196 old calls and the new ones coexist during the migration ([13-migration-plan.md](13-migration-plan.md)): new numbers are new FourCC tags, and each old family is removed in the phase that moves its service out. The ABI checker scripts (`scripts/check_syscall_abi.py` and its siblings) apply unchanged, since the five-layer structure stays until the old families are gone.
