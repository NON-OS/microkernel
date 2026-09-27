# The files a guest-test image carries beside the guests. Included by Guests.mk.

# The boot program and its arguments, `|` between them, one a line in the file.
LINUX_GUEST_BOOT_LINES ?= /bin/suite
LINUX_GUEST_BOOT_FILE := $(TARGET_DIR)/linux-guests/nonos-boot-guest
.PHONY: nonos-mk-linux-guest-boot
$(LINUX_GUEST_BOOT_FILE): nonos-mk-linux-guest-boot
	@mkdir -p $(@D) && printf '%s\n' '$(LINUX_GUEST_BOOT_LINES)' | tr '|' '\n' > $@
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
