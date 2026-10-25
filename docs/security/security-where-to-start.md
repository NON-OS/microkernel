# Security: Where to start

Part of [Security](README.md).

## Where to start

A security reviewer: [Protections and limits](protections-and-limits.md), then
[Capsule isolation](capsule-isolation.md), then the boot chain pages. [Checking
the security claims yourself](checking-the-claims.md) says how to run the
checks.

A person deciding whether to keep data on NONOS: [Device secrets and
keys](device-secrets-and-keys.md), then the "does not address" table in
[Protections and limits](protections-and-limits.md).

A person who wants to know what the network sees: [Privacy
networks](../using/privacy-network.md), then [How the Nym and Anyone transports
are built](anonymity-transports.md) and [TLS and certificate
trust](tls-and-certificates.md).

A driver or capsule author: [Capsule isolation](capsule-isolation.md), then
[Manifests and capabilities](../userland/manifests-and-capabilities.md).

Someone who found a security bug: [Reporting a
vulnerability](reporting-a-vulnerability.md).

## See also

- [Threat model](../overview/threat-model.md)
- [Architecture](../overview/architecture.md)
- [Capabilities in the kernel](../kernel/capabilities.md)
- [The capability bits, as published](../../abi/caps.toml)
- [Security policy](../../SECURITY.md)
