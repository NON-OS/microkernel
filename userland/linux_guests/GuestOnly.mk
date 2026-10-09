# Which guests an image carries. Included by Guests.mk after every guest.

# The store loads at most 16 MiB and each guest's proof alone is 320 KiB, so
# an image cannot carry every guest at once. LINUX_GUEST_ONLY names the
# programs an image carries, by their names under /linux/bin; every file
# outside /linux/bin (libraries, /etc) stays. Unset, the image carries all.
ifneq ($(strip $(LINUX_GUEST_ONLY)),)
linux-guest-words := $(subst --entry ,--entry@,$(strip $(LINUX_GUEST_STORE_ENTRIES)))
linux-guest-kept := $(filter-out --entry@/linux/bin/%,$(linux-guest-words)) \
	$(foreach g,$(LINUX_GUEST_ONLY),$(filter --entry@/linux/bin/$(g)=% \
		--entry@/linux/bin/$(g).%,$(linux-guest-words)))
LINUX_GUEST_STORE_ENTRIES := $(subst --entry@,--entry ,$(linux-guest-kept))
endif
