/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Admission, as `capsule_spawn::runner::preflight::run` decides it. The order of
the Rust is: verify the ID certificate, then `verify_with_publisher` (decode,
cert binding, namespace, `check_ceiling`, one verifying publisher signature per
algorithm `NONOS_PRODUCTION_POLICY` requires, payload, target triple,
endpoints, `check_grant`), then the image ceiling gate on the install word,
then the tier's attestation gate. Every step is a pure accept/refuse, so the
decision is their conjunction; `install_caps` then trims the word by the boot
profile (`profile_gate::caps`) before it reaches the PCB.

The cryptographic primitives are parameters, not axioms: `sigVerify` is
whatever `alg_id::verify` computes. The theorems hold for every such function;
that the real one is unforgeable is outside what Lean checks, and the README
says so.
-/

import Formal.State
import Formal.Data.Policy

namespace Formal

/-- What the model does not compute and takes as given. -/
structure Platform where
  /-- `crypto::asymmetric::alg_id::verify(alg, pubkey, msg, sig)`, `Ok(true)`. -/
  sigVerify : Nat → Nat → Nat → Nat → Bool
  /-- `NonosTrustAnchorPolicy::publisher_key_id_revoked`. -/
  keyRevoked : Nat → Bool
  /-- `image_ceiling::ceiling()`, the image's baked capability ceiling. -/
  imageCeiling : Caps
  /-- The bits `profile_gate::caps` leaves on this boot (Network off on an
      air-gapped profile, Debug withheld from a secret holder). -/
  profileKeep : Caps
  /-- `caller_launches()`: the caller is the desktop shell or a listed launcher. -/
  launcher : Pid → Bool
  /-- The parent of a process (`pcb.parent_pid`). -/
  parentOf : Pid → Pid
  /-- Whether a process owns a protected service name: compositor, input
      router, policy or the net stack (`capability::revoke_target`). -/
  protectedSvc : Pid → Bool

/-- `caps_bits::within_ceiling`. -/
def withinCeiling (required optional ceiling : Caps) : Bool :=
  decide (Within (required ||| optional) ceiling)

/-- `caps_bits::grant_within_manifest`. -/
def grantWithinManifest (required optional granted : Caps) : Bool :=
  decide (Within granted (required ||| optional))

/-- `caps_bits::install_caps`. -/
def installCaps (required optional granted : Caps) : Caps :=
  required ||| (optional &&& granted)

/-- `NONOS_PRODUCTION_POLICY.required`, from the generated table. -/
def requiredAlgs : List Nat := Data.requiredSignatures.map (·.2)

/-- `verify::dispatch::run` for one algorithm: some signature of that
    algorithm, under a key that is not revoked, found in the certificate with
    the same algorithm, verifies over the signed region. -/
def sigOk (P : Platform) (m : Manifest) (c : Cert) (alg : Nat) : Bool :=
  m.signatures.any fun s =>
    s.alg == alg && !P.keyRevoked s.keyId &&
      match c.keys.find? (·.keyId == s.keyId) with
      | none => false
      | some k => k.alg == alg && P.sigVerify alg k.key m.signedRegion s.sig

/-- The loop in `verify_with_publisher` over the policy's algorithms. -/
def sigsOk (P : Platform) (m : Manifest) (c : Cert) : Bool :=
  requiredAlgs.all (sigOk P m c)

/-- The install word `verify_with_publisher` returns. -/
def Request.install (r : Request) : Caps :=
  installCaps r.manifest.required r.manifest.optional r.granted

/-- The admission decision: `some w` when every check passes, with `w` the
    word installed on the PCB. -/
def admission (P : Platform) (r : Request) : Option Caps :=
  if r.structural &&
      withinCeiling r.manifest.required r.manifest.optional r.cert.ceiling &&
      sigsOk P r.manifest r.cert &&
      grantWithinManifest r.manifest.required r.manifest.optional r.granted &&
      decide (Within r.install P.imageCeiling) &&
      r.attested
  then some (r.install &&& P.profileKeep)
  else none

end Formal
