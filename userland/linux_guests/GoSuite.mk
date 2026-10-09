# The Go standard-library suite's guests, included by Guests.mk.

# Go's own standard-library tests as guests, opt in with NONOS_LINUX_GO_SUITE=1,
# so the default build does not grow: each test binary is the one `go test -c`
# makes, and the same bytes are run on the build host for comparison.
# NONOS_LINUX_GO_SUITE_PKGS names the packages enrolled; each is the guest
# gs<package without slashes>, ids 5042 upward in list order, 29 at most in
# the 5040 to 5099 range. gostd (5040), a static C program, changes to the
# package's directory and becomes its test binary, since go test runs each one
# there, beside testdata/.
ifeq ($(NONOS_LINUX_GO_SUITE),1)
NONOS_LINUX_GO_SUITE_PKGS ?= sync time os
GO_STD_OUT := $(TARGET_DIR)/linux-guests/go-std
GO_ROOT := $(shell $(GO) env GOROOT)
ifneq ($(word 30,$(NONOS_LINUX_GO_SUITE_PKGS)),)
$(error NONOS_LINUX_GO_SUITE_PKGS names more than the 29 packages ids 5042 to 5099 hold)
endif
# The suite's ids, a service and a reply per guest: gostd's pair, then each
# package's in list order, then execbig's. scripts/check_capsule_ports.py
# holds the whole range for this file.
GO_SUITE_IDS := $(shell seq 5040 5099)
go_suite_id = $(word $(1),$(GO_SUITE_IDS))
$(GO_STD_OUT)/%.test: $(GO)
	@mkdir -p $(@D) && CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
		GOCACHE=$(abspath $(GO_OUT))/cache GOPATH=$(abspath $(GO_OUT))/path \
		$(GO) test -c -o $(abspath $@) $(subst _,/,$*)
$(LINUX_GUESTS_C)/gostd: $(LINUX_GUESTS_DIR)/go/std/gostd.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,gostd,$(call go_suite_id,1),$(call go_suite_id,2),$(LINUX_GUESTS_C)/gostd))
$(foreach i,$(shell seq 1 $(words $(NONOS_LINUX_GO_SUITE_PKGS))),$(eval $(call LINUX_GUEST,gs$(subst /,,$(word $(i),$(NONOS_LINUX_GO_SUITE_PKGS))),$(call go_suite_id,$(shell expr 1 + 2 \* $(i))),$(call go_suite_id,$(shell expr 2 + 2 \* $(i))),$(GO_STD_OUT)/$(subst /,_,$(word $(i),$(NONOS_LINUX_GO_SUITE_PKGS))).test)))
# A 12 MB program that execs itself: it runs only if the personality lets the
# first program's bytes go once it is running. It takes the ids after the
# packages, so the list is one shorter when it is asked for.
ifeq ($(NONOS_LINUX_GO_SUITE_EXECBIG),1)
GO_SUITE_EXECBIG_AT := $(shell expr 3 + 2 \* $(words $(NONOS_LINUX_GO_SUITE_PKGS)))
ifeq ($(call go_suite_id,$(shell expr 1 + $(GO_SUITE_EXECBIG_AT))),)
$(error NONOS_LINUX_GO_SUITE_EXECBIG=1 needs a free id pair; name at most 28 packages)
endif
$(LINUX_GUESTS_C)/execbig: $(LINUX_GUESTS_DIR)/go/std/execbig.c
	@mkdir -p $(@D) && musl-gcc -O2 -static -o $@ $<
$(eval $(call LINUX_GUEST,execbig,$(call go_suite_id,$(GO_SUITE_EXECBIG_AT)),$(call go_suite_id,$(shell expr 1 + $(GO_SUITE_EXECBIG_AT))),$(LINUX_GUESTS_C)/execbig))
endif
endif
