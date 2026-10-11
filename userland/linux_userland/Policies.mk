# The box each vendor program runs in is compiled into nonos_linux_policy's
# vendor table, bound to the program's BLAKE3 (userland/capsule_linux/
# policies.txt, read by userland/nonos_linux_policy/build.rs). So the
# personality, which builds that crate, is built after the programs the file
# names, and is told where they are, as tools/nix/capsules.nix tells the
# flake's build. A line's /bin/<name> is <name> in the userland build.

LINUX_USERLAND_POLICIES := userland/capsule_linux/policies.txt
LINUX_USERLAND_POLICY_FILES := $(shell awk '!/^\#/ && NF { sub("^/bin/", "", $$1); print $$1 }' $(LINUX_USERLAND_POLICIES))
$(linux_BIN): $(addprefix $(LINUX_USERLAND_OUT)/,$(LINUX_USERLAND_POLICY_FILES)) $(LINUX_USERLAND_POLICIES) \
	userland/nonos_linux_policy/build.rs $(wildcard userland/nonos_linux_policy/build/*.rs)
$(linux_BIN): export NONOS_LINUX_USERLAND := $(abspath $(LINUX_USERLAND_OUT))
