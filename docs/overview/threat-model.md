# Threat model

What NONOS 0.9.2 protects, from whom, what it relies on, and what it leaves out; [Protections and limits](../security/protections-and-limits.md) gives the same answer feature by feature.

## Assets

| Asset | Where it lives | What guards it |
|---|---|---|
| Files a person keeps | the [data volume](glossary.md#data-volume) on an installed disk | every sector sealed, under a TPM-derived key or a passphrase |
| What a live boot holds | RAM | the [ZeroState](glossary.md#zerostate) wipe at shutdown and restart |
| The wallet's key | the keyring capsule | the keyring's vault gate |
| Where this machine is on the network | its IP address and its cards' station addresses | the chosen anonymity network for the system's own connections, and a fresh station address at each bring-up |
| What runs | the kernel image and every capsule | signatures, attestation and the [capability word](glossary.md#capability-word) |
| The device secret | derived from the TPM | the `DeviceSecret` bit, held by one capsule |

The sections below give the code behind each guard.
