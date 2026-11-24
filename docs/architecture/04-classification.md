# Classification: what stays, what moves, what is deleted

Every subtree of `src/` gets one of five outcomes. The full table, one row per path prefix, is the appendix; [method.md](method.md) gives the script that applies it. This page gives the rule, the decisions and the reasons.

| Class | Meaning | phys | code |
|---|---|---:|---:|
| K | Stays in ring 0 as it is, apart from the object-model changes of [07-target-object-model.md](07-target-object-model.md) | 134,781 | 68,469 |
| R | Stays in ring 0 in a reduced form; the rest moves or is deleted | 51,949 | 27,531 |
| U | Moves to a named userland service | 96,019 | 49,693 |
| D | Deleted: dead or inert today, or made redundant by a move | 67,067 | 41,054 |
| X | Not compiled for x86_64; the same rules apply to it when another architecture ships | 47,100 | 22,897 |

The phys figures for K and R are today's sizes of the code that stays. The target size of ring 0 is worked out in [13-migration-plan.md](13-migration-plan.md).

## The rule

A piece of code stays in ring 0 only if at least one of these holds:

1. **It executes a privileged instruction or writes state the CPU or an IOMMU trusts for isolation**: page tables, CR3, the IDT, GDT and TSS, MSRs, the local and I/O APIC, IOMMU tables and registers, the `SYSCALL` and interrupt entry paths.
2. **It is the capability check itself**: the code that decides whether a caller holds the authority for an operation on a kernel object.
3. **It must be atomic with respect to scheduling**: the context switch, the IPC rendezvous that moves a message and a reply capability between threads, the scheduler's choice.
4. **It must run before any user thread can**: early memory, the first address space and the start of the first user task.

Nothing else qualifies, however trusted it is. A component can stay in the TCB and still leave ring 0; [05-trusted-computing-base.md](05-trusted-computing-base.md) treats that separately. Parsing untrusted bytes (ELF images, ACPI tables, manifests, certificates, attestation trailers, TCG logs) never qualifies on its own: the parse can run in an address space that holds only the authority its answer needs.

## The destination services

These are the userland components that receive what leaves ring 0. Their trust and its bounds are in [05-trusted-computing-base.md](05-trusted-computing-base.md); the capabilities named here are the object capabilities of [07-target-object-model.md](07-target-object-model.md).

| Service | Receives | Holds | New kernel interface it needs |
|---|---|---|---|
| **init** (root task) | `src/userspace/init` (spawn plan, supervisor, Linux jobs), every `embed.rs` and `spawn.rs` (as a boot package of images, certificates, manifests and trailers), `services/lifecycle`, `tool_capsules`, app launch and instance spawn, prover jobs | Every capability the kernel creates at boot: untyped memory, the ASID pool, IRQ control, the device root, the TPM window, the boot package frames | The object model itself: address-space, thread, endpoint, frame and untyped capabilities; a boot-info page that describes them |
| **loader** | The live ELF path (`elf/loader`, `elf/types`, `elf/errors`, `elf/aslr`) | During one spawn: the new address space's capability and the image's frames; afterwards nothing | `MapFrame` into a target address space, `ProtectRange`, `ThreadWriteRegisters` (see [migration/elf-loading.md](migration/elf-loading.md)) |
| **verifier** | Certificate, manifest and attestation verification (`security/capsule_manifest`, `nonos_id_cert`, `nonos_trust_anchor`, `capsule_attest`, `admission_cache`, `dev_roots`, `local_build`, `attest_policy`), the verification crypto, `MkCapsuleVerify`, `MkLocalVerify`, `MkLocalSign`, enrolment of dev and local roots | Read access to frames it is handed; a badged endpoint on which it answers; for local signing, a key derived through the TPM service | None beyond IPC with capability transfer; the kernel trusts its answer through the mechanism of [migration/spawn-gate.md](migration/spawn-gate.md) |
| **devmgr** (device manager) | Claim admission rules, scope tables, reset, restore and wipe sequencing (`hardware/broker/claim` in part, `scope*.rs`, `census`), `hardware/inventory`, `hardware/boot_census`, PCI enumeration and BAR assignment (`bus/pci`, part of `drivers/pci`) | The device root: per-device config-window, device-frame, IOMMU-domain and IRQ capabilities, which it hands to drivers | Device, IOMMU-domain and IRQ objects (see [10-target-device-authority.md](10-target-device-authority.md)) |
| **acpi** (the existing ACPI capsule) | Table parsing (`arch/x86_64/acpi` except PM1 and reset), the resource extractor (`aml`), ACPI device description, the grant and policy rules of `hardware/broker/acpi`, battery and thermal policy, `MkAcpi*` | Firmware-table frames, OperationRegion windows as frame and port-range capabilities, a power endpoint | Port-range capabilities in place of per-access port I/O system calls |
| **tpm** | The TPM driver and command builders (`security/tpm`), the boot verdict (`security/boot`), the attestation document, PCR 14 capsule events, the machine key, the device secret, enrolment | The TPM window as a device-frame capability, from devmgr | None |
| **attest** | The running-capsule registry as a report, `MkAttestDoc`, `MkAttestEntries`, `MkBootAttest` | A read capability on the kernel's measurement table; an endpoint to tpm | A read-only measurement table per process (see [09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md)) |
| **volume** | The encrypted data volume (`fs/blockfs_volume`, `fs/blockfs`, `fs/cryptoblock`), `MkData*`, `MkUserObj*`, its AEAD and Argon2, the keyring client, zerostate's volume rules | An endpoint to storage, an endpoint to tpm, an endpoint to the keyring capsule | None |
| **storage** | The block layer (`hardware/block_device`), its clients for nvme, ahci, virtio_blk and usb_msc, raw store sectors (`MkStoreRead`, `MkStoreWrite`) | Endpoints to the block-driver capsules | None |
| **names** | The service registry (`services/registry`, `services/caps`), `MkServiceLookup`, `MkServiceRegister` | One endpoint capability per registered service, received from init | Capability transfer over IPC |
| **procmgr** | Process statistics, capability listings, log tail, terminal table, process stdout and stdin, private writes | A read capability on the kernel's process-information page; endpoints to terminals | A read-only process-information page |
| **display** | The surface registry's policy, fences, vsync and the input ring (`kernel_core/surface_registry`), the framebuffer blit, `MkSurface*`, `MkInput*` | The boot framebuffer as a device-frame capability; memory objects shared with clients | Shared memory objects (frames mapped into several address spaces) |
| **installer** (the existing `capsule_installer`) | Install sources, app install and uninstall | An endpoint to verifier, to volume and to storage | None |
| **settings** | Hostname, timezone and policy push (`sys/policy`) | An endpoint | None |
| **linux** (the existing personality) | The personality's roles and terminal tickets (`src/userspace/capsule_linux`) move to init; the personality keeps its userland | Per guest: the guest's address-space and thread capabilities; per connection, a narrow network capability ([12-target-network-authority.md](12-target-network-authority.md)) | Fault and syscall delivery to an endpoint, already present as the foreign mechanism |

## Decisions by area

### Stays in ring 0 (K)

| Area | Why it cannot leave |
|---|---|
| `arch/x86_64`: `asm`, `syscall`, `context`, `gdt`, `cpu`, `boot`, `interrupt`, `iommu`, `paging`, `pat`, `port`, `mmio_order`, `idle`, `diag`; `arch/paging`, `arch/trap`, `arch/interrupt_controller`, `arch/user_access` | Privileged instructions and the hardware's isolation state (rule 1) |
| `arch/x86_64/acpi/hw` | PM1 sleep, reset and power-off are port writes that end every address space at once; the ACPI service asks for them through a power capability |
| `memory/paging`, `phys`, `frame_alloc`, `buddy_alloc`, `heap`, `layout`, `kaslr`, `mmio`, `iommu`, `encryption`, `addr`, `unified`, `proof`, `dma/coherency` | Page tables, frame ownership and the kernel's own memory (rules 1 and 4) |
| `process` core, scheduler (without its dead policy registry), exit, userspace, context, caps, accounting, `address_space/lifecycle` | Threads, address spaces, scheduling (rules 1 and 3) |
| `smp`, `interrupts`, `sys/apic`, `sys/serial`, `sys/idt`, `sys/timer`, `sys/sync/irq_mutex`, `sys/boot_log`, `time`, `log`, `entry` | CPU bring-up, interrupt entry, the kernel's own locks and console |
| `capabilities` types, bits and token | The capability check (rule 2); the representation changes to object capabilities |
| `ipc/nonos_inbox`, `ipc/kernel_ipc`, `ipc/nonos_channel` (compiled part) | IPC (rule 3); the representation changes to endpoints |
| `syscall` abi, caps, contract, numbers, types, validation, dispatch router, and the mechanism handlers in `syscall/microkernel` (IPC, memory, futex, device broker, IRQ, MMIO, DMA, PCI config, capability, kill, wait, process, time, debug) | The trap path and the check (rules 1 and 2); the handler set shrinks to the surface of [08-target-syscall-surface.md](08-target-syscall-surface.md) |
| `hardware/broker`: `confine`, `dma`, `irq` (x86_64), `mmio`, `pci`, `table`, `grant`, `holder`, `device`, `reserved`, `legacy`, `bootfb`, `pci_index` and the root files other than the scope tables | Device authority is enforced by IOMMU domains, interrupt routing and page tables (rule 1); [10-target-device-authority.md](10-target-device-authority.md) gives the reduced form |
| `drivers/pci` | Config-space transport, MSI and MSI-X programming and VMD bring-up are needed by the IOMMU and IRQ code above |
| `boot` (handoff, main, panic, stop, entry marker), `kernel_core/init`, `kernel_core/process_spawn` stacks | Boot (rule 4) |
| `security/hardening` (spectre mitigations, speculation barriers, secure zeroing), `observability`, `boot_session`, `image_ceiling`, `security/crypto/random*`; `crypto/util/rng`, `entropy`, `constant_time`, `crypto/random_api`, `crypto/core` | CPU state on every entry and exit; the boot nonce; the ceiling as a monotone bound the kernel enforces itself ([migration/spawn-gate.md](migration/spawn-gate.md)); the kernel's own randomness for KASLR and ASLR |
| `usercopy` | The kernel's only path into user memory |

### Stays in reduced form (R)

| Area | phys today | What stays | What leaves, and where |
|---|---:|---|---|
| `arch/x86_64/acpi` parser and tables, `arch/acpi_capsule*`, `hardware/broker/acpi`, `acpi_gpio`, `acpi_i2c`, `described`, `syscall/microkernel/acpi` | 18,488 | A fixed-table reader for what the kernel needs before any user thread runs: MADT (CPUs and interrupt controllers), MCFG (config space), DMAR and IVRS (IOMMU units), FADT reset and PM1 registers, HPET | Everything else to **acpi**: SRAT, SLIT, HMAT, SMBIOS, LPIT, the device description, OperationRegion policy, power-button, thermal and battery policy |
| `arch/x86_64/time` | 5,272 | TSC calibration and the timer the scheduler uses | RTC civil time to **libc** through the clock page; the dead PIT and HPET init paths deleted |
| `kernel_core/surface_registry`, `syscall/dispatch/router/surface_handlers` | 5,221 | Nothing of its own: frame sharing becomes the generic shared-memory object of [07-target-object-model.md](07-target-object-model.md) | Surfaces, rights, fences, vsync and the input ring to **display** and **input** |
| `process/foreign` | 4,842 | Delivery of a thread's unknown system call or fault to an endpoint, register read and write on a thread capability, mapping into an address space one holds a capability on | The Linux-specific parts (the seal, guest naming, row reads for windows) become userland or generic operations; [migration/signal-delivery.md](migration/signal-delivery.md) |
| `hardware/broker/claim` | 3,592 | The claim record, its epoch, and revocation on holder exit: bus master off, IOMMU detach, IRQ mask | Hold rules, scope check, reset, restore, wipe and scanout takeover sequencing to **devmgr** |
| `kernel_core/process_spawn/capsule_spawn` | 2,765 | Recording the measurement the kernel computed over the frames it mapped; enforcing the ceiling on the capability set a new process receives | Certificate, manifest and attestation checks to **verifier**; ELF loading to **loader**; spawn sequencing to **init** |
| `arch/x86_64/idt`, `bus` | 2,618 | One IDT (the live one is `src/interrupts/idt`); one PCI enumerator | The duplicate IDT deleted; BAR assignment and AER reporting to **devmgr** |
| `arch/x86_64/hypervisor`, `arch/hv_backdoor.rs`, `syscall/microkernel/hv_backdoor.rs` | 2,280 | Hypercall page and SynIC MSR setup | The VMware backdoor becomes a port-range capability held by the driver that needs it |
| `process/signal` | 2,057 | Terminating a process on a fault or a kill, with an exit code | Handler frames, sigaction storage and sigreturn deleted ([migration/signal-delivery.md](migration/signal-delivery.md)) |
| `memory/hardening`, `memory/mmu` protection bits | 2,193 | Guard-page checks on kernel faults; CR0, CR4 and EFER protection bits | Dead module-protection and mapping code deleted |
| `sys/clock` | 941 | The clock-page writer | Civil time and formatting to **libc** |
| `security/attest_registry` | 637 | The per-process measurement table the kernel writes at map time | The report built from it to **attest** |
| `security/zerostate` | 529 | The RAM wipe on shutdown and panic | Volume keep and leave rules to **volume** |
| `arch/hv_vmbus.rs`, `syscall/microkernel/hv_vmbus.rs` | 514 | SynIC message and event pages | Channel policy to the VMBus driver |

### Moves to userland (U)

| Destination | phys | From |
|---|---:|---|
| init | 37,627 | `userspace/` (init, 69 capsule mirrors, tool capsules), the 64 driver `*_capsule` mirrors less their clients, `fs/vfs_capsule` and `fs/ramfs_capsule` spawn and embed, `security/*_capsule` spawn and embed, the market client, `services/lifecycle`, spawn-instance, tool-run, app-launch, prove-job and Linux-run-net system calls |
| verifier | 14,884 | Spawn-time verification in `security/`, the verification crypto (Ed25519, P-256, ML-DSA-65, hashes, bigint, HMAC, PQClean support) and its boot self-test (`crypto/application/certification`, run from `src/kernel_core/init/entry/microkernel_init.rs:58`), `MkCapsuleVerify`, `MkLocalSign`, `MkLocalVerify`, local consent, dev and local root enrolment, `MkAttestPolicy` |
| volume | 11,729 | `fs/blockfs_volume`, `fs/blockfs`, `fs/cryptoblock`, ChaCha20-Poly1305, Argon2, the keyring client, `MkData*`, `MkUserObj*` |
| storage | 7,357 | `hardware/block_device`, the nvme, ahci, virtio_blk and usb_msc clients, `MkStoreRead`, `MkStoreWrite` |
| tpm | 6,155 | `security/tpm`, `MkDeviceSecret`, `MkEnroll`, `MkBootSlots` (and `CryptoMachineKey`, which moves with the crypto system calls) |
| loader | 4,155 | `elf/loader`, `elf/types`, `elf/errors`, `elf/aslr` |
| acpi | 3,851 | `arch/x86_64/acpi/aml`, `arch/x86_64/acpi/devices`, `arch/acpi_devices.rs`, battery |
| devmgr | 2,606 | `hardware/inventory`, `hardware/boot_census`, the broker's scope tables and census |
| attest | 2,013 | `security/attest_doc`, `security/capsule_measure`, `security/boot` (the verdict and boot slots), the attestation system calls |
| names | 1,481 | `services/registry`, `services/caps` |
| linux (via init) | 1,438 | `userspace/capsule_linux`: roles, tickets and terminal hand-off become init's spawn plan for the personality |
| procmgr | 945 | Process statistics, capability listings, log tail, terminal table, process stdout and stdin |
| settings, installer, display, input, platform | 1,778 | `sys/policy`; install sources and app install; the framebuffer blit; the input operations; the hypervisor report |

### Deleted (D)

| Group | phys | Why |
|---|---:|---|
| Dead today (no live caller) | 63,816 | Listed below |
| Inert today (reached, with no effect) | 766 | `src/context`, whose getter is called on every user copy and always returns `None` because no setter is ever called (`src/usercopy/walk/root.rs:28`, `src/context/current.rs:95`, `src/context/current.rs:106`); and the per-process descriptor table in `process/fd_table.rs`, `process_fd_table.rs` and `fd_types.rs`, which every PCB carries (`src/process/core/pcb.rs:107`) and only the dead `src/fs/api.rs` reads |
| Never compiled | 382 | `ipc/nonos_channel/bus.rs`, `channel.rs`, `stats.rs` are not declared in their `mod.rs` |
| Crypto system-call proxy | 1,094 | `syscall/dispatch/crypto`: a capsule that wants a hash or an AEAD calls the crypto capsule directly |
| Clients whose callers are deleted | 1,009 | The crypto and entropy capsule clients, used only by the proxy above |

The dead code, by grep, at `551bbd34`:

- **arch**: `x86_64/uefi`, `multiboot`, `smm`, `pci`, `serial`, `vga`, `watchdog`, `x86_64/api.rs`, `nonos_boot.rs` (11,755).
- **elf**: everything outside the live load path: `libmgr`, `got`, `symbol`, `hash`, `dynlink`, `reloc`, `embedded`, `cache`, `init`, `fini`, `interpreter`, `tls`, `auxv`, `stack`, `process`, `minimal` (7,304). They are deleted, and the proof bindings that point at them are rebound to the live relocator ([migration/elf-loading.md](migration/elf-loading.md), step 1).
- **fs**: the never-mounted VFS, ramfs, cryptofs, procfs (its load average moves to procmgr), fd tables, cache, path, pipe, storage, utils, the vfs and ramfs clients (12,709).
- **crypto**: P-384, RSA, Curve25519, Kyber, NTRU, Classic McEliece, SPHINCS+, the unvetted set, AES, AES-GCM, `application` apart from its self-test, base64, hardware acceleration stubs (8,405).
- **memory**: `boot_memory`, `region`, `safety`, `secure_memory`, `page_allocator` (whose only live function, `init`, calls `buddy_alloc::init`), `page_info`, the DMA allocator (7,828).
- **security**: `policy`, `quantum`, `monitoring`, most of `security/crypto`, `hardening/memory_encryption`, `boot/secure_boot`, `boot/firmware.rs`, `kernel_attest.rs`, `module_db.rs`, `periodic.rs` (4,969).
- **hardware and drivers**: the eight unused driver clients and protocols (e1000, xhci, hda, ps2_kbd, rtl8139, rtl8169, virtio_net, virtio_rng) (3,936) and `drivers/security` (879).
- **capabilities**: `audit`, `chain`, `delegation`, `multisig`, `resource`, `roles` (2,406).
- **process**: the third page-table walker in `address_space` and the Linux-style scheduling registry (1,357).
- **sys, ipc, kernel_core, boot, userspace**: `SeqLock`, `Semaphore`, `IrqRwLock`, `sys/gdt`, `ipc/pipe`, `boundary.rs`, `service.rs`, `spawn.rs`, `boot/init.rs`, `boot/vga`, the usb_hid client (2,268).

Each deletion is checked by building every profile after it ([13-migration-plan.md](13-migration-plan.md), phase 0).

## What the classification does not decide

- **The object model.** K code keeps its function and changes its representation: the 44-bit word becomes capabilities to objects, named inboxes become endpoints. That change is in [07-target-object-model.md](07-target-object-model.md).
- **Target sizes.** An R row's "what stays" is a design statement; the line budget it implies is in [13-migration-plan.md](13-migration-plan.md).
- **Other architectures.** The X trees (47,100 lines) follow the same rules; their own audit is out of scope while x86_64 is the only release target.

## Appendix: the classification table

Tab-separated: path prefix under `src/`, class, destination. Each file takes the row of its longest matching prefix. Under `src/hardware`, a `*_capsule` directory without a row is `U init`, and its `client/` and `protocol/` are `D` unless the capsule is nvme, ahci, virtio_blk or usb_msc, whose clients go to storage. Save the block as `classes.tsv` for `classify.py`.

```
# prefix	class	destination
# class: K stays in ring 0; R stays in ring 0 in reduced form; U moves to userland; D deleted; X not compiled for x86_64
# D destinations: dead (no live caller), inert (reached, with no effect), uncompiled, proxy, callers-deleted
lib.rs	K	-
nonos_main.rs	K	-
arch	K	-
arch/aarch64	X	-
arch/riscv64	X	-
arch/fdt	X	-
arch/iommu_window.rs	X	-
arch/nonos_boot.rs	D	dead
arch/acpi_capsule	R	acpi
arch/acpi_capsule.rs	R	acpi
arch/acpi_devices.rs	U	acpi
arch/hv_backdoor.rs	R	broker-pio
arch/hv_vmbus.rs	R	broker
arch/report_hypervisor.rs	U	platform
arch/x86_64/api.rs	D	dead
arch/x86_64/acpi	R	acpi
arch/x86_64/acpi/aml	U	acpi
arch/x86_64/acpi/devices	U	acpi
arch/x86_64/acpi/parser	R	acpi
arch/x86_64/acpi/tables	R	acpi
arch/x86_64/acpi/hw	K	-
arch/x86_64/hypervisor	R	broker-pio
arch/x86_64/idt	R	dup
arch/x86_64/multiboot	D	dead
arch/x86_64/pci	D	dead
arch/x86_64/serial	D	dead
arch/x86_64/smm	D	dead
arch/x86_64/uefi	D	dead
arch/x86_64/vga	D	dead
arch/x86_64/watchdog	D	dead
arch/x86_64/time	R	-
boot	K	-
boot/init.rs	D	dead
boot/vga	D	dead
bus	R	dup
capabilities	K	-
capabilities/audit	D	dead
capabilities/chain	D	dead
capabilities/delegation	D	dead
capabilities/multisig	D	dead
capabilities/resource	D	dead
capabilities/roles.rs	D	dead
context	D	inert
crypto	U	verifier
crypto/util/rng	K	-
crypto/util/entropy	K	-
crypto/util/constant_time	K	-
crypto/random_api	K	-
crypto/mod.rs	K	-
crypto/core	K	-
crypto/error.rs	K	-
crypto/asymmetric/p384	D	dead
crypto/asymmetric/rsa	D	dead
crypto/asymmetric/curve25519	D	dead
crypto/pqc/kyber.rs	D	dead
crypto/pqc/kyber_drawn.rs	D	dead
crypto/pqc/mceliece	D	dead
crypto/pqc/ntru	D	dead
crypto/pqc/sphincs	D	dead
crypto/pqc/quantum.rs	D	dead
crypto/pqc/unvetted	D	dead
crypto/pqc/unvetted_off.rs	D	dead
crypto/symmetric/aes	D	dead
crypto/symmetric/aes_gcm	D	dead
crypto/symmetric/chacha20poly1305	U	volume
crypto/util/argon2	U	volume
crypto/application	D	dead
crypto/application/certification	U	verifier
crypto/base64.rs	D	dead
crypto/hardware_accel.rs	D	dead
drivers	K	-
drivers/security	D	dead
elf	D	dead
elf/loader	U	loader
elf/types	U	loader
elf/errors	U	loader
elf/aslr	U	loader
elf/mod.rs	U	loader
entry	K	-
fs	D	dead
fs/blockfs	U	volume
fs/blockfs_volume	U	volume
fs/cryptoblock	U	volume
fs/vfs_capsule	U	init
fs/vfs_capsule/client	D	dead
fs/ramfs_capsule	U	init
fs/ramfs_capsule/client	D	dead
fs/ramfs_capsule/protocol	D	dead
hardware	K	-
hardware/broker/acpi	R	acpi
hardware/block_device	U	storage
hardware/inventory	U	devmgr
hardware/boot_census	U	devmgr
hardware/broker/claim	R	devmgr
hardware/broker/scope.rs	U	devmgr
hardware/broker/scope_hosts.rs	U	devmgr
hardware/broker/scope_ids.rs	U	devmgr
hardware/broker/scope_ids_nic.rs	U	devmgr
hardware/broker/scope_lpss.rs	U	devmgr
hardware/broker/scope_match.rs	U	devmgr
hardware/broker/scope_rows.rs	U	devmgr
hardware/broker/scope_table.rs	U	devmgr
hardware/broker/census	U	devmgr
hardware/broker/acpi_gpio	R	acpi
hardware/broker/acpi_i2c	R	acpi
hardware/broker/described	R	acpi
hardware/broker/irq/aarch64	X	-
hardware/broker/irq/riscv64	X	-
interrupts	K	-
ipc	K	-
ipc/pipe	D	dead
ipc/nonos_channel/bus.rs	D	uncompiled
ipc/nonos_channel/channel.rs	D	uncompiled
ipc/nonos_channel/stats.rs	D	uncompiled
kernel_core	K	-
kernel_core/boundary.rs	D	dead
kernel_core/service.rs	D	dead
kernel_core/spawn.rs	D	dead
kernel_core/process_spawn/capsule_spawn	R	verifier+loader+init
kernel_core/surface_registry	R	display
log	K	-
memory	K	-
memory/boot_memory	D	dead
memory/region	D	dead
memory/safety	D	dead
memory/secure_memory	D	dead
memory/page_allocator	D	dead
memory/page_info	D	dead
memory/dma	D	dead
memory/dma/coherency	K	-
memory/mmu/mmu	R	-
memory/hardening	R	-
process	K	-
process/foreign_absent.rs	X	-
process/foreign_absent_peer.rs	X	-
process/foreign_absent_query.rs	X	-
process/foreign_absent_seal.rs	X	-
process/foreign	R	generic-thread-and-space-caps
process/signal	R	linux+libc
process/scheduler/policy.rs	D	dead
process/scheduler/policy_types.rs	D	dead
process/address_space	D	dead
process/address_space/lifecycle	K	-
process/address_space/mod.rs	K	-
process/fd_table.rs	D	inert
process/process_fd_table.rs	D	inert
process/fd_types.rs	D	inert
process/operations_exec.rs	K	-
sched	K	-
security	K	-
security/tpm	U	tpm
security/attest_doc	U	attest
security/attest_policy	U	verifier
security/attest_registry	R	attest
security/admission_cache	U	verifier
security/capsule_attest	U	verifier
security/capsule_manifest	U	verifier
security/nonos_id_cert	U	verifier
security/nonos_trust_anchor	U	verifier
security/dev_roots	U	verifier
security/local_build	U	verifier
security/capsule_measure	U	attest
security/boot	U	attest
security/boot/secure_boot	D	dead
security/boot/firmware.rs	D	dead
security/crypto	D	dead
security/crypto/random.rs	K	-
security/crypto/random_fill.rs	K	-
security/crypto_capsule	U	init
security/crypto_capsule/client	D	callers-deleted
security/entropy_capsule	U	init
security/entropy_capsule/client	D	callers-deleted
security/keyring_capsule	U	init
security/keyring_capsule/client	U	volume
security/keyring_capsule/protocol	U	volume
security/market_capsule	U	init
security/market_capsule/client	U	init
security/market_capsule/protocol	U	init
security/hardening/memory_encryption	D	dead
security/policy	D	dead
security/quantum	D	dead
security/monitoring	D	dead
security/periodic.rs	D	dead
security/module_db.rs	D	dead
security/kernel_attest.rs	D	dead
security/zerostate	R	volume
services	U	names
services/lifecycle	U	init
smp	K	-
sys	K	-
sys/gdt	D	dead
sys/sync/irq_rwlock	D	dead
sys/sync/semaphore	D	dead
sys/sync/seqlock	D	dead
sys/policy	U	settings
sys/clock	R	libc
syscall	K	-
syscall/dispatch/crypto	D	proxy
syscall/dispatch/router/graphics_present	U	display
syscall/dispatch/router/surface_handlers	R	display
syscall/dispatch/router/input_ops	U	input
syscall/microkernel/acpi	R	acpi
syscall/microkernel/data	U	volume
syscall/microkernel/store_read.rs	U	storage
syscall/microkernel/store_write.rs	U	storage
syscall/microkernel/store_errno.rs	U	storage
syscall/microkernel/capsule_load	U	init
syscall/microkernel/capsule_verify	U	verifier
syscall/microkernel/local_sign.rs	U	verifier
syscall/microkernel/local_verify.rs	U	verifier
syscall/microkernel/local_consent.rs	U	verifier
syscall/microkernel/local_image	U	verifier
syscall/microkernel/local_image.rs	U	verifier
syscall/microkernel/enrol_dev_root.rs	U	verifier
syscall/microkernel/enrol_local_root.rs	U	verifier
syscall/microkernel/attest.rs	U	attest
syscall/microkernel/attest_doc.rs	U	attest
syscall/microkernel/attest_entries.rs	U	attest
syscall/microkernel/attest_policy.rs	U	verifier
syscall/microkernel/boot_attest.rs	U	attest
syscall/microkernel/device_proof	U	tpm
syscall/microkernel/prove_job.rs	U	init
syscall/microkernel/install_source.rs	U	installer
syscall/microkernel/install_source_approval.rs	U	installer
syscall/microkernel/install_source_in_use.rs	U	installer
syscall/microkernel/install_source_kinds.rs	U	installer
syscall/microkernel/install_source_modules.rs	U	installer
syscall/microkernel/install_source_records.rs	U	installer
syscall/microkernel/install_source_window.rs	U	installer
syscall/microkernel/app_install.rs	U	installer
syscall/microkernel/app_install_detail.rs	U	installer
syscall/microkernel/app_install_status.rs	U	installer
syscall/microkernel/app_launch.rs	U	init
syscall/microkernel/app_launch_rule.rs	U	init
syscall/microkernel/app_launch_take.rs	U	init
syscall/microkernel/app_launch_word.rs	U	init
syscall/microkernel/app_uninstall.rs	U	installer
syscall/microkernel/spawn_instance.rs	U	init
syscall/microkernel/tool_run.rs	U	init
syscall/microkernel/linux_run_net.rs	U	init
syscall/microkernel/boot_census.rs	U	devmgr
syscall/microkernel/battery	U	acpi
syscall/microkernel/battery.rs	U	acpi
syscall/microkernel/procstat.rs	U	procmgr
syscall/microkernel/procstat_boot.rs	U	procmgr
syscall/microkernel/procstat_codes.rs	U	procmgr
syscall/microkernel/procstat_entry.rs	U	procmgr
syscall/microkernel/procstat_fill.rs	U	procmgr
syscall/microkernel/procstat_header.rs	U	procmgr
syscall/microkernel/procstat_header_fill.rs	U	procmgr
syscall/microkernel/procstat_redact.rs	U	procmgr
syscall/microkernel/caps_of	U	procmgr
syscall/microkernel/log_tail.rs	U	procmgr
syscall/microkernel/tty.rs	U	procmgr
syscall/microkernel/tty_table.rs	U	procmgr
syscall/microkernel/proc_output.rs	U	procmgr
syscall/microkernel/proc_stdin	U	procmgr
syscall/microkernel/proc_stdin.rs	U	procmgr
syscall/microkernel/private_write.rs	U	procmgr
syscall/microkernel/hv_backdoor.rs	R	broker-pio
syscall/microkernel/hv_vmbus.rs	R	broker
time	K	-
usercopy	K	-
userspace	U	init
userspace/capsule_linux	U	init+linux
userspace/capsule_driver_usb_hid/client	D	dead
userspace/capsule_driver_usb_hid/protocol	D	dead
syscall/microkernel/pio/unsupported.rs	X	-
arch/interrupt_controller/ipi_dispatch/aarch64.rs	X	-
arch/iommu_backend/smmu.rs	X	-
arch/user_access/pan.rs	X	-
arch/trap/contract/backend_aarch64	X	-
arch/trap/contract/backend_riscv64	X	-
arch/port_io/window.rs	X	-
boot/handoff/kernel_handoff/aarch64	X	-
boot/handoff/kernel_handoff/riscv64	X	-
fs/procfs/cpuinfo/aarch64.rs	X	-
smp/topology/detection/probe_aarch64.rs	X	-
hardware/broker/pio_absent.rs	X	-
hardware/broker/pio_absent_claim.rs	X	-
```
