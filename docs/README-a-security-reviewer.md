# NONOS documentation: A security reviewer

Part of [NONOS documentation](README.md).

### A security reviewer

You want to know what NONOS claims, what holds each claim, and where the claims
stop.

1. [Threat model](overview/threat-model.md), then
   [Security](security/README.md).
2. [Protections and limits](security/protections-and-limits.md): what NONOS
   protects against with the code that does it, and the known gaps with their
   reasons.
3. [Boot chain and signatures](security/boot-chain-and-signatures.md), [STARK
   attestation](security/stark-attestation.md), [Rollback
   protection](security/rollback-protection.md) and [Measured boot and the
   TPM](security/measured-boot-and-tpm.md).
4. [Capsule isolation](security/capsule-isolation.md), [Device secrets and
   keys](security/device-secrets-and-keys.md) and [Randomness and
   cryptography](security/randomness-and-cryptography.md).
5. [TLS and certificate trust](security/tls-and-certificates.md) and [How the
   Nym and Anyone transports are built](security/anonymity-transports.md) for
   what leaves the machine.
6. [Capabilities](kernel/capabilities.md), [IOMMU](kernel/iommu.md) and [Signing
   and publisher keys](userland/signing-and-publisher-keys.md) for the
   mechanisms underneath.
7. [Checking the security claims yourself](security/checking-the-claims.md) to
   run the tools that test these claims.
8. [Reproducible builds](build/reproducible-builds.md) and [SBOM](build/sbom.md)
   for the supply chain.
9. [Reporting a vulnerability](security/reporting-a-vulnerability.md) when you
   find something.

### A driver writer

You want a device to work under NONOS.

1. [Drivers](drivers/README.md): how a device is found, how its driver starts,
   and how it is confined.
2. [The hardware broker](kernel/hardware-broker.md) for the kernel side, then
   [Broker API](drivers/broker-api.md) for the driver side.
3. [Writing a driver](drivers/writing-a-driver.md): a whole driver capsule, step
   by step.
4. [Broker](abi/broker.md) in the ABI section for the records and constants, and
   [IOMMU](kernel/iommu.md) for DMA.
5. [Tests and proofs](contributing/tests-and-proofs.md): what a proof crate is,
   and why a new driver ships with one.
6. [Hardware support matrix](hardware/MATRIX.md) and [Reporting a
   machine](hardware/report.md).

### A contributor

You want to send a change.

1. [CONTRIBUTING.md](../CONTRIBUTING.md), then [Contributing to
   NONOS](contributing/README.md).
2. [Build NONOS](build/README.md) and [Toolchain](build/toolchain.md), with
   [Host tools](build/host-tools.md) for the programs under `tools/`.
3. [Code style](contributing/code-style.md) and
   [Commits](contributing/commits.md).
4. [Tests and proofs](contributing/tests-and-proofs.md), [CI](build/ci.md) and
   [Review](contributing/review.md).

## Every page
