#!/usr/bin/env bash
# Scratch trust-chain bootstrap for CI lanes that need to build a
# signed kernel image but do not hold the upstream trust-anchor seed.
#
# Generates a fresh in-tree Ed25519 + ML-DSA-65 trust anchor, fresh
# per-capsule publisher keypairs, wipes the committed scratch outputs
# (policy, certs, manifests), and re-signs every capsule listed in
# CAPSULE_SLUGS through the production Makefile recipes. The kernel
# image these certs go into is signature-shaped but does NOT chain to
# the upstream trust anchor; the production ledger never trusts it.
#
# Inputs (env, all optional):
#   CAPSULE_SLUGS           dash-form slug list, default from Makefile includes
#   CAPSULE_KEY_PREFIXES    publisher key prefixes, default from Capsule.mk names
#
# Idempotent: calling twice produces a different scratch chain. Safe
# only inside an ephemeral CI workspace.

set -euo pipefail

if [ -z "${CAPSULE_SLUGS:-}" ] || [ -z "${CAPSULE_KEY_PREFIXES:-}" ]; then
    capsule_inventory="$(mktemp)"
    # The capsule includes live in the top-level Makefile or, since the
    # modular split, in mk/*.mk; scan both so the inventory survives either
    # layout. An empty inventory would mean zero publisher keys and every
    # capsule key check downstream failing, so treat it as fatal here.
    cat Makefile mk/*.mk 2>/dev/null \
        | awk '/^include userland\/.*\/Capsule\.mk$/ { print $2 }' > "${capsule_inventory}"
    if ! [ -s "${capsule_inventory}" ]; then
        echo "::error::no capsule includes found in Makefile or mk/*.mk"
        exit 1
    fi
    derived_slugs=""
    derived_prefixes=""
    while IFS= read -r capsule_mk; do
        [ -f "${capsule_mk}" ] || { echo "::error::missing ${capsule_mk}"; exit 1; }
        slug="$(awk -F ':=' '$1 ~ /^[[:space:]]*CAPSULE_SLUG[[:space:]]*$/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "${capsule_mk}")"
        prefix="$(awk -F ':=' '$1 ~ /^[[:space:]]*CAPSULE_BIN_NAME[[:space:]]*$/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "${capsule_mk}")"
        [ -n "${slug}" ] || { echo "::error::missing CAPSULE_SLUG in ${capsule_mk}"; exit 1; }
        [ -n "${prefix}" ] || { echo "::error::missing CAPSULE_BIN_NAME in ${capsule_mk}"; exit 1; }
        # A development test capsule is on NONOS_DEV_CAPSULES, which no make
        # lane signs or enrolls (nonos-mk/capsule.mk); only the seal of a
        # development image takes it.
        if awk -F ':=' '$1 ~ /^[[:space:]]*CAPSULE_DEV_ONLY[[:space:]]*$/ { gsub(/[[:space:]]/, "", $2); if ($2 != "") found = 1 } END { exit !found }' "${capsule_mk}"; then
            continue
        fi
        derived_slugs="${derived_slugs} ${slug}"
        derived_prefixes="${derived_prefixes} ${prefix}"
    done < "${capsule_inventory}"
    rm -f "${capsule_inventory}"
    CAPSULE_SLUGS="${CAPSULE_SLUGS:-${derived_slugs}}"
    CAPSULE_KEY_PREFIXES="${CAPSULE_KEY_PREFIXES:-${derived_prefixes}}"
fi

CAPSULE_SLUGS="$(printf '%s\n' "${CAPSULE_SLUGS}" | xargs)"
CAPSULE_KEY_PREFIXES="$(printf '%s\n' "${CAPSULE_KEY_PREFIXES}" | xargs)"

mkdir -p .keys
mkdir -p nonos-data/trust/keys
mkdir -p nonos-data/trust/policy
mkdir -p nonos-data/trust/capsules

# The flake's host tools when the caller has them (NONOS_HOST_TOOLS, as
# `nix build .#host-tools` gives), else a cargo build of capsule-sign.
if [ -n "${NONOS_HOST_TOOLS:-}" ]; then
    CS="${NONOS_HOST_TOOLS}/bin/capsule-sign"
else
    echo "[scratch-trust-bootstrap] building capsule-sign host tool"
    ( cd nonos-sign && cargo build --release --bin capsule-sign )
    CS="$(pwd)/nonos-sign/target/release/capsule-sign"
fi
[ -x "${CS}" ] || { echo "::error::capsule-sign not built at ${CS}"; exit 1; }

echo "[scratch-trust-bootstrap] generating scratch trust-anchor keypair"
"${CS}" keygen --alg ed25519 --out .keys/nonos_trust_anchor_ed25519
"${CS}" keygen --alg mldsa65 --out .keys/nonos_trust_anchor_mldsa65
chmod 600 .keys/nonos_trust_anchor_ed25519.seed \
          .keys/nonos_trust_anchor_mldsa65.seed
mv .keys/nonos_trust_anchor_ed25519.pub nonos-data/trust/keys/
mv .keys/nonos_trust_anchor_mldsa65.pub nonos-data/trust/keys/

# The device policy key signs the boot-root record the kernel checks the loader
# against, and kernel.approval. The kernel compiles in its public point, x || y,
# so a scratch image carries a scratch key end to end, made as the ceremony
# makes the real one.
echo "[scratch-trust-bootstrap] generating scratch device policy key (P-256)"
openssl ecparam -name prime256v1 -genkey -noout -out .keys/device_policy_p256.pem
chmod 600 .keys/device_policy_p256.pem
openssl ec -in .keys/device_policy_p256.pem -pubout -outform DER 2>/dev/null | tail -c 64 \
    > nonos-data/trust/policy/device_policy_p256.pub

echo "[scratch-trust-bootstrap] generating publisher keypairs: ${CAPSULE_KEY_PREFIXES}"
for prefix in ${CAPSULE_KEY_PREFIXES}; do
    "${CS}" keygen --alg ed25519 --out ".keys/${prefix}_publisher_ed25519"
    "${CS}" keygen --alg mldsa65 --out ".keys/${prefix}_publisher_mldsa65"
    chmod 600 ".keys/${prefix}_publisher_ed25519.seed" \
              ".keys/${prefix}_publisher_mldsa65.seed"
    mv ".keys/${prefix}_publisher_ed25519.pub" nonos-data/trust/keys/
    mv ".keys/${prefix}_publisher_mldsa65.pub" nonos-data/trust/keys/
done

# Every publisher the catalogue names, the Linux userland's included,
# which the Capsule.mk scan above does not see: both modes sign with them.
python3 -c 'import json; [print(e[f"seed_{a}"][:-5], e[f"pub_{a}"]) for e in json.load(open("tools/nix/capsules.json")) for a in ("ed25519", "mldsa65")]' \
    | sort -u | while read -r seed pub; do
    [ -f "${seed}.seed" ] && continue
    alg="${seed##*_}"
    "${CS}" keygen --alg "${alg}" --out "${seed}"
    chmod 600 "${seed}.seed"
    mv "${seed}.pub" "${pub}"
done

# The marketplace operator signs the Marketplace index and the Qwen model
# catalogue, and capsule_model_fetch compiles its public key in from
# .keys/marketplace_operator_ed25519.pub. A sealed image for use refuses to
# go without one (tools/nonos_seal/market.py), so a scratch image gets a
# scratch operator, written over the committed public key on this runner
# only. A workflow that seals a release profile commits that key locally
# first, since the seal seals only a committed tree.
if [ ! -f .keys/marketplace_operator_ed25519.seed ]; then
    echo "[scratch-trust-bootstrap] generating scratch marketplace operator key"
    "${CS}" keygen --alg ed25519 --out .keys/marketplace_operator_ed25519
    # capsule-sign writes the key in an 11-byte NONOSSK1/NONOSPK1 container,
    # but the marketplace operator key is read as a raw 32-byte seed and
    # public key: the market-index CLI (nonos-mk keys::from_seed_bytes) and
    # the model-catalogue tool (tools/nonos_qwen_tier) both require 32 bytes,
    # and the committed .pub is stored that way. Take the raw 32-byte tail of
    # each; without this the catalogue and index signing steps fail with
    # "the operator seed does not belong to the operator public key".
    for _ext in seed pub; do
        _f=".keys/marketplace_operator_ed25519.${_ext}"
        tail -c 32 "${_f}" > "${_f}.raw" && mv "${_f}.raw" "${_f}"
    done
    chmod 600 .keys/marketplace_operator_ed25519.seed
fi

# NONOS_SCRATCH_KEYS_ONLY=1 stops at the keys, with the kernel's ML-DSA-65 key
# beside the Ed25519 seed setup-signing-key.sh wrote: `nix run .#seal` signs
# and enrolls from there, against the flake's artifacts.
if [ "${NONOS_SCRATCH_KEYS_ONLY:-0}" = "1" ]; then
    mkdir -p nonos-bootloader/keys
    [ -f nonos-bootloader/keys/kernel_mldsa65.seed ] || \
        "${CS}" keygen --alg mldsa65 --out nonos-bootloader/keys/kernel_mldsa65
    chmod 600 nonos-bootloader/keys/kernel_mldsa65.seed
    echo "[scratch-trust-bootstrap] scratch keys ready"
    exit 0
fi

echo "[scratch-trust-bootstrap] wiping stale committed policy + certs + manifests"
# Only the trust-anchor policy is key-dependent and gets re-sealed with the
# scratch keys by the kernel build. The zk capsule policy root is the Merkle
# root over capsule hashes and capability masks, independent of signing keys,
# and the kernel embeds it at compile time; keep the committed one.
rm -f nonos-data/trust/policy/nonos_trust_anchor.policy.bin
# ek's boot-root record names the sealed bootloader and is signed by the
# committed device policy key, which this runner replaced with a scratch one;
# without it the build signs a record for its own loader (nonos-mk-boot-root-record).
rm -f nonos-data/trust/policy/boot_root.approval
rm -f nonos-data/trust/capsules/*.nonos_id_cert.bin
rm -f nonos-data/trust/capsules/*.manifest.bin

echo "[scratch-trust-bootstrap] building userland libc"
make nonos-mk-libc

echo "[scratch-trust-bootstrap] re-signing capsules: ${CAPSULE_SLUGS}"
for slug in ${CAPSULE_SLUGS}; do
    make "nonos-mk-${slug}-sign"
done

echo "[scratch-trust-bootstrap] done"
