# Release rehearsal: standard sealed with throwaway keys, booted with a TPM

main 8bf24f59b, as a git-archive copy in its own repository, with every key
made fresh inside it (setup-signing-key.sh, scratch-trust-bootstrap.sh in
keys-only mode, a throwaway market operator key). No production key was used
or read; nothing from the copy was committed but this evidence.
`nix run .#seal -- --profile standard` with that copy's signing key.

## The seal (`seal.log`)

- 157 capsules enrolled (STARK membership proofs), then kept on the rerun:
  "every trailer passes the spawn gate's check for these exact capsules".
- [6/6] verify: [A] trust ledger, [B] manifest signatures Ed25519 and
  ML-DSA-65 under the trust anchor policy, [C] each binary's capabilities
  against its signed manifest, [D] every STARK membership proof by the gate
  the kernel and loader run, [E] the embedded capsules against the signed
  ones, [F] the sealed kernel and loader against a fresh nix build: all pass.
  "sealed standard". (Two earlier [F] runs stopped on a full disk.)
- `image.sha`: the sealed image; `build-receipt.json` its receipt.

## The boot (`serial.log`, `qemu.txt`)

`nix run .#qemu -- --profile standard --tpm --lane iommu=intel --mem 4G
--fresh`: swtpm behind tpm-crb, Intel IOMMU. Setup walked by keystroke.

- `[TPM] interface CRB at 0xFED40000, locality 0 (id 0x25C11, ACPI agrees)`.
- 75 `[SPAWN]`, 65 `[ZK-ATTEST] ok`; no capsule failed, exited or faulted.
- `desktop.png`, `desktop-5min.png`: the desktop, menu bar and dock, live.

## Not yet

hardened, airgapped, qemu and core are not sealed; the hardened and
airgapped loader refusals (no TPM, unsigned or tampered capsule) are not run.
