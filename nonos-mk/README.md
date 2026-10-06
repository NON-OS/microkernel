# nonos-mk

The shared build machinery for NONOS capsules. The kernel repository's
top-level Makefile includes one `Capsule.mk` per capsule, and each of
those includes `capsule.mk` from here, which materializes the standard
target set for that capsule: build, sign, verify, and key checks.

## What a capsule declares

A `Capsule.mk` is a dozen variables: slug, service handle, namespace,
endpoints, the required capability mask, and the directory. Everything
else, the cargo invocation, the certificate and manifest rules, the
attestation trailer dependency, the per-capsule verify target, comes
from the shared macro so all capsules are built and proven the same
way. A capsule cannot opt out of verification by writing its own rules.

## Two trust modes

The default mode is the signing flow: certificates and manifests are
produced under the owner's seeds and verified immediately after.

Under `NONOS_TRUST_REUSE=1` nothing signs. The committed artifacts are
asserted to exist, verified under the baked policy, and every freshly
built ELF must measure to its enrolled payload hash via
`capsule-sign verify-manifest --elf`. This is the production pipeline
mode: no rule demands a seed, and a capsule whose rebuild drifts from
the enrolled measurement fails by name. The cure for that failure is a
reproducible build, never a minted key.

## License

AGPL-3.0, like the rest of NONOS.
