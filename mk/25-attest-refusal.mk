# The booted spawn gate against broken capsules: a test profile, never a
# production build (lib.rs refuses it beside nonos-production). Every variant
# starts from an honest enrollment and is changed afterwards, and the enroll
# tool checks each one is refused before it writes it.

.PHONY: nonos-mk-attest-refusal-variants nonos-mk-attest-refusal-test nonos-mk-attest-refusal-run

ATTEST_REFUSAL_DIR := $(TARGET_DIR)/attest-refusal
ATTEST_REFUSAL_LOG := $(TARGET_DIR)/attest-refusal/serial.log
ATTEST_REFUSAL_CERT := $(ATTEST_REFUSAL_DIR)/extra_cap.nonos_id_cert.bin
ATTEST_REFUSAL_MANIFEST := $(ATTEST_REFUSAL_DIR)/extra_cap.manifest.bin
# proof_io enrolled with 0x19; the probe identity grants one bit more (IO).
ATTEST_REFUSAL_CAPS := 0x1b

# One run writes all three, so they hang off one stamp the other rules can
# name.
ATTEST_REFUSAL_STAMP := $(ATTEST_REFUSAL_DIR)/.variants
$(ATTEST_REFUSAL_STAMP): $(proof-io_ATTESTATION) $(ZK_CAPSULE_ROOT) $(NONOS_STARK_ENROLL)
	@mkdir -p $(ATTEST_REFUSAL_DIR)
	@echo "Writing the refusal variants from proof_io's honest enrollment..."
	@$(NONOS_STARK_ENROLL) refusal-variants $(ZK_CAPSULE_ROOT) \
		$(proof-io_REQUIRED_CAPS):$(proof-io_BIN):$(proof-io_ATTESTATION) $(ATTEST_REFUSAL_DIR)
	@touch $@

$(ATTEST_REFUSAL_CERT): $(NONOS_TRUST_ANCHOR_POLICY_BIN) $(proof-io_KEY_ED_PUB) $(proof-io_KEY_MLDSA_PUB) \
		$(CAPSULE_SIGN_BIN) | nonos-mk-check-trust-keys nonos-mk-check-proof-io-keys
	@mkdir -p $(ATTEST_REFUSAL_DIR)
	@$(CAPSULE_SIGN_BIN) sign-id-cert --serial 9001 --nonos-id $(proof-io_NONOS_ID_HEX) \
		--ns-glob $(proof-io_NAMESPACE) --caps-ceiling $(ATTEST_REFUSAL_CAPS) \
		--epoch $(NONOS_TRUST_ANCHOR_EPOCH) \
		--valid-from-ms $(NONOS_CERT_VALID_FROM_MS) --valid-until-ms $(NONOS_CERT_VALID_UNTIL_MS) \
		--pub-key ed25519=$(proof-io_KEY_ED_PUB) --pub-key mldsa65=$(proof-io_KEY_MLDSA_PUB) \
		--ta-seed ed25519=$(NONOS_TA_ED25519_SEED) --ta-seed mldsa65=$(NONOS_TA_MLDSA65_SEED) \
		--metadata "attest refusal probe" --out $@

$(ATTEST_REFUSAL_MANIFEST): $(ATTEST_REFUSAL_CERT) $(proof-io_BIN) $(CAPSULE_SIGN_BIN)
	@$(CAPSULE_SIGN_BIN) sign-manifest --cert $(ATTEST_REFUSAL_CERT) \
		--namespace $(proof-io_NAMESPACE) --version $(proof-io_VERSION) \
		--target $(proof-io_TARGET) --elf $(proof-io_BIN) \
		--required-caps $(ATTEST_REFUSAL_CAPS) --optional-caps $(proof-io_OPTIONAL_CAPS) \
		--endpoint $(proof-io_SERVICE_ENDPOINT) --endpoint $(proof-io_REPLY_ENDPOINT) \
		--pub-seed ed25519=$(proof-io_KEY_ED_SEED) --pub-seed mldsa65=$(proof-io_KEY_MLDSA_SEED) \
		--out $@
	@$(CAPSULE_SIGN_BIN) verify-manifest --manifest $@ --cert $(ATTEST_REFUSAL_CERT) \
		--policy $(NONOS_TRUST_ANCHOR_POLICY_BIN) >/dev/null

nonos-mk-attest-refusal-variants: $(ATTEST_REFUSAL_STAMP) $(ATTEST_REFUSAL_MANIFEST)

nonos-mk-attest-refusal-test: $(proof-io_ARTIFACTS) nonos-mk-attest-refusal-variants \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,nonos-attest-refusal-smoketest,nonos-attest-refusal-smoketest)

# Boot the probe kernel headless, then read the log: one admission and four
# refusals, each from the gate itself, or the run fails.
nonos-mk-attest-refusal-run: $(QEMU_BLK_IMG) $(QEMU_BLK_STORE_STAMP)
	$(call nonos_kernel_and_esp,nonos-mk-attest-refusal-test)
	@mkdir -p $(dir $(ATTEST_REFUSAL_LOG))
	@-timeout 240 $(QEMU) -m $(QEMU_MEM) $(QEMU_ACCEL_ARGS) -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_RNG) -serial "file:$(ATTEST_REFUSAL_LOG)" -display none -no-reboot
	@$(NONOS_PYTHON) nonos-ci/attest_refusal_check.py --log $(ATTEST_REFUSAL_LOG)
