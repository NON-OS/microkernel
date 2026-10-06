# The Two Trust Modes

Every rule in the capsule machinery runs in one of two postures, and
the difference is a single question: does this environment hold
signing seeds, or must it prove things without them?

## Signing mode, the default

The owner flow. Certificates are signed under the trust-anchor seeds,
manifests under the publisher seeds, and each manifest is verified
immediately after it is produced. Key checks run first and fail
loudly with the exact missing path, because a signing flow that
discovers a missing seed halfway through has already written partial
state. Scratch CI also runs in this mode, against a throwaway trust
chain minted by the bootstrap script, so full-system lanes exercise
the same rules the owner runs without touching anything real.

## Reuse mode: NONOS_TRUST_REUSE=1

The production pipeline posture. No rule may demand a seed, and the
committed keystore is the truth to be checked, not regenerated:

- the trust-anchor policy must exist and is never resealed
- certificates must exist; a reuse build cannot mint one
- each manifest must exist, verify under the baked policy, and its
  enrolled payload hash must equal a fresh BLAKE3 of the just-built
  ELF, via `capsule-sign verify-manifest --elf`
- the capsule policy root must exist; the set is never re-enrolled
- the trust-key check answers true, because nothing will sign

The measurement equality is the load-bearing check. It converts the
question "can CI sign this capsule" — which must always be no — into
"is CI's build of this capsule the enrolled build", which is
answerable from public material and fails by name when a build
drifts. The cure for that failure is making the capsule reproducible,
never minting a key: enrollment happens over reproducible-builder
output precisely so this equality can hold everywhere.

## Choosing a mode

Nothing chooses implicitly. The kernel repository's production lanes
export the flag in their provisioning steps; developer machines and
scratch lanes never set it. A rule that behaves differently between
the modes carries the `ifeq` visibly at its definition, so reading
`capsule.mk` shows both postures side by side rather than hiding one
behind the other.
