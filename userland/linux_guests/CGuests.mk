# C guests built against musl, each checking what Linux does in one area
# and failing loudly when the personality does otherwise. Included by
# Guests.mk.

# A C guest that faults in a worker thread while main joins: it proves the
# whole process ends, as on Linux, and that musl threads run. Static, so no
# loader is needed. musl carries pthreads in libc, so no -lpthread.
$(LINUX_GUESTS_C)/threadfault: $(LINUX_GUESTS_DIR)/c/threadfault.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,threadfault,4972,4973,$(LINUX_GUESTS_C)/threadfault))

# musl pthreads end to end: stacks reserved and committed by mprotect, a mutex,
# and joins that wait on the clear-tid word each exit zeroes and wakes.
$(LINUX_GUESTS_C)/cthreads: $(LINUX_GUESTS_DIR)/c/cthreads.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cthreads,4974,4975,$(LINUX_GUESTS_C)/cthreads))

# Waiting as Linux waits: futex timeouts and requeue, eventfd, epoll_wait's
# timeout and wake, a non-blocking pipe, a full pipe, and edge-triggered epoll.
$(LINUX_GUESTS_C)/cwait: $(wildcard $(LINUX_GUESTS_DIR)/c/cwait/*.c) $(LINUX_GUESTS_DIR)/c/cwait/cwait.h
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $(filter %.c,$^)
$(eval $(call LINUX_GUEST,cwait,4978,4979,$(LINUX_GUESTS_C)/cwait))

# A caught signal for a thread spinning with no call: it arrives only if the
# kernel stops the running thread for its supervisor.
$(LINUX_GUESTS_C)/cpreempt: $(LINUX_GUESTS_DIR)/c/cpreempt.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cpreempt,4946,4947,$(LINUX_GUESTS_C)/cpreempt))

# Bytes through a pipe between two processes: writev and one long write
# finish whole on a blocking pipe, readv reads them back, and a non-blocking
# writer stops short with EAGAIN.
$(LINUX_GUESTS_C)/cpipe: $(LINUX_GUESTS_DIR)/c/cpipe.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cpipe,4982,4983,$(LINUX_GUESTS_C)/cpipe))
