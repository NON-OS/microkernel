# Signing and publisher keys

Every [capsule](../overview/glossary.md#capsule) is signed with two hybrid key pairs and enrolled once with the rest of the set, and the kernel refuses any capsule whose certificate, manifest, signatures or proof do not check out.

## The keys

Two kinds of key sign a capsule, and both come as a hybrid pair: one Ed25519 key and one ML-DSA-65 key.

- The [trust anchor](../overview/glossary.md#trust-anchor) signs every [NONOS ID certificate](../overview/glossary.md#nonos-id-certificate). Its public halves are `nonos_trust_anchor_ed25519.pub` and `nonos_trust_anchor_mldsa65.pub` under `nonos-data/trust/keys/` (`tools/nonos_seal/keys.py:32`, `TA_PUBS`). The trust anchor policy built from them is baked into the kernel (`mk/20-build.mk:200-203`, `BAKED_TRUST_ANCHOR_POLICY`).
- A [publisher](../overview/glossary.md#publisher) key pair signs a capsule's [manifest](../overview/glossary.md#manifest). Each capsule has its own pair, named after its binary: `<bin>_publisher_ed25519.pub` and `<bin>_publisher_mldsa65.pub` in the same directory (`nonos-mk/capsule.mk:85`, `CAPSULE_KEY_PUB_PREFIX`). The 19 Linux userland programs share one publisher, `linux_userland_publisher` (`userland/linux_userland/Userland.mk:119-120`, `CAPSULE_KEY_PUB_PREFIX`).

The `.pub` files, certificates, manifests, trailers and the policy are committed under `nonos-data/trust/`. The private halves are seeds kept in `.keys/`, a directory `.gitignore` keeps out of git (`nonos-mk/capsule.mk:56-63`, `NONOS_BAKED_TRUST_DIR`). Key files are self-tagged binary blobs, `NONOSSK1` for a seed and `NONOSPK1` for a public key (`nonos-sign/src/cli/usage.rs:35-37`, `keygen`).

To see which publisher pairs a build needs, list the key prefix of every capsule the make files include:

```sh
python3 tools/nonos-capsule-key-prefixes
```

On this tree it prints 97 names on one line, from `proof_io` and `std_proof` to `power`. The script reads only the `include` lines of the make files, so the shared Linux userland publisher is not among them. Its own description says the key ceremony, `tools/nonos-key-ceremony`, makes one publisher pair per name (`tools/nonos-capsule-key-prefixes:17-19`, `CAPSULE_BIN_NAME`).

## capsule-sign

The host tool is the binary `capsule-sign`, built in `nonos-sign` around the `nonos_capsule_sign` library (`nonos-sign/Cargo.toml:8-14`). Its subcommands are `keygen`, `derive-id`, `mk-trust-policy`, `sign-id-cert`, `sign-manifest`, `sign-release`, `verify-release`, `verify-policy`, `verify-cert` and `verify-manifest` (`nonos-sign/src/cli/dispatch.rs:29-45`, `dispatch`). The `nonos-sign` host tests, 21 of them, passed in the flake checks on this commit.
