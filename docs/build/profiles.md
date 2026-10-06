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

## The keys of `nonos.toml`

| key | default | what it does |
|---|---|---|
| `profile` | `standard` | the kind of image (`profile`, `nonos.toml:17`) |
| `smp` | `true` | brings up every CPU, through the `nonos-smp` feature (`smp`, `nonos.toml:19-20`) |
| `install` | `true` | keeps first-boot setup and the installer; `false` takes both out (`install`, `nonos.toml:22-25`) |
| `rollback_index` | `1` | the [rollback index](../overview/glossary.md#rollback-index) bound into the signed kernel (`rollback_index`, `nonos.toml:27-29`) |
| `features` | none | extra kernel features, checked against `Cargo.toml` (`features`, `nonos.toml:31-32`) |
| `loader` | the profile's | a stricter loader policy than the profile's own (`loader`, `nonos.toml:34-36`) |
| `linux_packages` | empty | the package mirror, as `name:port`, that the in-tree Linux tools the image does not carry install from (`linux_packages`, `nonos.toml:38-44`) |
| `store.linux`, `store.media` | `true`, `true` | whether the package store carries the Linux tools and the Qwen runner, and the sample films (`store`, `nonos.toml:46-49`) |

The defaults live in the flake as well, and a key the flake does not know fails the evaluation (`defaults`, `tools/nix/config.nix:120-134`).

`nonos.toml` states that every boot is amnesic: nothing is kept unless the person chooses to install in first-boot setup. With `install = false` the image has no first-boot setup and no installer, so no boot of it can keep anything or write a disk (`install`, `nonos.toml:22-25`). Such a build takes the name suffix `-live`, and a development twin takes `-dev` (`name`, `tools/nix/config.nix:184`).

## How a profile resolves

The flake turns a profile into kernel features in one place (`resolve`, `tools/nix/config.nix:136-188`):

1. Start from the profile's `kernel` features.
2. Add `nonos-smp` when `smp` is true, and the extra `features`.
3. Add `nonos-dev-attest` for a development twin, and `nonos-release` when the loader is not `dev-qemu`.
4. Take out everything the profile drops, and with `install = false` the setup and installer features too, replacing any feature that would bring one back by its own members.

The features come from `Cargo.toml`, never from a hand list, so what a profile takes out is not in the kernel binary at all (`kernelFeatures`, `tools/nix/config.nix:4-15`).

## The floors

A profile sets floors, not defaults. A `nonos.toml` that asks a profile for less fails to evaluate, before anything builds, with a message that starts with `nonos.toml:` (`assertMsg`, `tools/nix/config.nix:157-172`). It fails when:

- a key is unknown, or a feature is not in `Cargo.toml`;
- a feature the profile takes out comes back in through another feature;
- the loader is weaker than the profile's, in the order `dev-qemu`, `standard-qemu`, `standard`, `production` (`loaders`, `tools/nix/config.nix:66`);
- `rollback_index` is not an integer of at least 1, or `linux_packages` is not `name:port`;
- `nonos-dev-attest`, which admits capsules without their STARK proof, or the wallet test vectors, would reach an image whose loader is not `dev-qemu`.

For example, this `nonos.toml` fails with `nonos.toml: profile hardened needs at least loader production, not standard` (`level`, `tools/nix/config.nix:160`):

```
profile = "hardened"
loader = "standard"
```

When a feature the kernel tests by name pulls back something the profile takes out, the flake still evaluates, but the kernel refuses to build and says which features are tangled and that the profile builds only with `install = false` (`blocked`, `tools/nix/config.nix:178-182`, `refuse`, `tools/nix/image.nix:78-88`).

The `kernel-profile-<profile>` checks type-check the kernel with exactly each profile's features (`profileChecks`, `tools/nix/checks.nix:169-193`), and the build receipt reads the kernel's bytes to confirm that no capsule a profile takes out is inside it (`enforcement`, `tools/nonos-receipt:103-126`).

## The image capability ceiling

Each build writes the OR of the capability ceilings of the capsules its profile ships into the trust policy the kernel embeds (`ceilingOf`, `tools/nix/image.nix:59-64`), and the kernel bakes that value in (`BAKED`, `src/security/image_ceiling/value.rs:19-21`). In this release the ceiling is not enforced. Capsule spawn calls only `would_refuse`, which prints `[CEILING] not enforced, would refuse` and the capsule's name when a capsule asks for more, then lets the spawn go on (`would_refuse`, `src/security/image_ceiling/admits.rs:53-60`, `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:68`). Until it is, the image-wide limit adds nothing at spawn. [Capabilities](../kernel/capabilities.md) describes the bits.

## Build profiles and boot modes

A build profile decides what an image can ever do. A [boot mode](../overview/glossary.md#boot-mode) is chosen in the boot menu at each boot and narrows it further: Standard, Hardened, Safe Mode, Air-Gapped or Recovery (`BootProfile`, `src/boot/handoff/api/profile.rs:24-42`). The flake's own comments call the boot menu a run-time posture that any image offers (`tools/nix/config.nix`). A kernel started with no handoff from the loader runs Standard (`boot_profile`, `src/boot/handoff/api/profile.rs:60-63`).

| boot mode | network | what the kernel does differently |
|---|---|---|
| Standard | yes | nothing |
| Hardened | yes | nothing in the kernel's network rule |
| Safe Mode | no | starts no audio driver and no optional app |
| Air-Gapped | no | starts no network driver or network service |
| Recovery | no | goes straight to its desktop; setup does not run |

Only Standard and Hardened let a network driver or service start (`network`, `src/boot/handoff/api/profile.rs:44-47`). Safe Mode is the one `minimal` mode (`minimal`, `src/boot/handoff/api/profile.rs:49-52`), and Recovery the one that `skips_setup` (`skips_setup`, `src/boot/handoff/api/profile.rs:54-57`). An airgapped build has no network code to start in any boot mode. [Boot modes](../install/boot-modes.md) describes the menu.

## The configurator of the `mk/` targets

The older make build has its own interactive configurator, `tools/nonos-config`, which `make nonos-mk-menuconfig` opens and which writes `.nonos-config` for `make nonos-mk-from-config` (`FROM_CONFIG_FEATURES`, `mk/20-build.mk:960-986`). It does not read `nonos.toml`, and the flake does not read `.nonos-config`.
