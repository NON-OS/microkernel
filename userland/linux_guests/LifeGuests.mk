# Guests for process lifecycle and signals. Included by Guests.mk.
#
# Process lifecycle and signals, each against Linux: a leader's plain exit
# that leaves its worker running, os/exec from Go through clone(CLONE_VFORK),
# SIGCHLD with waitid and wait4, SIGPIPE on a widowed pipe, and SIGALRM ending
# a sleep, with the timer and signal-wait calls.
$(LINUX_GUESTS_C)/leaderexit: $(LINUX_GUESTS_DIR)/c/leaderexit.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,leaderexit,4990,4991,$(LINUX_GUESTS_C)/leaderexit))
$(GO_OUT)/exec: $(LINUX_GUESTS_DIR)/go/exec/forkexec.go
$(eval $(call LINUX_GUEST,goexec,4992,4993,$(GO_OUT)/exec))
$(LINUX_GUESTS_C)/sigchld: $(addprefix $(LINUX_GUESTS_DIR)/c/,sigchld.c sigchld_wait.c sigchld_spawn.c sigchld.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,sigchld,4994,4995,$(LINUX_GUESTS_C)/sigchld))
$(LINUX_GUESTS_C)/sigpipe: $(LINUX_GUESTS_DIR)/c/sigpipe.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,sigpipe,4996,4997,$(LINUX_GUESTS_C)/sigpipe))
$(LINUX_GUESTS_C)/alarm: $(addprefix $(LINUX_GUESTS_DIR)/c/,alarm.c alarm_wait.c alarm_timer.c alarm.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,alarm,4998,4999,$(LINUX_GUESTS_C)/alarm))
