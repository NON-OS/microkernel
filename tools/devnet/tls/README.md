# The local test chain's TLS root (P5)

A development image's settle capsule (a `devnet` build) dials
`rpc.devnet.nonos.test` directly at 10.0.2.2, QEMU's host, over TLS. These
files let a TLS proxy on the host answer it in front of a local node.

| File | Holds |
|------|-------|
| `root.der` | the self-signed P-256 root, CN `NONOS-devnet-throwaway-root` |
| `root_subject.der`, `root_spki.der` | its SubjectDN and public key, as nonos_tls compiles them in |
| `leaf.der` | the one leaf it signs, for `rpc.devnet.nonos.test` |
| `root-key.throwaway.der`, `leaf-key.throwaway.der` | both private keys, PKCS#8 DER |

## Public on purpose

Both keys are throwaway and committed so that anyone can stand up the
proxy. They are derived from fixed labels in `generate.py`, so anyone can
also derive them; they protect nothing. A release never trusts this root:
nonos_tls compiles it in only with its `devnet` feature, which only the
development capsule set turns on (`scripts/check_devnet_feature.py`), and
the release-names check (`tools/nix/release_names.nix`) fails if a release
capsule holds the root's name or its key's hash.

Never reuse these keys for anything else, and never add this root to a
trust store outside a development image.

## Regenerating

`python3 tools/devnet/tls/generate.py` writes the same bytes on every run;
`--check` says whether the committed files are still those. Changing the
root means changing `userland/nonos_tls/src/roots/dev.rs` with it; its test
in tls_proofs fails until the hash there matches `root_spki.der`.

## The proxy

Most proxies take PEM:

    openssl x509 -inform DER -in leaf.der > /tmp/devnet-chain.pem
    openssl x509 -inform DER -in root.der >> /tmp/devnet-chain.pem
    openssl pkey -inform DER -in leaf-key.throwaway.der > /tmp/devnet-leaf-key.pem

Then serve TLS 1.3 on the host's port 443 with that chain and key, and
forward the plain stream to the node's RPC port.
