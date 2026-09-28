# The guests that prove the file calls, /proc and Go's os against the
# host. Included by Guests.mk.

# The file calls and the system-information calls as Linux answers them,
# part by part (cfiles). It was run on the host first through sh/oracle.sh,
# which is the oracle.
$(LINUX_GUESTS_C)/cfiles: $(wildcard $(LINUX_GUESTS_DIR)/c/cfiles/*.[ch])
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,cfiles,5020,5021,$(LINUX_GUESTS_C)/cfiles))
