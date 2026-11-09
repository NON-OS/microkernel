# check-dark-features: the switches no profile, image or boot lane may turn on,
# each type-checked once so their code still compiles. Compile only: no nix,
# no image, no boot, a debug check with no signing key in the environment, so
# nothing it produces could be signed or booted. It is the one lane
# scripts/check_proof_features.py allows to name the boot proofs, and the
# lane scripts/check_dark_features.py counts them as lit by. The Makefile reads
# it outside the flake shell too, so it is guarded against a second include;
# rust-toolchain.toml picks the toolchain.

ifndef NONOS_NSH_RELEASE_MK
NONOS_NSH_RELEASE_MK := 1
.PHONY: check-nsh-release-refused

# check-dark-features runs it too (mk/55-dark-features.mk keeps to compile-only
# DARK_CHECK lines, so this one lives beside it).
check-dark-features: check-nsh-release-refused

# A production or release kernel refuses each nsh capsule at compile time
# unless it is the server profile's (nonos-server), and nsh.front refuses to
# build without nsh.attest (src/lib.rs): each refused check must fail, and
# fail on that refusal, not on something else first. A server kernel, and a
# development twin (nonos-production beside nonos-dev-attest) with both
# capsules, must not hit the refusal.
NSH_RELEASE_REFUSAL := ship in the server profile alone
NSH_PAIR_REFUSAL := so it needs nonos-capsule-nsh-attest
NSH_PAIR := nonos-capsule-nsh-front,nonos-capsule-nsh-attest
check-nsh-release-refused:
	@for p in nonos-production nonos-release; do for f in nonos-capsule-nsh-front nonos-capsule-nsh-attest $(NSH_PAIR); do \
	  if $(DARK_CHECK) --no-default-features --features microkernel-core,$$p,$$f > nsh-release.log 2>&1; then \
	    echo "check-nsh-release-refused: a $$p kernel compiled with $$f"; rm -f nsh-release.log; exit 1; \
	  fi; \
	  if ! grep -q "$(NSH_RELEASE_REFUSAL)" nsh-release.log; then \
	    echo "check-nsh-release-refused: $$f failed beside $$p, but not on the refusal"; rm -f nsh-release.log; exit 1; \
	  fi; \
	  echo "check-nsh-release-refused: $$f refused in a $$p kernel"; \
	done; \
	  $(DARK_CHECK) --no-default-features --features microkernel-core,$$p,nonos-server > nsh-release.log 2>&1; \
	  if grep -q "$(NSH_RELEASE_REFUSAL)" nsh-release.log; then \
	    echo "check-nsh-release-refused: a $$p server kernel refused the nsh capsules"; rm -f nsh-release.log; exit 1; \
	  fi; \
	  echo "check-nsh-release-refused: the nsh capsules allowed in a $$p server kernel"; \
	done; \
	$(DARK_CHECK) --no-default-features --features microkernel-core,nonos-production,nonos-dev-attest,$(NSH_PAIR) > nsh-release.log 2>&1; \
	if grep -q "$(NSH_RELEASE_REFUSAL)" nsh-release.log; then \
	  echo "check-nsh-release-refused: a development twin refused the nsh capsules"; rm -f nsh-release.log; exit 1; \
	fi; \
	echo "check-nsh-release-refused: the nsh capsules allowed in a development twin"; \
	if $(DARK_CHECK) --no-default-features --features microkernel-core,nonos-production,nonos-dev-attest,nonos-capsule-nsh-front > nsh-release.log 2>&1 \
	  || ! grep -q "$(NSH_PAIR_REFUSAL)" nsh-release.log; then \
	  echo "check-nsh-release-refused: nsh.front built without nsh.attest"; rm -f nsh-release.log; exit 1; \
	fi; \
	echo "check-nsh-release-refused: nsh.front refused without nsh.attest"; \
	rm -f nsh-release.log

endif
