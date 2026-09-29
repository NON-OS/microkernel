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
# Every guest is built, signed and proven, but the store the vfs loads holds
# at most 16 MiB, which the whole set outgrows. LINUX_GUEST_STORE_ONLY, when
# set, names the guests the store carries, for example
#   make LINUX_GUEST_STORE_ONLY="busybox csock" LINUX_GUEST_BOOT_ARGS=... \
#        target/qemu-virtio-blk.img.store.stamp
# and unset it carries them all, as before.
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
LINUX_GUEST_ENTRIES_$(1) := --entry /linux$(or $(5),/bin/$(1))=$$(linux-guest-$(1)_BIN) \
	--entry /linux$(or $(5),/bin/$(1)).nonos_id_cert.bin=$$(linux-guest-$(1)_CERT) \
	--entry /linux$(or $(5),/bin/$(1)).manifest.bin=$$(linux-guest-$(1)_MANIFEST) \
	--entry /linux$(or $(5),/bin/$(1)).zk_trailer.bin=$$(linux-guest-$(1)_ATTESTATION)
# Every guest, or only those LINUX_GUEST_STORE_ONLY names: vfs loads 16 MiB at most.
LINUX_GUEST_STORE_ENTRIES += $$(if $$(filter $(1),$$(or $$(LINUX_GUEST_STORE_ONLY),$(1))),$$(LINUX_GUEST_ENTRIES_$(1)))
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
# net/http inside one guest: a server on 127.0.0.1:0 and its client, twenty
# GETs over one kept-alive connection.
$(eval $(call LINUX_GUEST,gohttp,5002,5003,$(GO_OUT)/http))

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

# Process lifecycle and signals, each against Linux; see LifeGuests.mk.
include $(LINUX_GUESTS_DIR)/LifeGuests.mk
# Qwen, pinned by the personality, checked token for token; see QwenGuest.mk.
include $(LINUX_GUESTS_DIR)/QwenGuest.mk
# Waiting as Linux waits: futex timeouts and requeue, eventfd, epoll_wait's
# timeout and wake, a non-blocking pipe, a full pipe, and edge-triggered epoll.
$(LINUX_GUESTS_C)/cwait: $(LINUX_GUESTS_DIR)/c/cwait.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cwait,4978,4979,$(LINUX_GUESTS_C)/cwait))

# Sockets as Linux has them, on the family's own loopback: socketpair, a
# listener with accept4's flags, a non-blocking connect, a refused port,
# half-close, epoll on a listener, end of file, EAGAIN, MSG_PEEK, EPIPE, an
# accept and a receive that wait, the options a server sets, and fork.
$(LINUX_GUESTS_C)/csock: $(LINUX_GUESTS_DIR)/c/csock.c $(wildcard $(LINUX_GUESTS_DIR)/c/csock_*.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,csock,5000,5001,$(LINUX_GUESTS_C)/csock))

# Datagrams on the family's loopback: an echo, a connected socket, MSG_TRUNC,
# a refused port, sendmmsg and recvmmsg, and no peer at all.
$(LINUX_GUESTS_C)/cudp: $(LINUX_GUESTS_DIR)/c/cudp.c $(wildcard $(LINUX_GUESTS_DIR)/c/cudp_*.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cudp,5004,5005,$(LINUX_GUESTS_C)/cudp))

# What a guest's sockets may reach, and what a descriptor number alone gets.
$(LINUX_GUESTS_C)/cpolicy: $(LINUX_GUESTS_DIR)/c/cpolicy.c $(wildcard $(LINUX_GUESTS_DIR)/c/cpolicy_*.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cpolicy,5006,5007,$(LINUX_GUESTS_C)/cpolicy))

# A guest blocked in accept with nothing happening, for the loop's wakeups.
$(LINUX_GUESTS_C)/cidle: $(LINUX_GUESTS_DIR)/c/cidle.c $(wildcard $(LINUX_GUESTS_DIR)/c/cidle_*.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cidle,5008,5009,$(LINUX_GUESTS_C)/cidle))

# Unix sockets with names: a path, what it leaves behind, abstract names,
# a connected datagram socket, autobind, and a connection across fork.
$(LINUX_GUESTS_C)/cunix: $(LINUX_GUESTS_DIR)/c/cunix.c $(wildcard $(LINUX_GUESTS_DIR)/c/cunix_*.h)
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,cunix,5010,5011,$(LINUX_GUESTS_C)/cunix))

# The memory proofs, one program whose first argument names the proof, so the
# store carries one binary and one set of proofs for all five:
#   guardpage  a pthread recursing into its guard page ends on SIGSEGV, 139
#   protnone   PROT_NONE means no access, and bytes survive a close and reopen
#   protfork   a fork after mprotect gives the child the protection set now
#   touchfork  bytes written into a reservation survive a fork; MAP_FIXED
#              over a written page replaces it with zeroes
#   memcalls   mmap placement, brk, alignment, mremap, and mlock, msync and
#              mincore, each against Linux's answer
MEMPROOF_PARTS := guardpage protnone protfork touchfork memcalls escape
# Files the proofs share, built as they are: no main of their own.
MEMPROOF_SHARED := memproof_run memcalls_map memcalls_remap memcalls_lock escape_map escape_exec
MEMPROOF_SRCS := $(foreach p,memproof $(MEMPROOF_PARTS) $(MEMPROOF_SHARED),\
	$(LINUX_GUESTS_DIR)/c/$(p).c) $(LINUX_GUESTS_DIR)/c/memproof.h $(LINUX_GUESTS_DIR)/c/memcalls.h $(LINUX_GUESTS_DIR)/c/escape.h
$(LINUX_GUESTS_C)/memproof: $(MEMPROOF_SRCS)
	@mkdir -p $(@D)/memproof.o
	@for p in $(MEMPROOF_PARTS); do \
		musl-gcc -O2 -c -Dmain=$${p}_main -o $(@D)/memproof.o/$$p.o $(LINUX_GUESTS_DIR)/c/$$p.c || exit 1; \
	done
	@for p in $(MEMPROOF_SHARED); do \
		musl-gcc -O2 -c -o $(@D)/memproof.o/$$p.o $(LINUX_GUESTS_DIR)/c/$$p.c || exit 1; \
	done
	@musl-gcc -O2 -static -o $@ $(LINUX_GUESTS_DIR)/c/memproof.c \
		$(foreach p,$(MEMPROOF_PARTS) $(MEMPROOF_SHARED),$(@D)/memproof.o/$(p).o)
$(eval $(call LINUX_GUEST,memproof,4980,4981,$(LINUX_GUESTS_C)/memproof))

# Go's own standard-library tests as guests, opt in (GoSuite.mk).
include $(LINUX_GUESTS_DIR)/GoSuite.mk

# The Linux-guest test store is about guests, not the desktop's media and demo
# capsules. Drop both so the signed guest set fits the vfs load budget; the
# normal image, which does not set NONOS_LINUX_GUESTS, still ships them.
override NONOS_STORE_MEDIA_ENTRIES :=
override NONOS_STORE_DEMO_ENTRIES :=

include $(LINUX_GUESTS_DIR)/GuestFiles.mk
include $(LINUX_GUESTS_DIR)/GuestProofs.mk
include $(LINUX_GUESTS_DIR)/GuestOnly.mk
