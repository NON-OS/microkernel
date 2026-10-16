# Manifests and capabilities: How the word is fixed, continued

Part of [Manifests and capabilities](manifests-and-capabilities.md).

## How the word is fixed, continued

One more ceiling exists, and the spawn gate does not enforce it. An image the
flake builds carries the union of the ceilings of the capsules its profile ships
(`tools/nix/image.nix:58-63`, `ceilingOf`). A word outside it is written to the
serial log as `[CEILING] not enforced, would refuse` with the extra bits named,
and the capsule still starts with that word
(`src/security/image_ceiling/admits.rs:53-61`, `would_refuse`).

The arithmetic of the three bit tests is mounted by `mechanism_proofs`
(`userland/mechanism_proofs/src/spawn/mod.rs:20-21`, `caps_bits`), whose 56 host
tests passed in the flake checks on this commit.


## See also

- [Signing and publisher keys](signing-and-publisher-keys.md)
- [Shipping an app](shipping-an-app.md)
- [Capabilities in the kernel](../kernel/capabilities.md)
- [ABI: capabilities](../abi/capabilities.md)
- [Capsule isolation](../security/capsule-isolation.md)
- [Userland](README.md)
