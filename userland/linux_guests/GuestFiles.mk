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
