# Hostile Linux guests, signed and enrolled like capsules, for test images.
#
# NONOS_LINUX_GUESTS=1 builds each guest for musl, gives it a NØNOS-ID
# certificate and manifest through the same template a capsule uses, and so
# puts it under the enrolled policy root. The personality then verifies a
# guest exactly as it would any NØNOS-built Linux program: nothing here
# weakens a check to let a test through.
#
# Test images only: publisher keys are minted on first use, so this needs a
# scratch trust tree (NONOS_DEV=1). A guest holds no capabilities; its two
# endpoints are what a manifest must declare, and no guest registers them.

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

# A publisher key per guest, minted into the scratch tree when missing.
$(NONOS_BAKED_TRUST_DIR)/keys/guest_%_publisher_ed25519.pub \
$(NONOS_BAKED_TRUST_DIR)/keys/guest_%_publisher_mldsa65.pub: | $(CAPSULE_SIGN_BIN)
	@mkdir -p .keys $(NONOS_BAKED_TRUST_DIR)/keys
	@for alg in ed25519 mldsa65; do \
		$(CAPSULE_SIGN_BIN) keygen --alg $$alg --out .keys/guest_$*_publisher_$$alg && \
		chmod 600 .keys/guest_$*_publisher_$$alg.seed && \
		mv .keys/guest_$*_publisher_$$alg.pub $(NONOS_BAKED_TRUST_DIR)/keys/; \
	done

# name, service port, reply port. The enrolled copy is named guest_<name>,
# so its certificate and trailer cannot collide with a capsule's.
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
CAPSULE_PREBUILT_BIN     := $(LINUX_GUESTS_OUT)/$(1)
CAPSULE_MK_FILE          := $(LINUX_GUESTS_DIR)/Guests.mk
CAPSULE_METADATA         := NØNOS hostile Linux guest $(1)
include nonos-mk/capsule.mk
endef

$(eval $(call LINUX_GUEST,suite,4950,4951))
$(eval $(call LINUX_GUEST,holder,4952,4953))
$(eval $(call LINUX_GUEST,reader,4954,4955))

LINUX_GUEST_SLUGS := linux-guest-suite linux-guest-holder linux-guest-reader
# The template checks keys exist; this makes the check wait for the mint.
$(foreach g,suite holder reader,$(eval nonos-mk-check-linux-guest-$(g)-keys: \
	$(NONOS_BAKED_TRUST_DIR)/keys/guest_$(g)_publisher_ed25519.pub \
	$(NONOS_BAKED_TRUST_DIR)/keys/guest_$(g)_publisher_mldsa65.pub))

# What the store image carries: each guest under /linux/bin with its proof
# beside it, and the file naming the one the boot instance runs.
LINUX_GUEST_BOOT ?= suite
LINUX_GUEST_BOOT_FILE := $(TARGET_DIR)/linux-guests/boot-$(LINUX_GUEST_BOOT)
$(LINUX_GUEST_BOOT_FILE):
	@mkdir -p $(@D) && printf '/bin/%s\n' '$(LINUX_GUEST_BOOT)' > $@

LINUX_GUEST_STORE_ENTRIES := --entry /linux/etc/nonos-boot-guest=$(LINUX_GUEST_BOOT_FILE) \
	$(foreach s,$(LINUX_GUEST_SLUGS),\
		--entry /linux/bin/$(patsubst guest_%,%,$($(s)_BIN_NAME))=$($(s)_BIN) \
		--entry /linux/bin/$(patsubst guest_%,%,$($(s)_BIN_NAME)).nonos_id_cert.bin=$($(s)_CERT) \
		--entry /linux/bin/$(patsubst guest_%,%,$($(s)_BIN_NAME)).manifest.bin=$($(s)_MANIFEST) \
		--entry /linux/bin/$(patsubst guest_%,%,$($(s)_BIN_NAME)).zk_trailer.bin=$($(s)_ATTESTATION))
LINUX_GUEST_STORE_DEPS := $(LINUX_GUEST_BOOT_FILE) \
	$(foreach s,$(LINUX_GUEST_SLUGS),$($(s)_ARTIFACTS) $($(s)_ATTESTATION))
