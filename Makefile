# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NONOS is built by its flake (flake.nix, tools/nix/). This file is the short
# way to type it: every target is one nix command, and you need nothing but Nix.
#
#   1. build   make                  the kernel, every capsule, the Linux
#                                    userland and the loader, unsigned and
#                                    reproducible: anyone gets the same bytes
#              make check            every proof crate and every static check
#
#   2. seal    make seal             enroll, sign and pack those bytes into a
#                                    bootable image, with ek's keys. The build
#                                    never holds a key and the seal never
#                                    compiles; this is the one seam between them
#
#   3. boot    make boot             the sealed image under QEMU
#              make boot-install     beside a blank disk, to try the installer
#              make boot-installed   the disk the installer wrote, alone
#              make usb DISK=...     write the sealed image to a stick
#
#   4. test    make dev-image        a profile's development twin, sealed with
#                                    throwaway keys in target/dev and path-only
#                                    attestation: minutes, no prover, no real key
#              make dev-boot         boot it under QEMU
#
#   A boot keeps its data disk, and the encrypted volume on it, from one boot
#   to the next. FRESH=1 starts it again; MODEL=auto lays the Qwen tier setup
#   picks for the memory on that new disk (or MODEL=small, MODEL=qwen3-4b ...):
#              make dev-boot FRESH=1 MODEL=auto
#
#   make profiles       what each profile is for, and its privacy posture
#   PROFILE=airgapped make build seal boot    the same for another profile
#   make shell          the pinned toolchain, for work by hand
#   make doctor         what this machine needs before the first build
#
# tools/nix/README.md says all of it at length. The nonos-mk-* lanes in mk/
# (smoke tests, the attack suite, the boot matrix) run in the flake's shell;
# `make nonos-mk-<lane>` enters it.

.DEFAULT_GOAL := build

PROFILE ?=
ATTR := $(if $(PROFILE),$(PROFILE),default)
NIX ?= nix
# The data disk and the model a boot starts with (tools/nonos_qemu/disk.py).
BOOT_DISK = $(if $(FRESH),--fresh) $(foreach m,$(MODEL),--model $(m))

.PHONY: build check seal boot dev-image dev-boot boot-install boot-installed qemu qemu-tpm qemu-install qemu-installed profiles shell usb doctor clean help

# The build, then its receipt: every artifact, the toolchain and every pinned
# input by hash, what the seal will sign, and whether these are the bytes the
# committed receipts/<profile>.json names.
build:
	$(NIX) build .#$(ATTR)
	@$(NIX) run .#receipt
	@echo "Next: make seal (with your keys), then make boot."

# Every check, then what each proved: passed or failed, tests run, the bill of
# materials. The record goes to receipts/check-<system>.json.
check:
	@$(NIX) run .#check-report

seal:
	$(NIX) run .#seal -- $(if $(PROFILE),--profile $(PROFILE)) $(SEAL_ARGS)

# Every boot attaches a software TPM, so the loader measures what it starts.
boot:
	$(NIX) run .#qemu -- $(if $(PROFILE),--profile $(PROFILE)) --tpm $(BOOT_DISK) $(QEMU_ARGS)

# A development image: the profile's twin with path-only attestation, sealed
# with CI's throwaway keys in a copy of the tree under target/dev, never beside
# the release keys. It boots in minutes and is never a release.
DEV_ATTR = $(if $(filter dev,$(PROFILE)),dev,$(if $(PROFILE),$(PROFILE),qemu)-dev)
dev-image:
	$(NIX) develop --command python3 tools/nonos-dev-image $(if $(PROFILE),--profile $(PROFILE))

dev-boot:
	$(NIX) run .#qemu -- --image target/dev/tree/target/release/$(DEV_ATTR)/nonos.img --tpm $(BOOT_DISK) $(QEMU_ARGS)

# The installer lane: the sealed stick beside a blank NVMe disk, then that disk
# alone, which proves the machine boots from what the installer wrote.
boot-install:
	$(MAKE) --no-print-directory boot QEMU_ARGS="--install-target $(QEMU_ARGS)"

boot-installed:
	$(MAKE) --no-print-directory boot QEMU_ARGS="--installed $(QEMU_ARGS)"

# The names these had before boot existed.
qemu:
	$(NIX) run .#qemu -- $(if $(PROFILE),--profile $(PROFILE)) $(BOOT_DISK) $(QEMU_ARGS)

qemu-tpm:
	$(MAKE) --no-print-directory qemu QEMU_ARGS="--tpm $(QEMU_ARGS)"

qemu-install:
	$(MAKE) --no-print-directory qemu QEMU_ARGS="--tpm --install-target $(QEMU_ARGS)"

qemu-installed:
	$(MAKE) --no-print-directory qemu QEMU_ARGS="--tpm --installed $(QEMU_ARGS)"

profiles:
	@cat "$$($(NIX) build .#profiles --no-link --print-out-paths)"

shell:
	$(NIX) develop

# Writing a disk is the one step that touches the machine, so it asks for the
# disk twice. macOS names the raw device rdiskN and unmounts the disk first.
USB_IMG = $(firstword $(wildcard target/release/$(if $(PROFILE),$(PROFILE),*)/nonos.img))
usb:
	@test -n "$(USB_IMG)" || { echo "no sealed image: make seal"; exit 1; }
	@test -n "$(DISK)" || { echo "Image: $(USB_IMG). Write it with: make usb DISK=/dev/..."; exit 0; }
	@echo "About to OVERWRITE $(DISK) with $(USB_IMG). Everything on it is destroyed."
	@printf "Type the disk path again to confirm: "; read confirm; \
	if [ "$$confirm" != "$(DISK)" ]; then echo "Mismatch; not writing."; exit 1; fi; \
	if [ "$$(uname -s)" = Darwin ]; then \
		diskutil unmountDisk "$(DISK)" && \
		sudo dd if="$(USB_IMG)" of="$$(echo $(DISK) | sed 's#/dev/disk#/dev/rdisk#')" bs=4m && sync && \
		diskutil eject "$(DISK)"; \
	else \
		sudo dd if="$(USB_IMG)" of="$(DISK)" bs=4M status=progress conv=fsync; \
	fi

doctor:
	@command -v $(NIX) >/dev/null || { \
		echo "Install Nix (https://nixos.org/download), then run make again."; exit 1; }
	@$(NIX) --extra-experimental-features "nix-command flakes" flake metadata . >/dev/null && \
		echo "ok    nix, with flakes" || { \
		echo "Turn on flakes: add 'experimental-features = nix-command flakes' to ~/.config/nix/nix.conf"; exit 1; }
	@if [ -w /dev/kvm ] || [ "$$(uname -s)-$$(uname -m)" = Darwin-x86_64 ]; then echo "ok    hardware virtualization for QEMU"; \
		elif [ "$$(uname -s)-$$(uname -m)" = Darwin-arm64 ]; then \
		echo "note  Apple silicon: its hypervisor runs only arm64 guests, and NONOS is x86_64, so QEMU"; \
		echo "      emulates the CPU. Boots use up to 8 cores on their own threads; a small Qwen tier"; \
		echo "      (MODEL=small) is the one to try here. Full speed needs an x86_64 machine with KVM."; \
		else echo "note  no hardware virtualization for an x86_64 guest (no writable /dev/kvm): QEMU emulates the CPU"; fi
	@echo "This machine can build NONOS: make"

clean:
	rm -rf result result-* target

help:
	@sed -n '3,32p' $(firstword $(MAKEFILE_LIST)) | sed 's/^# \{0,1\}//'

# The lanes in mk/ need the flake's tools; outside its shell they enter it.
ifeq ($(NONOS_IN_FLAKE),)
nonos-mk-% ci-%:
	@$(NIX) develop --command $(MAKE) --no-print-directory $@
else
include $(sort $(wildcard mk/*.mk))
endif
