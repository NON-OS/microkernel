# Build NONOS

Follow these steps to go from a clean machine to a NONOS image booting under QEMU; the table at the end maps the other build pages.

## From clone to boot

Install Nix with flakes turned on ([toolchain.md](toolchain.md) says how), then get the source:

```
git clone https://github.com/NON-OS/nonos-unified
cd nonos-unified
```

Not tested in this release.

Check that the machine can build:

```
make doctor
```

The `doctor` target checks that Nix is installed, that flakes are on, and whether QEMU will have hardware virtualization, and ends with `This machine can build NONOS: make` (`Makefile:124-136`).

Build, make an image you can boot, and boot it:

```
make
make dev-image
make dev-boot
```

Not tested in this release.

- `make` runs the `build` target: `nix build` for the profile `nonos.toml` names, then the build receipt (`build`, `Makefile:53-56`).
- `make dev-image` builds the qemu profile's development twin and seals it with throwaway keys, in a copy of the checkout under `target/dev/tree` (`TREE`, `tools/nonos-dev-image:43`).
- `make dev-boot` boots that image under QEMU with a software TPM; `BOOT_DISK` and `QEMU_ARGS` pass more options (`Makefile:77-78`).

The first build fetches every pinned source. Every seal, a development one included, lays the `qwen3-0.6b` Qwen tier on the stick and downloads its pinned files unless they are already in `target/models/files` and match their pins (`STICK_TIER`, `tools/nonos_seal/media.py:36-42`). Both steps need a network the first time ([seal.md](seal.md)).

A [development image](../overview/glossary.md#development-image) is for testing. Its gates admit a [capsule](../overview/glossary.md#capsule) on its path alone, without the STARK proof, and its loader runs the development policy (`dev`, `tools/nix/config.nix:122-126`). The seal refuses it for a release (`release`, `tools/nonos_seal/__main__.py:119-120`). A release image needs the release keys, which the maintainers hold: see [seal.md](seal.md).
