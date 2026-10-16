# FAQ: Audits and proofs

Part of [FAQ](faq.md).

## Audits and proofs

This release claims no independent security audit. What it has is [proof
crates](glossary.md#proof-crate) that test the shipped kernel and capsule source
on the host, run by `nix flake check`; Lean 4 models in `verification/lean/`;
and a list of everything NONOS trusts without proof,
[verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md). [Design
principles](design-principles-proofs-live-next.md#proofs-live-next-to-the-code)
gives the results on this commit, failures included, and [Tests and
proofs](../contributing/tests-and-proofs.md) says how to run them.

## The licence

NONOS is free software under the GNU Affero General Public License, version 3
([LICENSE](../../LICENSE)). Each source file's header adds "or (at your option)
any later version". Vendored code under `third_party/` keeps its own licence, as
in `third_party/minimp3/LICENSE`, and device firmware under
`nonos-bootloader/firmware/` keeps its vendor's terms, as in
`nonos-bootloader/firmware/realtek/LICENSE`.

## Contributing

Start with [CONTRIBUTING.md](../../CONTRIBUTING.md) and
[Contributing](../contributing/README.md). The build needs Nix with flakes
turned on, plus `git` and GNU `make`; every compiler and tool comes from the
flake ([Toolchain](../build/toolchain.md)). [Build](../build/README.md) and
[Make targets](../build/make-targets.md) give the commands, and
[Review](../contributing/review.md) says what a change is held to. To write your
first program, [Writing an app](../userland/writing-an-app.md) builds a windowed
app step by step.

## Reporting a security problem

Report it privately, not in a public issue. [SECURITY.md](../../SECURITY.md) and
[Reporting a vulnerability](../security/reporting-a-vulnerability.md) say how.

## Where this comes from

The source behind the answers above, at the commit in the footer.

- Running Linux programs
  - The store's Linux tools: `linux` in `tools/nix/store.json:100`.
- Hardware NONOS runs on
  - The RTL8821CE id: `PCI_DEVICE_RTL8821CE` in
    `userland/capsule_driver_rtl8821ce/src/constants/mod.rs:26`.
  - The Intel LPSS I2C ids, even ones only: `device_info` in
    `userland/capsule_driver_i2c_pci/src/constants/device_info.rs:29-38`.
  - The HD Audio classes and the SST exception: `is_candidate` in
    `userland/capsule_driver_hda/src/discover/candidate.rs:24-34`.
- Trying NONOS in a virtual machine
  - What each profile carries: `profiles` in `tools/nix/config.nix:69-117`.
- What happens at shutdown
  - The Power key notice: `POWER_OFF_UNAVAILABLE` in
    `userland/capsule_desktop_shell/src/state/system_key.rs:42-47`.
- Without a TPM
  - The Hardened entry's text: `Hardened` in
    `nonos-bootloader/src/bootmenu/entries.rs:41-43`.
  - The modes that need a TPM: `requires_tpm` in
    `nonos-bootloader/src/menu/types/mode.rs:57-58`.
  - The volume key comes from the TPM: `derive_for_kernel` in
    `src/fs/blockfs_volume/open_machine.rs:19-35`.
  - The passphrase call, which no capsule calls: `mk_data_volume_passphrase` in
    `userland/libc/src/data.rs:53`.
  - The keyring answers `ENOENT` without a TPM:
    `userland/capsule_keyring/src/server/handlers/vault_seal.rs:57-61`.
