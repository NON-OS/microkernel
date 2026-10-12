# Protections and limits: Checking a running machine

Part of [Protections and limits](protections-and-limits.md).

## Checking a running machine

The Terminal's `log` command reads the copy of the serial console that the
kernel keeps in memory, through `mk_log_tail`, which needs `AttestRead`
(`userland/capsule_terminal/src/command/builtin/log.rs:32-40`). Only a standard,
qemu or dev image keeps that copy; on a hardened or air-gapped image the kernel
keeps no copy, so `log` finds no lines (`keep` in
`src/sys/serial/tail.rs:51-54`).

```sh
log iommu vt-d
```

Not tested in this release.

It shows whether the DMA of claimed devices is confined: look for a `[VT-D]
pid=` line for each claimed PCI device that says `confined to its capsule's
domain`, and for `enforcing=1` and `unconfined grants=0` on the last `[IOMMU]
<vendor> present` line (`posture_line` in `src/memory/iommu/posture.rs:69-79`).
A device found at boot that no driver claimed still reaches all memory through
the identity domain, whatever these lines say.

Capability refusals are not visible anywhere in this release. The `[CAP-DENY]`
lines go through the kernel's structured `log`, and nothing in the kernel calls
its `init` (`src/log/manager/api.rs:26-53`), so they are dropped;
[Logging](../kernel/logging.md) explains.

The Security page in Settings shows whether the TPM gave a machine key in this
boot state. See [Device secrets and keys](device-secrets-and-keys.md).

The tools in the tree that test these claims, from the capability audit to the
tamper tests, are on [Checking the security claims
yourself](checking-the-claims.md), with what each printed at this commit.

## See also

- [Threat model](../overview/threat-model.md)
- [Capsule isolation](capsule-isolation.md)
- [Device secrets and keys](device-secrets-and-keys.md)
- [Measured boot and the TPM](measured-boot-and-tpm.md)
- [Rollback protection](rollback-protection.md)
- [TLS and certificate trust](tls-and-certificates.md)
- [How the Nym and Anyone transports are built](anonymity-transports.md)
- [Checking the security claims yourself](checking-the-claims.md)
- [Reporting a vulnerability](reporting-a-vulnerability.md)
