# NØNOS shared capsule build/sign/verify macro.
#
# A `userland/<capsule>/Capsule.mk` declares the capsule's identity
# in CAPSULE_* variables and then `include nonos-mk/capsule.mk` to
# materialise the standard target set:
#
#   nonos-mk-<slug>            build the userland ELF
#   nonos-mk-<slug>-sign       sign cert + manifest under the baked
#                              trust-anchor policy
#   nonos-mk-check-<slug>-keys assert the publisher seeds + pubs are
#                              present
#   $(<slug>_BIN)              path to the userland ELF
#   $(<slug>_CERT)             path to the NØNOS-ID cert blob
#   $(<slug>_MANIFEST)         path to the CapsuleManifest v3 blob
#   $(<slug>_ARTIFACTS)        union of the three above
#
# All capsule-specific cert/manifest/key/endpoint/caps values live
# in the per-capsule Capsule.mk; the root Makefile is forbidden by
# the static gate from defining any of those.
#
# Implementation note: the per-capsule rules are wrapped in a
# `define ... endef` block and instantiated through `$(call ...)`
# so the slug is interpolated into the rule text at parse time.
# Without that, recipe text like `$(CAPSULE_SLUG)_HANDLE` would
# resolve to the *last* value of `CAPSULE_SLUG` (clobbered by the
# next include), breaking every capsule but the most recent.

ifndef CAPSULE_SLUG
$(error CAPSULE_SLUG must be set before including nonos-mk/capsule.mk)
endif
ifndef CAPSULE_BIN_NAME
$(error CAPSULE_BIN_NAME must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_DIR
$(error CAPSULE_DIR must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_HANDLE
$(error CAPSULE_HANDLE must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_DOMAIN
$(error CAPSULE_DOMAIN must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_NAMESPACE
$(error CAPSULE_NAMESPACE must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_SERVICE_ENDPOINT
$(error CAPSULE_SERVICE_ENDPOINT must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_REPLY_ENDPOINT
$(error CAPSULE_REPLY_ENDPOINT must be set in $(CAPSULE_SLUG) Capsule.mk)
endif
ifndef CAPSULE_REQUIRED_CAPS
$(error CAPSULE_REQUIRED_CAPS must be set in $(CAPSULE_SLUG) Capsule.mk)
endif

# The committed trust-bundle lives at `nonos-data/trust/`:
#   keys/    publisher .pub files (committed)
#   capsules/  *.nonos_id_cert.bin and *.manifest.bin (committed)
#   policy/   nonos_trust_anchor.policy.bin (committed)
# Seeds stay in `.keys/` (gitignored). The macro reads pubs from
# the committed `keys/` directory and writes cert + manifest into
# the committed `capsules/` directory so a clean checkout has
# everything the kernel verifier needs.
NONOS_BAKED_TRUST_DIR ?= nonos-data/trust

# Effective values with `$(or)` fallbacks. The reset block at the bottom of
# this file undefines every input after each capsule, and `$(or)` gives each
# its default whenever it is unset or empty.
_NONOS_CAPSULE_OPTIONAL_CAPS := $(or $(CAPSULE_OPTIONAL_CAPS),0x0)
# The certificate's ceiling bounds every capability the manifest may name, the
# optional ones included, so by default it is required | optional. The spawn
# gate installs an optional bit only when the build grants it, so this widens
# what may be granted, never what is.
_NONOS_CAPSULE_CAPS_CEILING  := $(or $(CAPSULE_CAPS_CEILING),$(shell printf '0x%X' $$(( $(CAPSULE_REQUIRED_CAPS) | $(_NONOS_CAPSULE_OPTIONAL_CAPS) ))))
_NONOS_CAPSULE_TARGET        := $(or $(CAPSULE_TARGET),x86_64-nonos-user)
_NONOS_CAPSULE_VERSION       := $(or $(CAPSULE_VERSION),0.1.0)
_NONOS_CAPSULE_SERIAL        := $(or $(CAPSULE_ID_CERT_SERIAL),1)
# `$(or X,core,alloc)` parses as three args (X / core / alloc),
# returning `core` instead of the literal `core,alloc`. Use a
# comma variable so the default reaches `-Zbuild-std=` as one
# scalar.
_NONOS_CAPSULE_COMMA         := ,
_NONOS_CAPSULE_BUILD_STD     := $(or $(CAPSULE_BUILD_STD),core$(_NONOS_CAPSULE_COMMA)alloc)
_NONOS_CAPSULE_BUILD_STD_FEAT := $(or $(CAPSULE_BUILD_STD_FEATURES),compiler-builtins-mem)
_NONOS_CAPSULE_KEY_PUB_PREFIX := $(or $(CAPSULE_KEY_PUB_PREFIX),$(NONOS_BAKED_TRUST_DIR)/keys/$(CAPSULE_BIN_NAME)_publisher)
_NONOS_CAPSULE_KEY_SEED_PREFIX := $(or $(CAPSULE_KEY_SEED_PREFIX),.keys/$(CAPSULE_BIN_NAME)_publisher)
_NONOS_CAPSULE_METADATA      := $(or $(CAPSULE_METADATA),NØNOS $(CAPSULE_HANDLE) publisher v1)
_NONOS_CAPSULE_FEATURE       := $(or $(CAPSULE_FEATURE),nonos-capsule-$(CAPSULE_SLUG))
_NONOS_CAPSULE_CARGO_FEATURES := $(strip $(CAPSULE_CARGO_FEATURES))
_NONOS_CAPSULE_EXTRA_DEPS    := $(strip $(CAPSULE_EXTRA_DEPS))
_NONOS_CAPSULE_EXTRA_ORDER_DEPS := $(strip $(CAPSULE_EXTRA_ORDER_DEPS))
_NONOS_CAPSULE_PREBUILT_BIN  := $(strip $(CAPSULE_PREBUILT_BIN))

# Slug-namespaced variable snapshot. These are evaluated at
# include time so subsequent Capsule.mk files (which reassign
# CAPSULE_*) cannot disturb a prior capsule's metadata.
$(CAPSULE_SLUG)_DIR              := $(CAPSULE_DIR)
$(CAPSULE_SLUG)_BIN_NAME         := $(CAPSULE_BIN_NAME)
$(CAPSULE_SLUG)_TARGET           := $(_NONOS_CAPSULE_TARGET)
$(CAPSULE_SLUG)_BIN              := $(CAPSULE_DIR)/target/$(_NONOS_CAPSULE_TARGET)/release/$(CAPSULE_BIN_NAME)
$(CAPSULE_SLUG)_CERT             := $(NONOS_BAKED_TRUST_DIR)/capsules/$(CAPSULE_BIN_NAME).nonos_id_cert.bin
$(CAPSULE_SLUG)_MANIFEST         := $(NONOS_BAKED_TRUST_DIR)/capsules/$(CAPSULE_BIN_NAME).manifest.bin
$(CAPSULE_SLUG)_ATTESTATION      := $(NONOS_BAKED_TRUST_DIR)/capsules/$(CAPSULE_BIN_NAME).zk_trailer.bin
$(CAPSULE_SLUG)_ATTEST_PROOF     := $(TARGET_DIR)/capsule-attest/$(CAPSULE_BIN_NAME).proof.bin
$(CAPSULE_SLUG)_ATTEST_CTX       := $(TARGET_DIR)/capsule-attest/$(CAPSULE_BIN_NAME).ctx.bin
$(CAPSULE_SLUG)_STALE            := $(NONOS_BAKED_TRUST_DIR)/capsules/$(CAPSULE_BIN_NAME).STALE
$(CAPSULE_SLUG)_KEY_ED_SEED      := $(_NONOS_CAPSULE_KEY_SEED_PREFIX)_ed25519.seed
$(CAPSULE_SLUG)_KEY_ED_PUB       := $(_NONOS_CAPSULE_KEY_PUB_PREFIX)_ed25519.pub
$(CAPSULE_SLUG)_KEY_MLDSA_SEED   := $(_NONOS_CAPSULE_KEY_SEED_PREFIX)_mldsa65.seed
$(CAPSULE_SLUG)_KEY_MLDSA_PUB    := $(_NONOS_CAPSULE_KEY_PUB_PREFIX)_mldsa65.pub
$(CAPSULE_SLUG)_HANDLE           := $(CAPSULE_HANDLE)
$(CAPSULE_SLUG)_DOMAIN           := $(CAPSULE_DOMAIN)
$(CAPSULE_SLUG)_RECOVERY         := $(CAPSULE_RECOVERY)
$(CAPSULE_SLUG)_NAMESPACE        := $(CAPSULE_NAMESPACE)
$(CAPSULE_SLUG)_SERVICE_ENDPOINT := $(CAPSULE_SERVICE_ENDPOINT)
$(CAPSULE_SLUG)_REPLY_ENDPOINT   := $(CAPSULE_REPLY_ENDPOINT)
# Extra endpoints for on-demand instances (terminal, browser): a space
# separated list of kind:port:name specs, each declared in the signed
# manifest so a runtime-spawned instance can register its own window.
$(CAPSULE_SLUG)_INSTANCE_ENDPOINT_FLAGS := $(foreach ep,$(CAPSULE_INSTANCE_ENDPOINTS),--endpoint $(ep))
$(CAPSULE_SLUG)_REQUIRED_CAPS    := $(CAPSULE_REQUIRED_CAPS)
$(CAPSULE_SLUG)_OPTIONAL_CAPS    := $(_NONOS_CAPSULE_OPTIONAL_CAPS)
$(CAPSULE_SLUG)_CAPS_CEILING     := $(_NONOS_CAPSULE_CAPS_CEILING)
$(CAPSULE_SLUG)_VERSION          := $(_NONOS_CAPSULE_VERSION)
$(CAPSULE_SLUG)_SERIAL           := $(_NONOS_CAPSULE_SERIAL)
$(CAPSULE_SLUG)_BUILD_STD        := $(_NONOS_CAPSULE_BUILD_STD)
$(CAPSULE_SLUG)_BUILD_STD_FEAT   := $(_NONOS_CAPSULE_BUILD_STD_FEAT)
$(CAPSULE_SLUG)_NEEDS_RT         := $(findstring std,$(_NONOS_CAPSULE_BUILD_STD))
$(CAPSULE_SLUG)_RUSTFLAGS        := $(if $(findstring std,$(_NONOS_CAPSULE_BUILD_STD)),-Clink-arg=$(NONOS_RT_OBJ),)
$(CAPSULE_SLUG)_METADATA         := $(_NONOS_CAPSULE_METADATA)
$(CAPSULE_SLUG)_FEATURE          := $(_NONOS_CAPSULE_FEATURE)
$(CAPSULE_SLUG)_CARGO_FEATURES   := $(_NONOS_CAPSULE_CARGO_FEATURES)
$(CAPSULE_SLUG)_PREBUILT_BIN     := $(_NONOS_CAPSULE_PREBUILT_BIN)
$(CAPSULE_SLUG)_KERNEL_MIRROR    := $(CAPSULE_KERNEL_MIRROR)
$(CAPSULE_SLUG)_DEV_ONLY         := $(strip $(CAPSULE_DEV_ONLY))
# The file that declared this capsule, so editing it re-signs. A directory
# declaring several (userland/linux_guests) names its own file.
$(CAPSULE_SLUG)_CAPSULE_MK       := $(or $(CAPSULE_MK_FILE),$(CAPSULE_DIR)/Capsule.mk)
$(CAPSULE_SLUG)_CARGO_TOML       := $(CAPSULE_DIR)/Cargo.toml
$(CAPSULE_SLUG)_CARGO_LOCK       := $(wildcard $(CAPSULE_DIR)/Cargo.lock)
$(CAPSULE_SLUG)_SOURCES          := $(shell find $(CAPSULE_DIR)/src -type f -name '*.rs' 2>/dev/null | sort)
$(CAPSULE_SLUG)_EXTRA_DEPS       := $(_NONOS_CAPSULE_EXTRA_DEPS)
$(CAPSULE_SLUG)_EXTRA_ORDER_DEPS := $(_NONOS_CAPSULE_EXTRA_ORDER_DEPS)
$(CAPSULE_SLUG)_ARTIFACTS        := $($(CAPSULE_SLUG)_BIN) $($(CAPSULE_SLUG)_CERT) $($(CAPSULE_SLUG)_MANIFEST) $($(CAPSULE_SLUG)_ATTESTATION)
$(CAPSULE_SLUG)_VERIFY           := nonos-mk-$(CAPSULE_SLUG)-verify

# Shared userland library crates (those exposing a src/lib.rs) are
# compiled inline into every capsule by cargo, so a change in any of
# them must rebuild the dependent capsule. The per-capsule src glob
# above cannot express that, so track the union of their sources once
# and add it to every capsule ELF prerequisite below. Over-declaring a
# crate a capsule never links only forces a harmless rebuild; missing
# one ships a stale binary, which is exactly the trap this prevents.
ifndef NONOS_CAPSULE_SHARED_SRCS
NONOS_CAPSULE_SHARED_SRCS := $(shell find $(patsubst %/lib.rs,%,$(wildcard userland/*/src/lib.rs)) -type f -name '*.rs' 2>/dev/null | sort)
endif

# Track all capsules that have included this macro so the root
# Makefile can iterate them through `$(NONOS_VERIFIED_CAPSULES)`.
# `:=` plus explicit reuse forces eager append: `+=` against a
# recursive variable would expand `$(CAPSULE_SLUG)` after the reset
# block clears it, leaving the list empty.
# A capsule that sets CAPSULE_DEV_ONLY is a development test: it goes on
# NONOS_DEV_CAPSULES instead, which no make lane signs or enrolls, and the
# flake's seal enrolls it only for a development image (tools/nonos_seal).
ifeq ($(strip $(CAPSULE_DEV_ONLY)),)
NONOS_VERIFIED_CAPSULES := $(NONOS_VERIFIED_CAPSULES) $(CAPSULE_SLUG)
else
NONOS_DEV_CAPSULES := $(NONOS_DEV_CAPSULES) $(CAPSULE_SLUG)
endif

# Per-capsule rules. The slug ($1) is interpolated at parse time
# through `$(eval $(call ...))`; literal `$$` survives `eval` to
# become a runtime `$` in the recipe.
define NONOS_CAPSULE_RULES

.PHONY: nonos-mk-$(1) nonos-mk-$(1)-sign nonos-mk-$(1)-verify nonos-mk-check-$(1)-keys

ifneq ($$($(1)_PREBUILT_BIN),)
$$($(1)_BIN): $$($(1)_PREBUILT_BIN) $$($(1)_CAPSULE_MK)
	@echo "Installing prebuilt $$($(1)_BIN_NAME) capsule..."
	@mkdir -p "$$(@D)"
	@cp "$$<" "$$@"
	@touch "$$@"
else
$$($(1)_BIN): $$(USERLAND_LIBC) $$($(1)_CAPSULE_MK) \
               $$($(1)_CARGO_TOML) $$($(1)_CARGO_LOCK) $$($(1)_SOURCES) \
               $$(NONOS_CAPSULE_SHARED_SRCS) $$($(1)_EXTRA_DEPS) \
               $$(if $$($(1)_NEEDS_RT),$$(NONOS_RT_OBJ) $$(NONOS_STD_PAL_STAMP),) \
               | $$($(1)_EXTRA_ORDER_DEPS)
	@echo "Building $$($(1)_BIN_NAME) capsule..."
	@cd $$($(1)_DIR) && \
		RUSTUP_TOOLCHAIN=$$(TOOLCHAIN) \
		$$(if $$($(1)_RUSTFLAGS),RUSTFLAGS="$$($(1)_RUSTFLAGS)",) \
		$$(CARGO) build --release --target ../x86_64-nonos-user.json \
		$$(if $$($(1)_CARGO_FEATURES),--features $$($(1)_CARGO_FEATURES),) \
		-Zbuild-std=$$($(1)_BUILD_STD) \
		$$(if $$($(1)_BUILD_STD_FEAT),-Zbuild-std-features=$$($(1)_BUILD_STD_FEAT),)
	@touch $$@
endif

nonos-mk-$(1): $$($(1)_BIN)

nonos-mk-check-$(1)-keys:
	@for f in $$($(1)_KEY_ED_SEED) $$($(1)_KEY_ED_PUB) \
	          $$($(1)_KEY_MLDSA_SEED) $$($(1)_KEY_MLDSA_PUB); do \
	    [ -f "$$$$f" ] || { \
	        echo "::error::missing $$$$f; generate it with: $$(CAPSULE_SIGN_BIN) keygen --alg <ed25519|mldsa65> --out <prefix>"; \
	        exit 1; \
	    }; \
	done

# nonos_id is BLAKE3(handle/domain/recovery); recompute every sign
# so a rename of the cert file cannot silently change the identity.
$(1)_NONOS_ID_HEX = $$(shell $$(CAPSULE_SIGN_BIN) derive-id \
	--handle $$($(1)_HANDLE) \
	--domain $$($(1)_DOMAIN) \
	--recovery "$$($(1)_RECOVERY)")

# Under NONOS_TRUST_REUSE the committed artifacts are the truth and no
# signing runs: the pipeline holds no seeds by design. Each capsule's
# rule instead proves the committed certificate and manifest verify
# under the baked policy and that the freshly built ELF measures to
# the enrolled payload hash. A capsule that drifts fails by name; the
# cure is making its build reproducible, never minting a new seed.
ifeq ($$(NONOS_TRUST_REUSE),1)

$$($(1)_CERT): $$(NONOS_TRUST_ANCHOR_POLICY_BIN) $$(CAPSULE_SIGN_BIN)
	@test -f $$@ || { echo "::error::$$@ is not committed; a reuse build cannot mint it"; exit 1; }

$$($(1)_MANIFEST): $$($(1)_BIN) $$($(1)_CERT) $$(CAPSULE_SIGN_BIN)
	@test -f $$@ || { echo "::error::$$@ is not committed; a reuse build cannot mint it"; exit 1; }
ifeq ($$(NONOS_ENROLL_BUILD),1)
	@# The enrollment builder produces the candidate bytes a ceremony
	@# will sign; verifying them against the enrollment they exist to
	@# replace would refuse every legitimate rebuild. The committed
	@# manifest is still required to exist for consumers like the
	@# marketplace index.
	@touch $$@
else
	@echo "Verifying $$($(1)_HANDLE) against the enrolled trust set..."
	@$$(CAPSULE_SIGN_BIN) verify-manifest \
		--manifest $$($(1)_MANIFEST) \
		--cert $$($(1)_CERT) \
		--policy $$(NONOS_TRUST_ANCHOR_POLICY_BIN) \
		--elf $$($(1)_BIN) >/dev/null
	@touch $$@
endif

else

$$($(1)_CERT): $$(NONOS_TRUST_ANCHOR_POLICY_BIN) $$($(1)_CAPSULE_MK) \
               $$($(1)_KEY_ED_PUB) $$($(1)_KEY_MLDSA_PUB) \
               $$(CAPSULE_SIGN_BIN) | nonos-mk-check-trust-keys nonos-mk-check-$(1)-keys
	@$$(NONOS_PYTHON) scripts/check_device_secret_cap.py --slug $(1) \
		--caps $$($(1)_CAPS_CEILING) $$($(1)_REQUIRED_CAPS) $$($(1)_OPTIONAL_CAPS)
	@echo "Signing $$($(1)_HANDLE) NØNOS-ID certificate (hybrid)..."
	@$$(CAPSULE_SIGN_BIN) sign-id-cert \
		--serial $$($(1)_SERIAL) \
		--nonos-id $$($(1)_NONOS_ID_HEX) \
		--ns-glob $$($(1)_NAMESPACE) \
		--caps-ceiling $$($(1)_CAPS_CEILING) \
		--epoch $$(NONOS_TRUST_ANCHOR_EPOCH) \
		--valid-from-ms  $$(NONOS_CERT_VALID_FROM_MS) \
		--valid-until-ms $$(NONOS_CERT_VALID_UNTIL_MS) \
		--pub-key ed25519=$$($(1)_KEY_ED_PUB) \
		--pub-key mldsa65=$$($(1)_KEY_MLDSA_PUB) \
		--ta-seed ed25519=$$(NONOS_TA_ED25519_SEED) \
		--ta-seed mldsa65=$$(NONOS_TA_MLDSA65_SEED) \
		--metadata "$$($(1)_METADATA)" \
		--out $$@

# Manifest depends on the capsule ELF: rebuilding the ELF forces
# a manifest re-sign so payload_hash stays in sync.
$$($(1)_MANIFEST): $$($(1)_BIN) $$($(1)_CERT) $$($(1)_CAPSULE_MK) \
                   $$(CAPSULE_SIGN_BIN) | nonos-mk-check-$(1)-keys
	@$$(NONOS_PYTHON) scripts/check_device_secret_cap.py --slug $(1) \
		--caps $$($(1)_CAPS_CEILING) $$($(1)_REQUIRED_CAPS) $$($(1)_OPTIONAL_CAPS)
	@echo "Signing $$($(1)_HANDLE) capsule manifest..."
	@$$(CAPSULE_SIGN_BIN) sign-manifest \
		--cert $$($(1)_CERT) \
		--namespace $$($(1)_NAMESPACE) \
		--version $$($(1)_VERSION) \
		--target $$($(1)_TARGET) \
		--elf $$($(1)_BIN) \
		--required-caps $$($(1)_REQUIRED_CAPS) \
		--optional-caps $$($(1)_OPTIONAL_CAPS) \
		--endpoint $$($(1)_SERVICE_ENDPOINT) \
		--endpoint $$($(1)_REPLY_ENDPOINT) \
		$$($(1)_INSTANCE_ENDPOINT_FLAGS) \
		--pub-seed ed25519=$$($(1)_KEY_ED_SEED) \
		--pub-seed mldsa65=$$($(1)_KEY_MLDSA_SEED) \
		--out $$@
	@$$(CAPSULE_SIGN_BIN) verify-manifest \
		--manifest $$($(1)_MANIFEST) \
		--cert $$($(1)_CERT) \
		--policy $$(NONOS_TRUST_ANCHOR_POLICY_BIN) >/dev/null

endif

# Capsule attestation trailer. The whole capsule set is enrolled at once by the
# transparent STARK enrollment ($(ZK_CAPSULE_ROOT) rule in the top Makefile),
# which writes the policy root and every trailer together so the root commits to
# the real capsule measurements. Each trailer therefore depends only on that
# step; building any capsule's artifacts triggers the single enrollment.
$$($(1)_ATTESTATION): $$(ZK_CAPSULE_ROOT)
	@test -f $$@ || { echo "trailer $$@ was not produced by STARK enrollment"; exit 1; }


nonos-mk-$(1)-sign: $$($(1)_CERT) $$($(1)_MANIFEST) $$($(1)_ATTESTATION)

nonos-mk-$(1)-verify: $$($(1)_ARTIFACTS) $$(NONOS_TRUST_ANCHOR_POLICY_BIN) $$(CAPSULE_SIGN_BIN)
	@echo "Verifying $$($(1)_HANDLE) capsule manifest..."
	@$$(CAPSULE_SIGN_BIN) verify-manifest \
		--manifest $$($(1)_MANIFEST) \
		--cert $$($(1)_CERT) \
		--policy $$(NONOS_TRUST_ANCHOR_POLICY_BIN) >/dev/null

endef

$(eval $(call NONOS_CAPSULE_RULES,$(CAPSULE_SLUG)))

# Reset per-capsule input variables, so nothing one Capsule.mk set leaks into
# the next. The slug-prefixed snapshot above is permanent.
undefine CAPSULE_SLUG
undefine CAPSULE_BIN_NAME
undefine CAPSULE_DIR
undefine CAPSULE_HANDLE
undefine CAPSULE_DOMAIN
undefine CAPSULE_RECOVERY
undefine CAPSULE_NAMESPACE
undefine CAPSULE_SERVICE_ENDPOINT
undefine CAPSULE_REPLY_ENDPOINT
undefine CAPSULE_INSTANCE_ENDPOINTS
undefine CAPSULE_REQUIRED_CAPS
undefine CAPSULE_OPTIONAL_CAPS
undefine CAPSULE_CAPS_CEILING
undefine CAPSULE_TARGET
undefine CAPSULE_VERSION
undefine CAPSULE_ID_CERT_SERIAL
undefine CAPSULE_BUILD_STD
undefine CAPSULE_BUILD_STD_FEATURES
undefine CAPSULE_KEY_PUB_PREFIX
undefine CAPSULE_KEY_SEED_PREFIX
undefine CAPSULE_METADATA
undefine CAPSULE_FEATURE
undefine CAPSULE_CARGO_FEATURES
undefine CAPSULE_PREBUILT_BIN
undefine CAPSULE_KERNEL_MIRROR
undefine CAPSULE_DEV_ONLY
undefine CAPSULE_NOT_IN_IMAGE
undefine CAPSULE_EXTRA_DEPS
undefine CAPSULE_EXTRA_ORDER_DEPS
undefine _NONOS_CAPSULE_OPTIONAL_CAPS
undefine _NONOS_CAPSULE_CAPS_CEILING
undefine _NONOS_CAPSULE_TARGET
undefine _NONOS_CAPSULE_VERSION
undefine _NONOS_CAPSULE_SERIAL
undefine _NONOS_CAPSULE_BUILD_STD
undefine _NONOS_CAPSULE_BUILD_STD_FEAT
undefine _NONOS_CAPSULE_KEY_PUB_PREFIX
undefine _NONOS_CAPSULE_KEY_SEED_PREFIX
undefine _NONOS_CAPSULE_METADATA
undefine _NONOS_CAPSULE_FEATURE
undefine _NONOS_CAPSULE_CARGO_FEATURES
undefine _NONOS_CAPSULE_PREBUILT_BIN
undefine CAPSULE_MK_FILE
