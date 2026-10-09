# The attestation surface page

The browser page that verifies a whole NONOS release: the kernel and every
enrolled capsule, each proof against the roots the boot chain enforces, the
policy tree refolded in-wasm from the complete leaf set, and the boot chain
closed from the shipping artifacts themselves via the drop zone.

Only the page's code lives here. The fixtures are release artifacts,
generated per release by `gen-os-fixtures` from the real binaries, trailers,
and roots, verified before they are written, and deployed beside this page:

    gen-os-fixtures <out> --policy-root <bin> --epoch <n> \
        --kernel-elf <bin> --kernel-root <bin> --kernel-trailer <bin> \
        --capsules <tsv>

`smoke.mjs` mirrors the page's exact verification logic headless and
`chain.mjs` proves the boot chain flow; both run against a generated fixture
directory and gate CI.
