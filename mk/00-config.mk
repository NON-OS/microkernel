# Paths, versions and keys: the global variables every other module builds on.
# No targets live here.
#
# These lanes run inside the flake's shell only (the Makefile enters it), so
# every tool below is the pinned one on PATH and nothing here asks which
# operating system it is on.

# The component selection tools/nonos-config writes, for the from-config lane.
# Optional: the leading dash keeps an absent file from being an error.
-include .nonos-config

BOOTLOADER_DIR := nonos-bootloader
TARGET_DIR     := target
ESP_DIR        := $(TARGET_DIR)/esp
USB_IMG        := $(TARGET_DIR)/nonos.img
# Total image size in MiB. The attested kernel alone is ~70 MB, so keep generous headroom.
USB_IMG_MB     ?= 384
KEYS_DIR       := $(BOOTLOADER_DIR)/keys

export SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || date +%s)
export CARGO_INCREMENTAL := 0

# Capsules are independent targets and their shared inputs are real
# prerequisites, so the set fans out safely. NONOS_JOBS=1 when bisecting a
# build failure and interleaved output gets in the way.
#
# How wide is a memory question rather than a core-count one. A rustc pass over
# the kernel crate holds a couple of gigabytes, and make's fan-out multiplies
# that by cargo's own, so sizing purely off logical cores oversubscribes badly:
# eight make jobs each starting an eight-way cargo is dozens of concurrent
# rustc on a machine that fits about six. That drives the host into swap, where
# the build gets slower than it was serial and anything else running, a QEMU
# guest in particular, fails to allocate at all.
#
# So the width is the smaller of the cores and what memory can hold at
# roughly four gigabytes a job, and cargo is capped per invocation so the two
# layers cannot multiply out of hand.
NONOS_CORES := $(shell nproc)
NONOS_RAM_GB := $(shell python3 -c 'import os; print(os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES") >> 30)')
NONOS_JOBS ?= $(shell \
	c=$(NONOS_CORES); m=$$(expr $(NONOS_RAM_GB) / 4); \
	[ "$$m" -lt "$$c" ] && c=$$m; [ "$$c" -lt 1 ] && c=1; echo $$c)
MAKEFLAGS += -j$(NONOS_JOBS)
export CARGO_BUILD_JOBS ?= 2

# The toolchain is the flake's: rust-toolchain.toml read through rust-overlay,
# with the std platform layer already applied (tools/nix/capsules.nix). The
# name stays for the recipes that pass it; with no rustup it changes nothing.
TOOLCHAIN := $(shell sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml)
CARGO     := cargo
NONOS_PYTHON ?= python3
# Compares a capsule's `.nonos.caps` section against the capability set its
# manifest is about to be signed for. Runs before every manifest signature, so
# a manifest granting powers the source never declared is never signed.
NONOS_CAPS_CHECK ?= $(NONOS_PYTHON) scripts/check_declared_caps.py
NONOS_BENCH_OUT ?=
NONOS_BENCH_BUILD_CMD ?= make nonos-mk-verify-fast
NONOS_BENCH_SKIP_BUILD ?= 0
NONOS_BENCH_SKIP_BOOT ?= 0
NONOS_BENCH_STRICT ?= 0
NONOS_BENCH_BOOT_TIMEOUT ?= 360
NONOS_BENCH_BOOT_CMD ?=
NONOS_BENCH_HOST_OUT ?= target/bench/host
NONOS_BENCH_BOOT_JSON ?= target/bench/boot-log.json
NONOS_BENCH_SERIAL_LOG ?=
NONOS_BENCH_BASELINE ?=
NONOS_BENCH_CANDIDATE ?=
NONOS_BENCH_REGRESSION_LIMIT ?= 15

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)
HOST_TARGET := $(shell rustc -vV | sed -n 's/^host: //p')
SHA256 := sha256sum

# Signing key. Auto-generated on first use.
SIGNING_KEY ?= $(KEYS_DIR)/signing_key_v1.bin
KERNEL_MLDSA65_PREFIX ?= $(KEYS_DIR)/kernel_mldsa65
KERNEL_MLDSA65_KEY ?= $(KERNEL_MLDSA65_PREFIX).seed
KERNEL_MLDSA65_PUB ?= $(KERNEL_MLDSA65_PREFIX).pub
NONOS_TRUST_DIR := nonos-data/trust

# Transparent post-quantum STARK enrollment. Produces the capsule policy root
# and every capsule's money-grade membership trailer, verified by the same
# nonos-attest-path the kernel and bootloader link.
NONOS_STARK_ENROLL := nonos-stark-enroll/target/$(HOST_TARGET)/release/nonos-stark-enroll
CAPSULE_SIGN_BIN := nonos-sign/target/release/capsule-sign
ZK_CAPSULE_ROOT  ?= $(NONOS_TRUST_DIR)/policy/zk_capsule_policy_root.bin
# The kernel embeds this root at compile time (security/capsule_attest), so an
# attested kernel build depends on the capsule enrollment having run first.
ZK_POLICY_ROOT   ?= $(ZK_CAPSULE_ROOT)
# Kernel self-attestation: the bootloader embeds this root and verifies the
# kernel's own STARK membership trailer before the jump. Provisioned by the
# kernel enrollment step below.
KERNEL_ATTEST_ROOT_BIN ?= $(NONOS_TRUST_DIR)/policy/kernel_attest_root.bin
KERNEL_ATTEST_TRAILER  ?= $(TARGET_DIR)/kernel-attest/kernel.zk_trailer.bin
KERNEL_ATTEST_ELF      ?= $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel
# The bootloader's own tree. It cannot share the kernel's, since the loader
# embeds the kernel root; this root is embedded in nothing and is published with
# the release for the device proof and outside verifiers to open.
BOOTLOADER_ATTEST_ROOT_BIN ?= $(NONOS_TRUST_DIR)/policy/bootloader_attest_root.bin
# The boot-root record the kernel checks the loader against. A release uses
# the record ek signs for its bootloader root; a test image, with no such
# record, gets one signed by the scratch policy key, at the rollback index.
BOOT_ROOT_APPROVAL ?= $(NONOS_TRUST_DIR)/policy/boot_root.approval
DEVICE_POLICY_KEY  ?= .keys/device_policy_p256.pem
BOOTLOADER_ATTEST_TRAILER  ?= $(TARGET_DIR)/bootloader-attest/bootloader.zk_trailer.bin
_boot_comma := ,
EMBED_TOOL       := $(BOOTLOADER_DIR)/tools/embed-trailer/target/$(HOST_TARGET)/release/embed-trailer
SIGN_TOOL        := $(BOOTLOADER_DIR)/tools/sign-kernel/target/$(HOST_TARGET)/release/sign-kernel

# Anti-rollback index baked into and signed over the kernel image. Starts at 1,
# the minimum the bootloader accepts (index 0 means unset and is rejected); bump
# only for security-critical releases. The TPM monotonic counter enforces it as
# a floor so an older signed kernel cannot be rolled back onto a device.
NONOS_ROLLBACK_INDEX ?= 1

# Changing the index must force a re-sign: the signed kernel does not otherwise
# depend on the value, so it keys on this per-index stamp.
NONOS_ROLLBACK_STAMP := $(TARGET_DIR)/.rollback-index.$(NONOS_ROLLBACK_INDEX)

# Header and status line, shown once per invocation. Fields are read from
# the working tree at parse time.
NONOS_BRANCH    := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo detached)
NONOS_COMMIT    := $(shell git rev-parse --short HEAD 2>/dev/null || echo none)
NONOS_DIRTY     := $(shell test -n "$$(git status --porcelain 2>/dev/null)" && echo '*' || echo '')
NONOS_ATTESTED  := $(shell ls nonos-data/trust/capsules/*.zk_trailer.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_SIGNED    := $(shell ls nonos-data/trust/capsules/*.manifest.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_KERNEL_ROOT_FPR := $(shell test -f $(KERNEL_ATTEST_ROOT_BIN) && $(SHA256) $(KERNEL_ATTEST_ROOT_BIN) | cut -c1-16 || echo unenrolled)

define NONOS_BANNER

  ███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗
  ████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝
  ██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗
  ██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║
  ██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
endef
export NONOS_BANNER

# NONOS_QUIET=1 keeps stdout to what a target prints, for tools that parse it.
ifeq ($(NONOS_QUIET),)
$(info $(NONOS_BANNER))
endif

