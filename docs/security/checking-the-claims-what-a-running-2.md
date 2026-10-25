# Checking the security claims yourself: What a running machine shows about itself, continued

Part of [Checking the security claims yourself](checking-the-claims.md).

## What a running machine shows about itself, continued

In the Terminal, `log [word ...]` prints the serial console lines the kernel
keeps in memory: the newest 200, or every line that names one of the words,
ignoring case (`NEWEST`,
`userland/capsule_terminal/src/command/builtin/log.rs:29-55`). The kernel keeps
the first 64 KiB of the boot and the last 64 KiB (`HEAD` and `CAPACITY`,
`src/sys/serial/tail.rs:27-32`), and only in an image built with
`capsule-serial-debug`, which the hardened and airgapped [build
profiles](../overview/glossary-boot-root-record.md#build-profile) drop
(`debugFeatures`, `tools/nix/config.nix:84`, `tools/nix/config.nix:92`). The
call needs `AttestRead` (`can_attest_read`,
`src/syscall/contract/cap_table/mk.rs:58`), which the Terminal holds
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_terminal/Capsule.mk:17-22`). The
spawn gate writes one line for each capsule whose trailer it checks:
`[ZK-ATTEST] ok <name> <authority>`, `[ZK-ATTEST] FAIL <name>: <reason>` or
`[ZK-ATTEST] none <name>` (`print`,
`src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:28-59`).

```sh
log zk-attest
```

Not tested in this release.

`receipt` prints what the kernel recorded about every running capsule, read with
`mk_attest_entries`
(`userland/capsule_terminal/src/command/builtin/receipt/run.rs:36`), which needs
`AttestRead` too (`src/syscall/contract/cap_table/mk.rs:57`).
[Terminal](../using/terminal.md) has the other commands.


## See also

- [Protections and limits](protections-and-limits.md)
- [STARK attestation](stark-attestation.md)
- [Capsule isolation](capsule-isolation.md)
- [Threat model](../overview/threat-model.md)
- [CI](../build/ci.md)
- [Make targets](../build/make-targets.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
- [Kernel logging](../kernel/logging.md)
- [The Linux personality](../userland/linux-personality.md)
