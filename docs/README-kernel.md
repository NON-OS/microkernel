# NONOS documentation: Kernel

Part of [NONOS documentation](README.md).

### Kernel

- [The kernel](kernel/README.md): what runs in ring 0, what is left to capsules,
  and where each part lives in `src/`.
- [Boot handoff and kernel init](kernel/boot-handoff.md): how the loader hands
  over, what the kernel checks, and its start order.
- [Memory and paging](kernel/memory-and-paging.md): the address space, page
  tables, hardware protections and user copies.
- [Frame allocator](kernel/frame-allocator.md): which physical memory the kernel
  uses and how it hands out frames.
- [Scheduler and SMP](kernel/scheduler-and-smp.md): starting every CPU, per CPU
  state, and picking the next process.
- [Futex](kernel/futex.md): the two calls that let threads sleep on a word and
  wake each other.
- [Timers and clocks](kernel/timers.md): the counters the kernel reads, the
  scheduler tick and the time calls.
- [Kernel logging](kernel/logging.md): where kernel messages go, their tags, and
  how to read them.
- [Panic and boot stop](kernel/panic-and-boot-stop.md): what the kernel does
  when it cannot go on, and what you can do next.
- [System calls](kernel/syscalls.md): how a call enters the kernel, the
  dispatch, the capability check and the count.
- [Capabilities](kernel/capabilities.md): the capability bits, each process's
  token, the check, and grant and revoke.
- [IPC](kernel/ipc.md): endpoints, inboxes, size limits, who may send to whom,
  and timeouts.
- [Processes and capsule spawn](kernel/processes-and-spawn.md): the spawn gate,
  the ELF loader, stacks, and process exit.
- [The hardware broker](kernel/hardware-broker.md): the device table, claims,
  register windows, DMA, interrupts and release.
- [IOMMU](kernel/iommu.md): which devices the kernel confines with DMA
  remapping, and what happens when it cannot.
- [PCI and ACPI](kernel/pci-and-acpi.md): the ACPI tables the kernel reads and
  how it scans PCI on x86_64.

### Security

- [Security](security/README.md): what NONOS trusts, where each check lives, and
  which page answers which question.
- [Protections and limits](security/protections-and-limits.md): what NONOS
  protects against with the code that does it, and what it does not.
- [Boot chain and signatures](security/boot-chain-and-signatures.md): the checks
  at each stage, from the firmware to every capsule.
- [STARK attestation](security/stark-attestation.md): how a kernel, a loader or
  a capsule proves it belongs to the committed set.
- [Rollback protection](security/rollback-protection.md): the floors that keep
  older signed kernels, loaders and certificates out.
- [Measured boot and the TPM](security/measured-boot-and-tpm.md): what the TPM
  records and derives, and what is lost without one.
- [Capsule isolation](security/capsule-isolation.md): address spaces, the
  syscall check, send rules, confined drivers and the Linux sandbox.
- [Device secrets and keys](security/device-secrets-and-keys.md): where secrets
  live, what protects each, and what shutdown wipes.
- [Randomness and cryptography](security/randomness-and-cryptography.md): where
  random bytes come from, the two generators, the crypto system calls and which
  code runs which algorithm.
- [TLS and certificate trust](security/tls-and-certificates.md): the TLS 1.3
  client the capsules share, the chain check, where the roots come from and
  which clock the dates are read against.
- [How the Nym and Anyone transports are
  built](security/anonymity-transports.md): what `net.nym` and `net.anon`
  implement of each protocol, how each checks its directory, and what they lack.
- [Checking the security claims yourself](security/checking-the-claims.md): the
  tools in the tree that test the security claims, the command for each, and
  what a pass and a failure print.
- [Reporting a vulnerability](security/reporting-a-vulnerability.md): how to
  report a security bug in private, and what to include.
