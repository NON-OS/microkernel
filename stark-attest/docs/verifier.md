# The gate in a browser tab

`verifier-wasm/` compiles the deployment's verification entry point,
`verify_attestation_trailer`, to WebAssembly, and `web/` is a single
dependency-free page that runs it against a real enrolled kernel's trailer.
This page documents what that is, what it proves, how to rebuild it, and the
two design decisions that are easy to second-guess without their reasons.

## What the visitor actually runs

The wasm is not a demo verifier and not a reimplementation. It is the same
function the NONOS kernel calls at every process spawn and the bootloader
calls before jumping to the kernel, built from the same crate with the same
parameters from `attest_params`, at 94 KB. The page fetches four same-origin
files: the wasm, the 32-byte root, the trailer, and the context, then calls
the gate in the visitor's own tab. Nothing is transmitted anywhere; there is
no server-side component and no external request of any kind.

The fixture is real. `gen-web-fixture` enrolls actual artifacts through the
production enrolment path; the shipped fixture's member zero is an enrolled
NONOS kernel image, and the context binds that image's BLAKE3 measurement
with a caller byte, exactly as the OS binds a capsule's capability mask.

The tamper button flips the last bit of the trailer and verifies again. This
is not theatre: it demonstrates the property the whole construction stands
on, that the proof is a rigid object and the gate's rejection path costs the
same as its acceptance path.

## What a green verdict proves, and what it does not

It proves: this trailer is a valid STARK membership proof, under this root,
bound to this context, at the compiled soundness parameters. Equivalently,
whoever produced it knew an authentication path from an enrolled measurement
to the root, and ground the transcript for this exact context.

It does not prove that the kernel you downloaded is member zero. That link is
the measurement in the context: check the displayed BLAKE3 hash against your
own `b3sum` of the release image. The page states this on its face, because a
verification page that overstates what it verifies teaches visitors the wrong
habit.

## The interface, and why it is primitive

Three exports: `wasm_alloc`, `wasm_free`, `verify`. Raw pointers and linear
memory, no binding framework, glue small enough to read in one sitting. The
choice is deliberate: this page's trust story is "you can audit everything
between your click and the verdict", and every dependency shipped to the tab
is audit surface. The JS glue is the only code beside the wasm, and the wasm
is reproducible from the crate:

```
cargo build --release --target wasm32-unknown-unknown -p stark-attest-verifier-wasm
```

## The build that caught a bug

Compiling the gate for the browser was the first time anything outside the
crate called `verify_attestation_trailer` against a builder-produced trailer,
and it failed: the canonical entry point read the wire regions in the wrong
order and routed into a retired proof pipeline. Every deployed consumer had
its own parse, so nothing had ever walked through the crate's own front door.
The full account is in the changelog. It is recorded here because it is this
page's best credential: the browser verifier is not a marketing artifact
bolted onto the system, it is a second consumer of the real gate, and second
consumers find what single consumers cannot.

## Serving it

The five files must be served same-origin, the wasm with
`Content-Type: application/wasm`, and no asset transformation (minification,
script injection) between origin and visitor, since the page's value is that
its bytes are checkable against this repository. A CDN that rewrites content
breaks that property even when it breaks nothing functionally.

## Headless verification

`web/smoke.mjs` runs the same fixture through the same wasm under Node,
verifying the honest trailer and refusing a one-bit tamper. CI runs it on
every push, so the page cannot rot silently.
