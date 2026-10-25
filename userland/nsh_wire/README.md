# nsh_wire

The nsh/1 wire format, as specified in
[docs/design/nsh-v1.md](../../docs/design/nsh-v1.md) and its parts.

The crate covers:

- the frame length field and its cap at each stage;
- which frame kind may arrive at each stage;
- both plaintext hellos;
- `HOST_PROOF` and the enrolment certificate it carries;
- `USER_AUTH` and `ESCALATE`, with the role grant inside;
- the session bodies;
- the session record.

Every parser takes a byte slice and returns a typed view over it, or a
`WireError`. Each `WireError` maps to the one reason code the peer is told.
The crate is `no_std`, has no dependencies, never allocates and contains no
`unsafe`, so the server capsules and the host client build the same source.

What the crate settles, and what it leaves to its callers:

- **Settled here:** everything the bytes alone can settle. That covers
  sizes, order, reserved fields, name shapes, the grant's own rules and the
  record's sequence.
- **Left to the caller:** signatures, the clock, the attestation document
  and policy. Each parsed view hands back the exact bytes that its
  signature covers.

The proofs are in [nsh_wire_proofs](../nsh_wire_proofs).
