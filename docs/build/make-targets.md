# Make targets

Look up any `make` target here: what each top-level target runs, the options a QEMU boot takes, and the older `nonos-mk-*` targets in `mk/`.

## How the Makefile works

Every target of the top-level [Makefile](../../Makefile) is one `nix` command, and nothing but Nix is needed to run them (`Makefile:3-4`). With no target, `make` runs `build` (`.DEFAULT_GOAL`, `Makefile:40`). Try the read-only ones first:

```
make help
make -n build
```

`make help` prints the summary at the top of the Makefile (`help`, `Makefile:141-142`). `make -n TARGET` prints the commands a target would run without running them; `make -n build` prints `nix build .#default`, `nix run .#receipt` and the closing `echo`.

## Build and check

| target | what it does |
|---|---|
| `build`, the default | `nix build .#default`, or `.#<profile>` when `PROFILE` is set, then `nix run .#receipt`, which prints the build receipt and compares it with the receipt committed for that profile (`build`, `Makefile:53-56`) |
| `check` | `nix run .#check-report`: builds every flake check for this host and prints what each proved (`check`, `Makefile:60-61`) |
| `profiles` | prints each [build profile](../overview/glossary.md#build-profile), what it is for, its privacy posture and its loader policy (`profiles`, `Makefile:101-102`) |
| `shell` | `nix develop`, the pinned toolchain for work by hand (`shell`, `Makefile:104-105`) |
| `doctor` | checks for Nix, flakes and hardware virtualization (`doctor`, `Makefile:124-136`) |
| `clean` | removes `result`, every `result-*` link and the whole `target` directory (`clean`, `Makefile:138-139`) |
| `help` | prints the Makefile's own summary (`help`, `Makefile:141-142`) |

`make clean` also removes what lives under `target`: sealed images, the development checkout and its throwaway keys, the QEMU data disk, the software TPM state and the Qwen files fetched into `target/models/files`.

The receipt step writes too. It replaces the committed receipt for that profile in your checkout, except when the build failed to reproduce, and it exits with an error when the same commit and inputs gave other bytes or when the kernel holds a capsule its profile takes out (`main`, `tools/nonos-receipt:243-263`). [reproducible-builds.md](reproducible-builds.md) explains the verdicts.

## Seal and boot

| target | what it does |
|---|---|
| `seal` | `nix run .#seal`, with `--profile` when `PROFILE` is set and `SEAL_ARGS` appended; the [seal](../overview/glossary.md#seal) needs signing keys (`SEAL_ARGS`, `Makefile:63-64`) |
| `boot` | `nix run .#qemu` with `--tpm`: the sealed image under QEMU with a software TPM (`BOOT_DISK`, `Makefile:67-68`) |
| `boot-install` | `boot` with a blank NVMe disk beside the image, to try the installer (`QEMU_ARGS`, `Makefile:82-83`) |
| `boot-installed` | `boot` of the NVMe disk the installer wrote, alone (`QEMU_ARGS`, `Makefile:85-86`) |
| `dev-image` | runs `tools/nonos-dev-image` in the development shell: the [development image](../overview/glossary.md#development-image) of `PROFILE`, qemu when unset, sealed with throwaway keys (`DEV_ATTR`, `Makefile:73-75`) |
| `dev-boot` | boots the image `dev-image` wrote, with `--tpm` (`DEV_ATTR`, `Makefile:77-78`) |
| `qemu`, `qemu-tpm`, `qemu-install`, `qemu-installed` | the names these had before `boot`: `qemu` boots without a TPM, the other three add `--tpm` (`QEMU_ARGS`, `Makefile:88-99`) |
| `usb` | writes the sealed image to `DISK` (`USB_IMG`, `Makefile:107-122`) |

[seal.md](seal.md) says what the seal does and what to use instead when you do not hold the release keys.

### Variables

| variable | effect |
|---|---|
| `PROFILE` | the profile to build, seal or boot; empty means the one `nonos.toml` names (`ATTR`, `Makefile:42-43`) |
| `FRESH=1` | passes `--fresh`: a new data disk, and a new software TPM (`BOOT_DISK`, `Makefile:46`) |
| `MODEL=` | passes `--model` once per word: a Qwen tier name, or `auto` (`BOOT_DISK`, `Makefile:46`) |
| `QEMU_ARGS` | more options for the QEMU runner, listed below |
| `SEAL_ARGS` | more options for the seal, such as `--release` |
| `DISK` | the device `usb` writes |
| `NIX` | the `nix` command to run (`NIX`, `Makefile:44`) |

```
PROFILE=airgapped make build seal boot
make dev-boot FRESH=1 MODEL=auto
```

Not tested in this release.

### Writing a USB stick

```
make usb
make usb DISK=/dev/sdX
```

Not tested in this release.

Without `DISK`, `make usb` prints the path of the sealed image, or says there is none, and stops. With it, the target says the disk will be overwritten and asks you to type the disk path a second time; on a mismatch it writes nothing. On Linux it writes with `sudo dd` and `conv=fsync`; on macOS it unmounts the disk, writes to the raw `rdisk` device and ejects it (`USB_IMG`, `Makefile:107-122`). Every byte on that disk is lost. [Write a USB stick](../install/usb-stick.md) walks through it.

## Options of a QEMU boot

`make boot`, `make dev-boot` and `nix run .#qemu` run the QEMU runner in `tools/nonos_qemu`. Pass options after `--`, or through `QEMU_ARGS` (`args`, `tools/nonos_qemu/__main__.py:50-72`):

| option | what it does |
|---|---|
| `--profile P` | boots the image sealed for `P`; without it, the only image under `target/release` |
| `--image PATH` | boots this disk image instead of a sealed one |
| `--tpm` | attaches a software TPM 2.0 |
| `--fresh` | starts the data disk again from the image, and the TPM with it |
| `--stick` | boots the sealed stick alone, with no data volume |
| `--usb` | with `--stick`, plugs the stick into the USB controller as mass storage |
| `--model TIER` | lays a Qwen tier on a new data disk; `auto` picks the tier setup would pick for `--mem` |
| `--smp N` | CPUs; the default is 8, or every core of the host when it has fewer |
| `--mem SIZE` | memory, `8G` by default |
| `--net nat` or `--net off` | user mode networking, or none; `nat` is the default |
| `--install-target` | attaches a blank 8 GiB NVMe disk with the serial `NONOS-TARGET` |
| `--installed` | boots that NVMe disk alone |
| `--headless` | no window; the serial console goes to `--serial`, by default `target/qemu/serial.log` |
| `--timeout N` | stops a headless boot after `N` seconds |
| `--expect PATTERN` | a pattern the serial console must show; repeat it for more |

The machine is a q35 with UEFI firmware, a virtio VGA at 1920 by 1080, a USB controller, a virtio RNG, a virtio network card when `--net nat`, the TPM on a CRB interface when `--tpm`, and Intel HD Audio (`devices`, `tools/nonos_qemu/machine.py:91-103`). The blank install disk is `INSTALL_TARGET_GB` gibibytes (`tools/nonos_qemu/machine.py:23`).

A boot runs from the [ESP](../overview/glossary.md#esp), copied out of the image into a folder QEMU serves as a FAT drive, and from a virtio data disk that carries the [package store](../overview/glossary.md#package-store). The data disk, and the encrypted volume on it, is kept from one boot to the next; an image with another path or another modification time, or `--fresh`, starts it again (`needs_copy`, `tools/nonos_qemu/disk.py:53-62`). The software TPM's state is kept as well, and a new TPM cannot open a volume an earlier TPM made, so the runner warns when that happens (`made`, `tools/nonos_qemu/__main__.py:133-139`).

`--model auto` takes the largest Qwen3 tier whose file, plus a fifth, plus 512 MiB for the runtime, fits in the memory less 1 GiB (`fits`, `tools/nonos_qemu/disk.py:128-142`). The runner fetches the tier's pinned files once into `target/models/files`, checks them against their pins, and lays them only on a new data disk (`lay_models`, `tools/nonos_qemu/disk.py:154-163`). Fetching needs the network.

With `--headless --timeout N`, a boot passes when every `--expect` pattern appears on the serial console before the deadline. With no pattern, a boot still running at the deadline passes and one that stopped fails (`watch`, `tools/nonos_qemu/__main__.py:88-106`).

## The targets in `mk/`

The files in [mk/](../../mk) hold the older make build: it calls `cargo` directly instead of the flake. The Makefile includes them only inside the flake's shell; outside it, any `make nonos-mk-<name>` or `make ci-<name>` re-runs itself inside `nix develop` (`NONOS_IN_FLAKE`, `Makefile:145-150`).

These targets sign with a local development key. The first target that needs one generates an Ed25519 seed (`SIGNING_KEY`, `mk/10-qemu.mk:163-167`) and an ML-DSA-65 key (`KERNEL_MLDSA65_KEY`, `mk/10-qemu.mk:172-175`) in the loader's key directory (`KEYS_DIR`, `mk/00-config.mk:18`). An image built this way carries those local keys and is never a release. `nonos-mk-distclean` deletes the Ed25519 and ML-DSA-65 key files at those paths, whoever made them (`SIGNING_KEY`, `mk/00-config.mk:76-79`).

| target | what it does |
|---|---|
| `nonos-mk-run` | builds the kernel and the ESP, then boots under QEMU with a software TPM and NAT networking; quit with Ctrl+A then X (`nonos_kernel_and_esp`, `mk/40-run.mk:107-120`) |
| `nonos-mk-run-net` | the same with port forwards to the guest's SSH and HTTP ports (`QEMU_NET_MODE`, `mk/40-run.mk:122-123`) |
| `nonos-mk-run-serial-log` | a headless boot that writes the serial console to `QEMU_SERIAL_LOG` (`QEMU_SERIAL_LOG`, `mk/40-run.mk:205-215`) |
| `nonos-mk-run-smp-serial-log` | the same with `QEMU_SMP` CPUs and a kernel built with `nonos-smp` (`QEMU_SMP`, `mk/40-run.mk:419-429`) |
| `nonos-mk-debug` | waits for GDB on port 1234 before the first instruction (`nonos_kernel_and_esp`, `mk/40-run.mk:228-235`) |
| `nonos-mk-run-iommu-serial-log` | boots under TCG with an emulated Intel IOMMU, so the DMA remapping code runs (`QEMU_IOMMU_OPTS`, `mk/40-run.mk:489-501`) |
| `nonos-mk-boot-matrix` | boots every cell of the machine matrix `BOOT_MATRIX_REPEAT` times, 5 by default, each within `BOOT_MATRIX_TIMEOUT` seconds, 300 by default (`BOOT_MATRIX_REPEAT`, `mk/40-run.mk:514-536`) |
| `nonos-mk-iso` | a UEFI ISO from the ESP, with every timestamp pinned (`NONOS_ISO`, `mk/30-image.mk:19-37`) |
| `nonos-mk-usb-img`, `nonos-mk-usb-run` | a GPT disk image (`USB_IMG`, `mk/20-build.mk:1351-1354`), and a boot of it as a real disk rather than a FAT folder (`USB_IMG`, `mk/30-image.mk:41-51`) |
| `nonos-mk-static`, `nonos-mk-verify-fast` | the capability parity check `check_cap_parity`, the assumption register and the tree checks of `run-static-checks.sh`, with no kernel build (`mk/40-run.mk:345-356`) |
| `nonos-mk-verify`, `nonos-mk-test` | the static checks, the trust checks and a scan of `MICROKERNEL_BIN` for symbols the microkernel must not carry; `nonos-mk-test` adds three QEMU boot tests (`mk/40-run.mk:378-414`) |
| `nonos-mk-arm`, `nonos-mk-arm-run` | the aarch64 kernel, and a boot of it under `qemu-system-aarch64` (`ARM_QEMU_FLAGS`, `mk/20-build.mk:921-924`) |
| `ci-fast`, `ci-security`, `ci-release`, `ci-soak` | `ci-fast` runs the tests of `nonos-verify` with the pinned `TOOLCHAIN` and the claims check, `ci-security` adds two audit scripts, `ci-release` adds a byte for byte double build of the loader, and `ci-soak` runs the QEMU evidence scripts (`mk/50-ci.mk:16-24`) |
| `nonos-mk-clean`, `nonos-mk-clean-all` | removes the kernel build, or every build and the ESP (`TARGET_DIR`, `mk/50-ci.mk:82-91`) |
| `nonos-mk-distclean` | removes everything `nonos-mk-clean-all` does, and the local signing keys (`SIGNING_KEY`, `mk/50-ci.mk:93-101`) |
| `nonos-mk-capsule-catalogue`, `nonos-mk-store-catalogue` | print the JSON the flake reads about the capsules and the store (`nonos_catalogue_print`, `mk/60-nix.mk:52-79`) |

The QEMU of these targets has its own settings: 2 GiB of memory (`QEMU_MEM`, `mk/10-qemu.mk:13`), KVM when `/dev/kvm` is readable and writable, the macOS hypervisor on macOS, TCG otherwise (`QEMU_ACCEL_AUTO`, `mk/10-qemu.mk:19-31`), a 1920 by 1080 display (`QEMU_XRES`, `mk/10-qemu.mk:81-82`), and NAT networking captured to `target/qemu-net.pcap` (`QEMU_NET_CAPTURE`, `mk/10-qemu.mk:35-39`). `QEMU_NET_MODE=hostfwd` forwards host port 2222 to the guest's port 22 and 8080 to its port 80 (`QEMU_HOST_SSH_PORT`, `mk/10-qemu.mk:33-35`, `mk/10-qemu.mk:134-136`).

Make runs as many jobs as the smaller of the core count and the memory in GiB divided by 4, and caps each cargo at 2 jobs; `NONOS_JOBS=1` runs one at a time (`NONOS_JOBS`, `mk/00-config.mk:38-44`).

```
make nonos-mk-static
make nonos-mk-run-serial-log
```

Not tested in this release.
