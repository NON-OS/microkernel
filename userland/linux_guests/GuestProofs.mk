# The guests that prove the file calls, /proc and Go's os against the
# host. Included by Guests.mk.

# The file calls and the system-information calls as Linux answers them,
# part by part (cfiles). It was run on the host first through sh/oracle.sh,
# which is the oracle.
$(LINUX_GUESTS_C)/cfiles: $(wildcard $(LINUX_GUESTS_DIR)/c/cfiles/*.[ch])
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,cfiles,5020,5021,$(LINUX_GUESTS_C)/cfiles))

# /dev, /proc and /sys as a program reads them, isolation, and no host fact
# in any file (cproc, run as "cproc one two"); oracle as for cfiles.
$(LINUX_GUESTS_C)/cproc: $(wildcard $(LINUX_GUESTS_DIR)/c/cproc/*.[ch])
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,cproc,5022,5023,$(LINUX_GUESTS_C)/cproc))

# Go's os, io/fs and path/filepath with flock (goos); oracle as for cfiles.
$(GO_OUT)/goos: $(wildcard $(LINUX_GUESTS_DIR)/go/goos/*.go)
$(eval $(call LINUX_GUEST,goos,5024,5025,$(GO_OUT)/goos))
