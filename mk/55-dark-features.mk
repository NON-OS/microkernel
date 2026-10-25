# check-dark-features: the switches no profile, image or boot lane may turn on,
# each type-checked once so their code still compiles. Compile only: no nix,
# no image, no boot, a debug check with no signing key in the environment, so
# nothing it produces could be signed or booted. It is the one lane
# scripts/check_proof_features.py allows to name the boot proofs, and the
# lane scripts/check_dark_features.py counts them as lit by. The Makefile reads
# it outside the flake shell too, so it is guarded against a second include;
# rust-toolchain.toml picks the toolchain.

ifndef NONOS_DARK_FEATURES_MK
NONOS_DARK_FEATURES_MK := 1
.PHONY: check-dark-features

DARK_CHECK = env -u NONOS_SIGNING_KEY cargo check --locked --target x86_64-nonos.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

check-dark-features:
	$(DARK_CHECK) --features nonos-smp,nonos-shootdown-trace
	$(DARK_CHECK) --features nonos-smp,nonos-smp-park-proof
	$(DARK_CHECK) --features nonos-smp,nonos-smp-stuck-proof
	$(DARK_CHECK) --features nonos-smp,nonos-smp-pge-proof
	$(DARK_CHECK) --features nonos-panic-proof
	$(DARK_CHECK) --features nonos-fault-proof
	$(DARK_CHECK) --features pqc-unvetted
endif
