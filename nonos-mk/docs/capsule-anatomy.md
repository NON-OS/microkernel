# Anatomy of a Capsule.mk

A capsule declares itself in a dozen variables and inherits everything
else. The macro turns that declaration into the capsule's build, sign,
and verify machinery, so two capsules can only differ where their
declarations differ.

## Required declarations

| Variable | Meaning |
|----------|---------|
| `CAPSULE_SLUG` | the make-facing name; every generated target embeds it |
| `CAPSULE_HANDLE` | the service handle, `app.foo` or `driver.bar0` |
| `CAPSULE_DOMAIN` | the publisher domain the NONOS-ID derives from |
| `CAPSULE_DIR` | the crate directory under `userland/` |
| `CAPSULE_BIN_NAME` | the cargo binary name; artifact and key names derive from it |
| `CAPSULE_NAMESPACE` | the manifest namespace, checked against the certificate's globs |
| `CAPSULE_SERVICE_ENDPOINT` | `service:<port>:<name>`, declared in the signed manifest |
| `CAPSULE_REPLY_ENDPOINT` | `reply:<port>:<name>`, likewise |
| `CAPSULE_REQUIRED_CAPS` | the capability mask the kernel grants at spawn |
| `CAPSULE_KERNEL_MIRROR` | where the kernel-side embed and spawn wiring lives |

## Optional declarations and their defaults

| Variable | Default |
|----------|---------|
| `CAPSULE_OPTIONAL_CAPS` | `0x0` |
| `CAPSULE_CAPS_CEILING` | the required caps; a cert may allow more only deliberately |
| `CAPSULE_TARGET` | `x86_64-nonos-user` |
| `CAPSULE_VERSION` | `0.1.0` |
| `CAPSULE_ID_CERT_SERIAL` | `1` |
| `CAPSULE_BUILD_STD` | `core,alloc`; std-bearing capsules override |
| `CAPSULE_KEY_PUB_PREFIX` | `<keystore>/keys/<bin>_publisher` |
| `CAPSULE_KEY_SEED_PREFIX` | `.keys/<bin>_publisher` |
| `CAPSULE_FEATURE` | `nonos-capsule-<slug>`, the kernel embed feature |
| `CAPSULE_PREBUILT_BIN` | unset; when set, the binary is installed, not compiled |
| `CAPSULE_EXTRA_DEPS`, `CAPSULE_EXTRA_ORDER_DEPS` | extra prerequisites for the build |
| `CAPSULE_INSTANCE_ENDPOINTS` | extra windowed instances, each in the signed manifest |

The seed and public key prefixes split on purpose: public keys are
committed to the keystore, seeds live in a gitignored directory, and
nothing in the macro ever writes a seed anywhere else.

## What the macro materializes

For slug `foo` the include produces `nonos-mk-foo` (build the ELF),
`nonos-mk-foo-sign` (certificate and manifest under the owner's
seeds), `nonos-mk-foo-verify` (the chain checked back), and
`nonos-mk-check-foo-keys` (assert the publisher material exists before
anything signs). The artifact paths follow the bin name into the
keystore: `<bin>.nonos_id_cert.bin`, `<bin>.manifest.bin`,
`<bin>.zk_trailer.bin`.

The trailer has no per-capsule rule body: every trailer depends on the
one set-level STARK enrollment, so the policy root always commits to
the whole set at once and a capsule cannot be enrolled against a root
that never saw it.

## The build rule

Compiled capsules build with cargo against the declared target with
`-Z build-std` for the declared core set; the rule's prerequisites are
the crate sources, its lockfile, the shared libc, and the declared
extras, so a source change rebuilds and an artifact change re-signs,
while a cached binary can never satisfy a changed source. Prebuilt
capsules, the crates.io tools, skip compilation and install the
declared binary with a fresh mtime through the same target name, so
downstream rules cannot tell the difference and do not need to.
