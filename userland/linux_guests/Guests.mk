# Linux guests signed and enrolled like capsules, for test images.
#
# NONOS_LINUX_GUESTS=1 builds each guest, signs it through the capsule
# template and enrols it under the policy root, so the personality verifies it
# as it would any NØNOS-built Linux program; no check is weakened for a test.
# Scratch trust only: publisher keys are minted on first use. A guest holds no
# capabilities; its endpoints are declared, never registered.

ifneq ($(NONOS_DEV),1)
$(error NONOS_LINUX_GUESTS=1 mints scratch publisher keys and needs NONOS_DEV=1)
endif

LINUX_GUESTS_DIR := userland/linux_guests
LINUX_GUESTS_TRIPLE := x86_64-unknown-linux-musl
LINUX_GUESTS_OUT := $(LINUX_GUESTS_DIR)/target/$(LINUX_GUESTS_TRIPLE)/release
LINUX_GUESTS_SRCS := $(shell find $(LINUX_GUESTS_DIR)/src -name '*.rs') \
	$(LINUX_GUESTS_DIR)/Cargo.toml $(LINUX_GUESTS_DIR)/Cargo.lock

# Static and non-PIE, like the busybox the personality already runs.
$(LINUX_GUESTS_OUT)/%: $(LINUX_GUESTS_SRCS)
	@echo "Building Linux guest $*..."
	@cd $(LINUX_GUESTS_DIR) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=static" \
		cargo build --release --target $(LINUX_GUESTS_TRIPLE) --bin $*
$(NONOS_BAKED_TRUST_DIR)/keys/guest_%_publisher_ed25519.pub \
$(NONOS_BAKED_TRUST_DIR)/keys/guest_%_publisher_mldsa65.pub: | $(CAPSULE_SIGN_BIN)
	@mkdir -p .keys $(NONOS_BAKED_TRUST_DIR)/keys
	@for alg in ed25519 mldsa65; do \
		$(CAPSULE_SIGN_BIN) keygen --alg $$alg --out .keys/guest_$*_publisher_$$alg && \
		chmod 600 .keys/guest_$*_publisher_$$alg.seed && \
		mv .keys/guest_$*_publisher_$$alg.pub $(NONOS_BAKED_TRUST_DIR)/keys/; \
	done

# name, service port, reply port[, prebuilt ELF[, guest path]]. The enrolled
# copy is named guest_<name>, so its certificate and trailer cannot collide
# with a capsule's. The guest path defaults to /bin/<name>.
define LINUX_GUEST
CAPSULE_SLUG             := linux-guest-$(1)
CAPSULE_HANDLE           := linux.guest.$(1)
CAPSULE_DIR              := $(LINUX_GUESTS_DIR)
CAPSULE_BIN_NAME         := guest_$(1)
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_NAMESPACE        := systems.nonos.linux.guest.$(1)
CAPSULE_TARGET           := $(LINUX_GUESTS_TRIPLE)
CAPSULE_SERVICE_ENDPOINT := service:$(2):linux.guest.$(1)
CAPSULE_REPLY_ENDPOINT   := reply:$(3):endpoint.linux.guest.$(1).reply
CAPSULE_REQUIRED_CAPS    := 0x0
CAPSULE_PREBUILT_BIN     := $(or $(4),$(LINUX_GUESTS_OUT)/$(1))
CAPSULE_MK_FILE          := $(LINUX_GUESTS_DIR)/Guests.mk
CAPSULE_METADATA         := NØNOS Linux guest $(1)
include nonos-mk/capsule.mk
# The template checks the keys exist; this makes it wait for the mint.
nonos-mk-check-linux-guest-$(1)-keys: \
	$(NONOS_BAKED_TRUST_DIR)/keys/guest_$(1)_publisher_ed25519.pub \
	$(NONOS_BAKED_TRUST_DIR)/keys/guest_$(1)_publisher_mldsa65.pub
LINUX_GUEST_STORE_DEPS += $$(linux-guest-$(1)_ARTIFACTS) $$(linux-guest-$(1)_ATTESTATION)
LINUX_GUEST_STORE_ENTRIES += --entry /linux$(or $(5),/bin/$(1))=$$(linux-guest-$(1)_BIN) \
	--entry /linux$(or $(5),/bin/$(1)).nonos_id_cert.bin=$$(linux-guest-$(1)_CERT) \
	--entry /linux$(or $(5),/bin/$(1)).manifest.bin=$$(linux-guest-$(1)_MANIFEST) \
	--entry /linux$(or $(5),/bin/$(1)).zk_trailer.bin=$$(linux-guest-$(1)_ATTESTATION)
endef

$(eval $(call LINUX_GUEST,suite,4950,4951))
$(eval $(call LINUX_GUEST,holder,4952,4953))
$(eval $(call LINUX_GUEST,reader,4954,4955))
$(eval $(call LINUX_GUEST,window,4964,4965))
$(eval $(call LINUX_GUEST,signal,4966,4967))
# Alpine's static busybox, the one app.linux embeds, as a program from the store.
$(eval $(call LINUX_GUEST,busybox,4956,4957,userland/capsule_linux/guests/busybox.elf))

# Tier 2: a dynamically linked program, its library, and musl's loader, which
# is also its libc. Each is proved like any program; the loader refuses a
# library whose bytes were not.
LINUX_GUESTS_C := $(TARGET_DIR)/linux-guests/c
MUSL_LIBC := /usr/lib/x86_64-linux-musl/libc.so
$(LINUX_GUESTS_C)/libprobe.so: $(LINUX_GUESTS_DIR)/c/probe_lib.c
	@mkdir -p $(@D) && musl-gcc -shared -fPIC -O2 -o $@ $<
$(LINUX_GUESTS_C)/dyn: $(LINUX_GUESTS_DIR)/c/dyn.c $(LINUX_GUESTS_C)/libprobe.so
	@musl-gcc -O2 -o $@ $< -L$(LINUX_GUESTS_C) -lprobe
$(eval $(call LINUX_GUEST,dyn,4958,4959,$(LINUX_GUESTS_C)/dyn))
$(eval $(call LINUX_GUEST,libprobe,4960,4961,$(LINUX_GUESTS_C)/libprobe.so,/lib/libprobe.so))
$(eval $(call LINUX_GUEST,ldmusl,4962,4963,$(MUSL_LIBC),/lib/ld-musl-x86_64.so.1))

# Tier 1: static Go binaries, the cheapest guests: no cgo, no loader, no libc.
# Built here with the toolchain the container carries. A tool that comes up at
# all proves the runtime's threads, memory and signals under the personality.
GO := /usr/local/go/bin/go
GO_OUT := $(TARGET_DIR)/linux-guests/go
$(GO_OUT)/%: $(LINUX_GUESTS_DIR)/go/%/main.go
	@mkdir -p $(@D) && cd $(LINUX_GUESTS_DIR)/go/$* && \
		CGO_ENABLED=0 GOOS=linux GOARCH=amd64 GOFLAGS=-trimpath \
		GOCACHE=$(abspath $(GO_OUT))/cache GOPATH=$(abspath $(GO_OUT))/path \
		$(GO) build -ldflags '-s -w' -o $(abspath $@) .
$(eval $(call LINUX_GUEST,gohello,4968,4969,$(GO_OUT)/hello))
$(eval $(call LINUX_GUEST,goconc,4970,4971,$(GO_OUT)/conc))
# Go's network poller: a timer's epoll wait and its eventfd wake, and a pipe
# read through the poller to end of file.
$(eval $(call LINUX_GUEST,gopoll,4976,4977,$(GO_OUT)/poll))
# A goroutine spinning with no call, which only a signal to its running
# thread can move off the one CPU the guest has.
$(eval $(call LINUX_GUEST,gopreempt,4944,4945,$(GO_OUT)/preempt))

include $(LINUX_GUESTS_DIR)/CGuests.mk

# The Linux-guest test store is about guests, not the desktop's media and demo
# capsules. Drop both so the signed guest set fits the vfs load budget; the
# normal image, which does not set NONOS_LINUX_GUESTS, still ships them.
override NONOS_STORE_MEDIA_ENTRIES :=
override NONOS_STORE_DEMO_ENTRIES :=

include $(LINUX_GUESTS_DIR)/GuestFiles.mk

include $(LINUX_GUESTS_DIR)/Outside.mk
