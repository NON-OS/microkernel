# The files a guest-test image carries beside the guests. Included by Guests.mk.

# The boot program and its arguments, one a line: LINUX_GUEST_BOOT_ARGS names a
# file of them, since a shell script's own | would collide with any separator.
LINUX_GUEST_BOOT_ARGS ?=
LINUX_GUEST_BOOT_FILE := $(TARGET_DIR)/linux-guests/nonos-boot-guest
.PHONY: nonos-mk-linux-guest-boot
$(LINUX_GUEST_BOOT_FILE): nonos-mk-linux-guest-boot
	@mkdir -p $(@D) && if [ -n '$(LINUX_GUEST_BOOT_ARGS)' ]; then cp '$(LINUX_GUEST_BOOT_ARGS)' $@; \
		else echo /bin/suite > $@; fi
LINUX_GUEST_STORE_DEPS += $(LINUX_GUEST_BOOT_FILE)
LINUX_GUEST_STORE_ENTRIES += --entry /linux/etc/nonos-boot-guest=$(LINUX_GUEST_BOOT_FILE)

# busybox finds its applets through links: /bin/ls to /bin/busybox and so on,
# at the paths its own --list-full gives. The personality follows the table.
LINUX_GUEST_LINKS := $(TARGET_DIR)/linux-guests/nonos-links
$(LINUX_GUEST_LINKS): userland/capsule_linux/guests/busybox.elf
	@mkdir -p $(@D) && ./$< --list-full | grep -v '^bin/busybox$$' | \
		sed 's|^|/|; s|$$| /bin/busybox|' > $@
LINUX_GUEST_STORE_DEPS += $(LINUX_GUEST_LINKS)
LINUX_GUEST_STORE_ENTRIES += --entry /linux/etc/nonos-links=$(LINUX_GUEST_LINKS)

# The suite with one byte flipped, beside the suite's own proofs: a tampered
# library or program that must be refused however it is reached.
LINUX_GUEST_TAMPERED := $(TARGET_DIR)/linux-guests/tampered
$(LINUX_GUEST_TAMPERED): $(linux-guest-suite_BIN) tools/nonos-flip-byte
	@mkdir -p $(@D) && $(NONOS_PYTHON) tools/nonos-flip-byte $< $@
LINUX_GUEST_STORE_DEPS += $(LINUX_GUEST_TAMPERED)
LINUX_GUEST_STORE_ENTRIES += --entry /linux/bin/tampered=$(LINUX_GUEST_TAMPERED) \
	--entry /linux/bin/tampered.nonos_id_cert.bin=$(linux-guest-suite_CERT) \
	--entry /linux/bin/tampered.manifest.bin=$(linux-guest-suite_MANIFEST) \
	--entry /linux/bin/tampered.zk_trailer.bin=$(linux-guest-suite_ATTESTATION)

# libprobe.so with one byte flipped, beside the good library's proofs: the
# library a dynamic program is refused when it asks for it.
LINUX_GUEST_BAD_LIB := $(TARGET_DIR)/linux-guests/libprobe_bad.so
$(LINUX_GUEST_BAD_LIB): $(linux-guest-libprobe_BIN) tools/nonos-flip-byte
	@mkdir -p $(@D) && $(NONOS_PYTHON) tools/nonos-flip-byte $< $@
LINUX_GUEST_STORE_DEPS += $(LINUX_GUEST_BAD_LIB)
LINUX_GUEST_STORE_ENTRIES += --entry /linux/lib/libprobe_bad.so=$(LINUX_GUEST_BAD_LIB) \
	--entry /linux/lib/libprobe_bad.so.nonos_id_cert.bin=$(linux-guest-libprobe_CERT) \
	--entry /linux/lib/libprobe_bad.so.manifest.bin=$(linux-guest-libprobe_MANIFEST) \
	--entry /linux/lib/libprobe_bad.so.zk_trailer.bin=$(linux-guest-libprobe_ATTESTATION)

# A Go suite image holds the wrapper, the packages NONOS_LINUX_GO_SUITE_STORE
# names (all enrolled ones unless narrowed), each package's testdata/ and test
# sources at the paths they have on the build host, and Go's zone database,
# and nothing else: one test binary is 4 to 15 MB against the store's 16 MiB
# and 128 entries (tools/nonos-store-pack). Changing this list needs only the
# store step.
ifeq ($(NONOS_LINUX_GO_SUITE),1)
NONOS_LINUX_GO_SUITE_STORE ?= $(NONOS_LINUX_GO_SUITE_PKGS)
GO_SUITE_ENTRY = --entry /linux/bin/$(1)=$(linux-guest-$(1)_BIN) \
	--entry /linux/bin/$(1).nonos_id_cert.bin=$(linux-guest-$(1)_CERT) \
	--entry /linux/bin/$(1).manifest.bin=$(linux-guest-$(1)_MANIFEST) \
	--entry /linux/bin/$(1).zk_trailer.bin=$(linux-guest-$(1)_ATTESTATION)
# A test also reads its own sources: an example reads example_test.go, and
# the package directory exists for gostd to change into only if something is
# in it. NONOS_LINUX_GO_SUITE_SOURCES=0 leaves them out for a package whose
# testdata alone nearly fills the 128 entries (runtime).
NONOS_LINUX_GO_SUITE_SOURCES ?= 1
GO_SUITE_SOURCES = $(if $(filter 1,$(NONOS_LINUX_GO_SUITE_SOURCES)),$(notdir $(wildcard $(GO_ROOT)/src/$(1)/*_test.go)))
GO_SUITE_TESTDATA = $(foreach f,$(shell cd $(GO_ROOT)/src/$(1) && find testdata -type f 2>/dev/null | sort) $(GO_SUITE_SOURCES), \
	--entry /linux$(GO_ROOT)/src/$(1)/$(f)=$(GO_ROOT)/src/$(1)/$(f))
LINUX_GUEST_STORE_ENTRIES := $(call GO_SUITE_ENTRY,gostd) \
	$(foreach p,$(NONOS_LINUX_GO_SUITE_STORE),$(call GO_SUITE_ENTRY,gs$(subst /,,$(p))) $(call GO_SUITE_TESTDATA,$(p))) \
	--entry /linux$(GO_ROOT)/lib/time/zoneinfo.zip=$(GO_ROOT)/lib/time/zoneinfo.zip \
	--entry /linux/etc/nonos-boot-guest=$(LINUX_GUEST_BOOT_FILE)
endif
