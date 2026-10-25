# Every test guest runs in the box a guest had before policies: every
# service, every call. What these guests test is the personality's own
# refusals, which hold under any box, and a narrower one would turn their
# expected answers into policy refusals. The list names each guest program
# by its store path and its ELF (file=); nonos_linux_policy's build binds it
# by BLAKE3 into the vendor table, beside policies.txt (build/table.rs).

LINUX_GUEST_POLICIES := $(TARGET_DIR)/linux-guests/policies.txt
LINUX_GUEST_ALL := WAYLAND NET_ANON NET_LISTEN_LO FILES_HOME FILES_USER MODELS_RO EXEC JIT TTY
LINUX_GUEST_POLICY_BINS := $(foreach l,$(LINUX_GUEST_POLICY_LINES),$(lastword $(subst =, ,$(l))))
$(LINUX_GUEST_POLICIES): $(LINUX_GUEST_POLICY_BINS) $(LINUX_GUESTS_DIR)/GuestPolicies.mk
	@mkdir -p $(@D)
	@: > $@
	@$(foreach l,$(LINUX_GUEST_POLICY_LINES),printf '%s file=%s $(LINUX_GUEST_ALL)\n' \
		'$(firstword $(subst =, ,$(l)))' '$(abspath $(lastword $(subst =, ,$(l))))' >> $@;)
$(linux_BIN): $(LINUX_GUEST_POLICIES)
$(linux_BIN): export NONOS_LINUX_GUEST_POLICIES := $(abspath $(LINUX_GUEST_POLICIES))
