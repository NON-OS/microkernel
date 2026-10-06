# The seam between the capsule declarations and the flake.
#
# Every capsule is declared once, in its userland/*/Capsule.mk, and the
# image's package store once, in mk/40-run.mk and Userland.mk. The flake
# (flake.nix, tools/nix/) builds the capsules and tools/nonos-seal signs and
# packs them; both read these declarations through the two JSON files below,
# which these targets print. tools/nix/capsules.json and tools/nix/store.json
# are their output, committed (python3 tools/nix/catalogues.py writes them),
# and `nix flake check` regenerates both and fails on any difference, so the
# declarations and the build cannot drift apart.

.PHONY: nonos-mk-capsule-catalogue nonos-mk-store-catalogue

# A make value as a JSON string: backslashes and quotes escaped.
nonos_json = "$(subst ",\",$(subst \,\\,$(strip $(1))))"

# One object per enrolled program, in the order make declares them: what the
# flake needs to compile it and what the seal needs to sign and enroll it.
define nonos_catalogue_entry
{"slug":$(call nonos_json,$(1)),"dir":$(call nonos_json,$($(1)_DIR)),\
"bin":$(call nonos_json,$($(1)_BIN_NAME)),"target":$(call nonos_json,$($(1)_TARGET)),\
"build_std":$(call nonos_json,$($(1)_BUILD_STD)),"build_std_features":$(call nonos_json,$($(1)_BUILD_STD_FEAT)),\
"needs_rt":$(if $($(1)_NEEDS_RT),true,false),"cargo_features":$(call nonos_json,$($(1)_CARGO_FEATURES)),\
"prebuilt":$(call nonos_json,$($(1)_PREBUILT_BIN)),"feature":$(call nonos_json,$($(1)_FEATURE)),\
"kernel_mirror":$(call nonos_json,$($(1)_KERNEL_MIRROR)),\
"handle":$(call nonos_json,$($(1)_HANDLE)),"domain":$(call nonos_json,$($(1)_DOMAIN)),\
"recovery":$(call nonos_json,$($(1)_RECOVERY)),"namespace":$(call nonos_json,$($(1)_NAMESPACE)),\
"service_endpoint":$(call nonos_json,$($(1)_SERVICE_ENDPOINT)),\
"reply_endpoint":$(call nonos_json,$($(1)_REPLY_ENDPOINT)),\
"instance_endpoints":$(call nonos_json,$(patsubst --endpoint,,$($(1)_INSTANCE_ENDPOINT_FLAGS))),\
"required_caps":$(call nonos_json,$($(1)_REQUIRED_CAPS)),"optional_caps":$(call nonos_json,$($(1)_OPTIONAL_CAPS)),\
"caps_ceiling":$(call nonos_json,$($(1)_CAPS_CEILING)),"version":$(call nonos_json,$($(1)_VERSION)),\
"serial":$(call nonos_json,$($(1)_SERIAL)),"metadata":$(call nonos_json,$($(1)_METADATA)),\
"pub_ed25519":$(call nonos_json,$($(1)_KEY_ED_PUB)),"pub_mldsa65":$(call nonos_json,$($(1)_KEY_MLDSA_PUB)),\
"seed_ed25519":$(call nonos_json,$($(1)_KEY_ED_SEED)),"seed_mldsa65":$(call nonos_json,$($(1)_KEY_MLDSA_SEED)),\
"cert":$(call nonos_json,$($(1)_CERT)),"manifest":$(call nonos_json,$($(1)_MANIFEST)),\
"trailer":$(call nonos_json,$($(1)_ATTESTATION)),\
"dev_only":$(if $($(1)_DEV_ONLY),true,false)}
endef

# The development tests (CAPSULE_DEV_ONLY) follow, marked dev_only.
NONOS_CATALOGUE_CAPSULES = $(strip $(NONOS_VERIFIED_CAPSULES) $(NONOS_DEV_CAPSULES))

# Each entry is a recipe line of its own: one command for every capsule would
# pass the 128 KiB a single argument may take.
define nonos_catalogue_print
@printf '%s%s\n' '$(call nonos_catalogue_entry,$(1))' \
	'$(if $(filter $(1),$(lastword $(NONOS_CATALOGUE_CAPSULES))),,$(_boot_comma))'

endef

nonos-mk-capsule-catalogue:
	@printf '[\n'
	$(foreach s,$(NONOS_CATALOGUE_CAPSULES),$(call nonos_catalogue_print,$(s)))
	@printf ']\n'

# The package store, by group, each entry a path in the store and the file
# that fills it, as tools/nonos-store-pack takes them. The seal chooses the
# groups nonos.toml asks for.
nonos_store_group = $(call nonos_json,$(strip $(subst --entry ,,$(1))))

# The Linux packages follow the groups: each program the image does not carry
# (LINUX_USERLAND_PACKAGE in userland/linux_userland/Userland.mk), by name,
# with its files in the same form. The seal publishes them for the
# Marketplace's Linux tab.
define nonos_package_print
@printf '%s %s: %s\n' '$(if $(filter $(1),$(firstword $(LINUX_USERLAND_PACKAGES))),,$(_boot_comma))' \
	'$(call nonos_json,$(1))' '$(call nonos_json,$(LINUX_USERLAND_PACKAGE_$(1)))'

endef

nonos-mk-store-catalogue:
	@printf '{\n "demo": %s,\n "media": %s,\n "wallpapers": %s,\n "linux": %s,\n "packages": {\n' \
		'$(call nonos_store_group,$(NONOS_STORE_DEMO_ENTRIES))' \
		'$(call nonos_store_group,$(NONOS_STORE_MEDIA_ENTRIES))' \
		'$(call nonos_store_group,$(NONOS_STORE_WALLPAPER_ENTRIES))' \
		'$(call nonos_store_group,$(LINUX_USERLAND_STORE_ENTRIES))'
	$(foreach p,$(LINUX_USERLAND_PACKAGES),$(call nonos_package_print,$(p)))
	@printf ' }\n}\n'
