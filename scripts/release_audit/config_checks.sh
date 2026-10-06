#!/bin/sh
set -eu

# The trust posture a release depends on, read from the tree. Every STARK gate
# is enforced in every build, so these check that no escape came back.

fail=0
pass() { printf 'PASS: %s\n' "$1"; }
flunk() { fail=1; printf 'FAIL: %s\n' "$1"; }

removed='nonos-zk-rollout nonos-dev-unverified-capsules'
for f in $removed; do
  if grep -rn --include='*.rs' "feature *= *\"$f\"" src nonos-bootloader/src >/dev/null 2>&1; then
    flunk "a cfg site names the removed feature $f"
  else
    pass "no cfg site names the removed feature $f"
  fi
  if sed -n '/^default = /p' Cargo.toml | grep -F "$f" >/dev/null 2>&1; then
    flunk "default features include $f"
  else
    pass "default features do not include $f"
  fi
done

if [ -e src/kernel_core/process_spawn/capsule_spawn/runner/legacy.rs ]; then
  flunk "the unverified legacy spawn path is back (runner/legacy.rs)"
else
  pass "the verified spawn path is the only one"
fi

if grep -F 'feature = "nonos-production", feature = "nonos-attest-refusal-smoketest"' src/lib.rs >/dev/null 2>&1; then
  pass "production refuses the refusal smoketest's broken capsules at compile time"
else
  flunk "missing compile_error! for nonos-production beside nonos-attest-refusal-smoketest"
fi

if grep -E 'SecurityMode|Development' nonos-bootloader/src/boot/attestation/kernel_gate.rs >/dev/null 2>&1; then
  flunk "the loader's kernel STARK gate reads a security mode again"
else
  pass "the loader's kernel STARK gate is the same in every mode"
fi

exit "$fail"
