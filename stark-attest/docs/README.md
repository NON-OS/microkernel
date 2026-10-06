# stark-core documentation

The library commits a set of artifacts to one 32-byte root and issues each
member a self-contained STARK membership trailer, transparently and
post-quantum. These pages document the machinery from the field up, written
against the source and honest about status; each page states what is tested,
what is inherited, and what is not proven.

## Reading order

1. **[params.md](params.md)** - the four constants every prover and verifier
   share, and why they live in exactly one file. Read this first; it is the
   security posture in miniature.
2. **[field.md](field.md)** - Goldilocks, the carry-exact operations, the
   property-pinned fast reduction, the quadratic extension, the generic
   tower, and the duplicate-truth gates between them.
3. **[hashing.md](hashing.md)** - three hashes, three jobs: Keccak for
   transcript and commitments, width-8 Poseidon inside the circuit, Poseidon
   commitments for recursion.
4. **[fri.md](fri.md)** - the low-degree test, and why it exists in four
   variants along two axes (challenge field, commitment hash).
5. **[transcript.md](transcript.md)** - Fiat-Shamir: tagged absorption,
   ordering as the security, grinding, and what the transcript deliberately
   cannot do.
6. **[membership.md](membership.md)** - the trailer: the statement, the byte
   layout, the strict parse, enrollment via one committed set, and set
   semantics.
7. **[recursion.md](recursion.md)** - verifying FRI inside a STARK: the
   fused monolith, wiring, aggregation; builds on the deep map in
   `src/air/README.md`.
8. **[security.md](security.md)** - every claim with its honest status:
   INHERITED, TESTED, DESIGNED, or NOT PROVEN.

## Also in this directory

- [poseidon-spec.md](poseidon-spec.md): the in-circuit hash, fully derivable
  from the page, with the independent regeneration check CI runs.
- [methodology.md](methodology.md): how every published number is produced,
  and the audit that caught our own benchmark.
- [verifier.md](verifier.md): the gate compiled to WebAssembly and the
  browser page that runs it.

## The one-paragraph mental model

An artifact set is padded to a fixed-width Merkle tree of the in-circuit
Poseidon over the members' measurements; the root is the statement. A trailer
proves knowledge of one leaf's authentication path, with the proof
transcript-bound to the artifact's BLAKE3 measurement and caller-chosen
context bytes, so membership, content and metadata verify as one check. The
proof is a DEEP STARK over the membership AIR, low-degree-tested by FRI with
challenges from the quadratic extension and a ground transcript, committed
with Keccak, and every soundness parameter comes from the verifier's own
parameter module, never from the proof. Proving streams per coset, so memory stays flat as traces grow, and costs
seconds per member at enrollment; verification costs milliseconds anywhere,
forever, with nothing trusted but hash functions.
