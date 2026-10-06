# Changelog

Notable changes, including the defects this repository found in itself. The
bug entries are not housekeeping; they are the evidence that the gates work,
and removing them would remove the proof.

## Unreleased

### The engine unified

The `nonos-stark` crate in this tree is the merge of two lineages that had
been developed apart: the NONOS kernel's attestation engine and the zkolang
shielded-transfer engine. Unifying them under both test corpora surfaced
three defects that neither deployment could see alone, each in code the other
side exercised differently:

- **A completeness bug in the extension-field deep check** (from the
  transfers lineage's fix): a padding case the kernel's assemblies never
  triggered.
- **A root divergence in the streamed periodic committer**: an AIR with no
  periodic columns produced an empty tree through the materialized committer
  and a tree of hashed empty rows through the streamed one. Same AIR, two
  roots, decided by which committer ran. Every transfers-side AIR carries
  periodic columns, so only the kernel corpus could catch it. Both sides
  derived the identical fix independently, and the edge case now lives in the
  crate's own tests.
- **An emitter-verifier differential in the canonical trailer entry point**:
  `verify_attestation_trailer` read the sibling and direction regions in the
  opposite order from the builder beside it, and then routed the proof into
  the retired base-field pipeline with no grinding and no extra blowup. No
  deployed consumer ever noticed, because the kernel and the bootloader each
  carry their own parse; the crate's front door disagreed with its own
  factory and nothing walked through it until the gate was compiled to
  WebAssembly for the browser page. Fixed to the deployed wire format and
  pipeline; a round-trip test now builds a trailer and verifies it through
  this entry point so the two can never drift again. The Lean totality model
  had faithfully modelled the wrong parser and was restated over the real
  layout, which is worth reading twice: a proof is about the model it states,
  and only a test can tie the model to the wire.

### The hybrid measurement

`measure_capsule_hybrid` and `MeasuredSet::commit_hybrid`: BLAKE3 the
artifact, absorb only the digest. On the CI runner, enrolling 64 MiB of
artifacts went from 21.95 s to 16.7 ms. The two measurement schemes are
domain separated so no artifact measures to the same leaf under both and a
trailer from one never verifies against the other. The security reasoning,
including when it does not transfer to other systems, is in
`docs/membership.md`.

### The browser verifier

`verifier-wasm/` compiles the deployment's gate to 94 KB of WebAssembly, and
`web/` verifies a real enrolled kernel's trailer in the visitor's tab, with a
one-bit tamper button. The build and a headless verification run in CI.

### Measurement as a first-class artifact

`bench/` holds CI-produced JSON records: timing with raw samples and spread,
soundness derived from the compiled parameters under both the provable floor
and the conjectured regime, and the hash architecture study. A regression
gate compares medians and refuses to compare across host shapes.

**A benchmark bug, documented deliberately**: the first enrolment sweep
reported figures that were faster than the cost of their own fastest
component, because the timed closures dropped their results and the optimizer
elided part of the work. Caught by the floor audit before publication, fixed
by keeping every timed result alive, and written into
`docs/methodology.md` as the worked example. If your benchmarks have never
caught you, they are not testing you.

### Machine-checked facts

Twelve Lean 4 modules, core library only, no `sorry`, CI-audited axioms:
the trailer parse's totality, the padding separation that survives a hash
break, the no-downgrade property of the parameters, the Goldilocks reduction
identity, and the transfers lineage's streaming and batch-inversion facts,
imported verbatim with provenance.

### Reproducible parameters

The in-circuit Poseidon's constants derive from BLAKE3 of a public domain
string and a Cauchy matrix that is MDS by theorem. `regenerate-constants`
re-derives every one independently of the implementation, and CI fails on any
mismatch. The specification is `docs/poseidon-spec.md`.
