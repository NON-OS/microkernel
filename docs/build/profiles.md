# Profiles

Pick the kind of image to build here: the six build profiles, the keys of `nonos.toml`, the rules that refuse a weaker image, and how a build profile differs from a boot mode.

## Choose a profile

[nonos.toml](../../nonos.toml) is the configuration of the one build, and every key in it is optional. `nix build` builds what it says; `nix build .#<profile>` builds another [build profile](../overview/glossary.md#build-profile) with the rest of the file unchanged (`profile`, `nonos.toml:1-17`).

```
make profiles
make PROFILE=airgapped
nix build .#core
```

Not tested in this release.

`make profiles` prints, for each profile, what it is, who it is for, its privacy posture and its loader policy (`describe`, `tools/nix/config.nix:193-202`).

## The six profiles

| profile | kernel features it starts from | loader policy | takes out | for |
|---|---|---|---|---|
| `standard`, `tools/nix/config.nix:70-77` | `microkernel-full-gui` | `standard` | nothing | a person's own machine, every day |
| `hardened`, `tools/nix/config.nix:78-85` | `microkernel-full-gui` | `production` | `capsule-serial-debug` | a machine that may be seized or tampered with |
| `airgapped`, `tools/nix/config.nix:86-93` | `microkernel-full-gui` | `production` | `capsule-serial-debug` and every network feature | keys and documents that must never touch a network |
| `qemu`, `tools/nix/config.nix:94-101` | `microkernel-desktop-gui`, `microkernel-setup-wizard` | `standard-qemu` | nothing | trying NONOS in a virtual machine, and CI's boot smoke test |
| `dev`, `tools/nix/config.nix:102-109` | the qemu set and `nonos-dev-attest` | `dev-qemu` | nothing | working on NONOS itself |
| `core`, `tools/nix/config.nix:110-117` | `microkernel-capsules` | `standard-qemu` | nothing | kernel work and the smallest image that boots |

`microkernel-full-gui`, the set `make` builds by default, is the desktop with every production hardware driver [capsule](../overview/glossary.md#capsule), the market and first-boot setup (`Cargo.toml:627-646`). `microkernel-capsules`, the set of the `core` profile, is the core kernel with three capsules embedded: proof I/O, the RAM file system and the keyring (`Cargo.toml:251-257`).

`capsule-serial-debug` lets service capsules write to the serial console, which the hardened and airgapped images must not allow (`debugFeatures`, `tools/nix/config.nix:60-62`). The network features the airgapped profile removes are everything that reaches a network: the drivers, the stack, and the programs whose only job is to go online, such as the browser, the market and the model fetcher (`networkFeatures`, `tools/nix/config.nix:48-58`).

The profile's own description says the `production` loader refuses to start without Secure Boot and a TPM to measure into (`privacy`, `tools/nix/config.nix:78-85`). The build only selects that policy, by building the loader with the cargo feature of the same name (`cargo`, `tools/nix/image.nix:161-162`); the refusal itself is in the loader's verification code and is not checked on this page.
