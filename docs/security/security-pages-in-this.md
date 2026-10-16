# Security: Pages in this section

Part of [Security](README.md).

## Pages in this section

Boot and admission:

- [Boot chain and signatures](boot-chain-and-signatures.md): how the kernel and
  capsules are signed with Ed25519 and ML-DSA, and who checks each signature.
- [Rollback protection](rollback-protection.md): how an older, signed image is
  refused.
- [STARK attestation](stark-attestation.md): the proofs a capsule, the kernel
  and the bootloader carry, and the anonymous device proof.
- [Measured boot and the TPM](measured-boot-and-tpm.md): which PCRs are
  extended, the kernel's check of the bootloader, the machine key and the
  attestation key.

The running system:

- [Capsule isolation](capsule-isolation.md): address spaces, the capability
  check on every NONOS system call, IPC rules, driver confinement and the Linux
  sandbox.
- [Device secrets and keys](device-secrets-and-keys.md): the keyring, sealed
  records, the encrypted data volume, what stays in RAM and what is wiped at
  shutdown.
- [Randomness and cryptography](randomness-and-cryptography.md): where random
  bytes come from, the two generators, the crypto system calls and which code
  runs which algorithm.

The network:

- [TLS and certificate trust](tls-and-certificates.md): the TLS 1.3 client
  capsules share, how a certificate chain is checked, where the roots come from
  and which clock dates are read against.
- [How the Nym and Anyone transports are built](anonymity-transports.md): the
  two anonymity transports, the cryptography on their wire, how each checks its
  directory, and what they lack.

Limits, checks and reports:

- [Protections and limits](protections-and-limits.md): the threats NONOS
  addresses and the ones it does not.
- [Checking the security claims yourself](checking-the-claims.md): the tools in
  the tree that test these claims, how to run them and what they printed at this
  commit.
- [Reporting a vulnerability](reporting-a-vulnerability.md): how to report a
  security bug privately.

`capabilities-and-tokens.md` in this directory is an older capability page. It
stays because `scripts/check_docs_caps.py` reads its bit table (`PAGE`,
`scripts/check_docs_caps.py:39`), and its prose is out of date: it says
`MkCapGrant` needs `IPC`, where the kernel asks for `Admin` (`MkCapGrant`,
`src/syscall/contract/cap_table/mk_ipc.rs:50`), and it links pages that do not
exist. Use [Capsule isolation](capsule-isolation.md) and [ABI
capabilities](../abi/capabilities.md).
