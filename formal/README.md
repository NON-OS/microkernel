# NØNOS whole-system kernel model

eKisNonos

`formal/` is one Lean 4 state machine for the security-critical core of the
kernel, and five properties proven over every state it can reach. The
per-mechanism proofs in `verification/lean` each look at one function; this
model puts the capsules, their capability words and tokens, their page
mappings, frame ownership, the surface registry and IPC into one state and
one step relation, so the theorems hold across every interleaving of spawn,
grant, revoke, map, unmap, IPC, surface sharing and teardown.

The model is fed from the code. The capability bits, the ABI syscall list,
the syscall-to-capability table and the manifest signature policy are
generated from the Rust by `scripts/gen_tables.py`; CI regenerates them and
fails on any difference, and `userland/kernel_proofs` evaluates the generated
table against the real `cap_table::is_allowed` on every registered syscall.

## Build

```sh
cd formal
lake build                 # every theorem checks, zero errors
scripts/check_axioms.sh    # axiom closure of the headline theorems
python3 -I scripts/gen_tables.py --check   # tables match the Rust
```

Toolchain `leanprover/lean4:v4.15.0` (`lean-toolchain`), core Lean only, no
mathlib. `verification/lean` is a path dependency so the model cites its
lemmas instead of proving them again. A clean build takes a few seconds. The
`formal` job in `.github/workflows/lean.yml` runs all three commands and
publishes the axiom profile.

## What is proven

Every theorem below is stated for every `Platform` (every signature
primitive, image ceiling and boot profile) and every state
satisfying `Reachable P s`, the inductive closure of `Step` from boot
(`Formal/Step.lean`). File: `Formal/Headline.lean`.

1. **Capability monotonicity** (`capability_monotonicity`). Under the
   shipped grant rule, a live capsule holds nothing outside what its manifest
   declared (required and optional) under the image ceiling, nothing above
   its publisher's certificate ceiling, and nothing outside the envelope
   recorded at its spawn. There is no side condition on Admin: a grant
   outside the target's envelope is refused.
   The model also keeps the rule the kernel ran before that bound
   (`GrantRule.callerOnly`), which let an Admin holder give any bit it held
   to any process. `AdminGrant.old_rule_grant_exceeds_admitted` is a
   reachable trace under that rule in which a capsule holds a bit its
   manifest never named; `AdminGrant.fixed_rule_refuses_the_old_witness`
   shows the shipped rule refuses the same grant. `capability_bound_any_rule`
   and `capability_monotonicity_without_admin` hold under either rule.
2. **Admission soundness** (`admission_soundness`). A live capsule was
   admitted on a request whose manifest asks for nothing above the
   certificate ceiling, whose grant stays inside the manifest, whose install
   word is under the image ceiling, that carries a verifying Ed25519 and a
   verifying ML-DSA-65 publisher signature under unrevoked certificate keys,
   and that passed the structural checks and the attestation gate; its word
   is that install word trimmed by the boot profile.
3. **Syscall-gate completeness** (`syscall_gate_completeness`,
   `syscall_outside_allowlist_denied`). A handler runs for capsule `p` and
   syscall `nr` only if `p`'s token is its own and minted under its current
   revocation epoch and the token's allowlist permits `nr`. The allowlist is
   derived from the token's word at every token install, as its
   `allowlist::for_token` does, so `nr` is in the ABI and the generated cap
   table passes on the word `p` holds; the handler's effect belongs to `nr`.
   Outside that, no handler is reached, and the only outcomes are EPERM with
   the state unchanged (Linux personality) or the caller torn down (a native
   capsule is killed). `syscall_within_manifest`: under the
   shipped grant rule, every syscall but MkAttestDoc is reached only if the
   table passes on the word the manifest declared under the image ceiling;
   `only_attest_doc_negates` proves over the generated table that MkAttestDoc
   is the one requirement with a negation.
4. **W^X preservation** (`wx_preservation`). W^X holds at boot, every step
   preserves it, so no reachable state maps a page writable and executable.
5. **Isolation preservation** (`isolation_preservation`). The invariant
   `Iso` holds at boot and every step preserves it (spawn, load, every
   handler, protect, quarantine, teardown, slot free). In a reachable state:
   two distinct capsules share a frame only as views of one surface, each of
   them the project or a receiver the project named; a private frame is mapped in
   one address space at one address; every token is bound to its capsule.
   `caps_change_only_by_target`: only an operation aimed at a capsule
   changes its capability entry. Surface sharing is the one sanctioned
   crossing and the invariant states it.

### Further properties

These hold under either grant rule unless stated, in every reachable state.

- `dead_holds_nothing`: a pid with no live capsule has no mapping, owns no
  private frame, holds no surface view or share right, and has no queued
  message. `spawn_starts_empty` follows: a new capsule inherits nothing from
  an earlier holder of its pid.
- `handler_requires_capability`: grant runs only for an Admin holder, mmap
  and munmap only for a Memory holder, an IPC send only for an
  IPC holder, the surface calls only for the matching graphics capability.
  The bit comes from the generated table.
- `revoke_takes_effect`, `quarantine_takes_effect`: after a revoke, the
  target holds none of the revoked bits, and every syscall whose table entry
  needs one of them is refused to it in the next state.
- `revoke_bound`, `protected_service_answers_to_init`: a revoke that removes
  bits from another process runs only for an Admin holder that is init or the
  target's parent, and only init can revoke from a process that owns the
  compositor, the input router, policy or a net stack service. A process may
  drop its own bits with a valid token alone.
- `ipc_touches_only_receiver`: a send changes the receiver's inbox and
  nothing else.
- `empty_word_reaches_only_tty_query_or_random`, `launcher_matters_only_for_app_launch`,
  `every_abi_number_is_claimed`, `only_attest_doc_negates`: facts about all
  193 entries of the shipped table. A token with no capability reaches only
  MkTtyQuery and CryptoRandom. Only MkAppLaunch reads the launcher question. Every ABI number
  is claimed by a table group. Only MkAttestDoc has a negated requirement.

## What is assumed

`scripts/check_axioms.sh` prints the axiom closure of each headline theorem.
Every one is `[propext, Quot.sound]`, two of Lean's three standard axioms.
The model declares no axiom and contains no `sorry`; the script fails CI on
either.

The model computes no cryptography. Each of these is a field of `Platform`,
so every theorem holds whatever it is:

| Parameter | Stands for | What the theorems do not say |
|---|---|---|
| `sigVerify` | `crypto::asymmetric::alg_id::verify` | that a signature cannot be forged; the theorems say the check ran and passed |
| `keyRevoked` | `NonosTrustAnchorPolicy::publisher_key_id_revoked` | that the revocation list is current |
| `imageCeiling` | `image_ceiling::ceiling()` | which ceiling the image bakes |
| `profileKeep` | `profile_gate::caps` (Network on air-gapped profiles, Debug for secret holders) | which bits a profile clears |
| `launcher` | `caller_launches()` | who the registry lists as a launcher |

`Request.structural` and `Request.attested` are booleans the model takes as
given: manifest decode, ID certificate verification (trust anchor
signatures, validity window), certificate binding, namespace, payload hash,
target triple and endpoint drift; and the tier's STARK attestation gate.
Their correctness is covered elsewhere (`verification/lean` Spawn,
Stark.Attest; `userland/stark_proofs`), not here.

## How the model maps to the code

| Model | Rust |
|---|---|
| `Caps`, `has`, `excess`, `Within` | `u64` capability words; `Capability::bit` (`src/capabilities/types/defs.rs`); `a & !b == 0` |
| `Data.capabilityBits` | `capability_table!` in `src/capabilities/types/defs.rs` (generated) |
| `Data.abiSyscalls`, `Data.sys*` | `SyscallNumber` (`src/syscall/numbers/defs.rs`), `abi::REGISTRY` (generated) |
| `Data.capTable`, `Req`, `lookupReq`, `capTableAllows` | `cap_table::is_allowed` and its groups (`src/syscall/contract/cap_table/*.rs`), the `can_*` checks (`src/syscall/caps/checks/*.rs`, `src/capabilities/token/types/authority_*.rs`) (generated) |
| `Data.requiredSignatures`, `requiredAlgs` | `NONOS_PRODUCTION_POLICY` (`src/security/nonos_id_cert/policy.rs`) (generated) |
| `admission` | `capsule_spawn::runner::preflight::run`: `verify_with_publisher` (`src/security/capsule_manifest/verify/mod.rs`), then `image_ceiling::gate`, then the attestation gate |
| `withinCeiling`, `grantWithinManifest`, `installCaps` | `caps_bits.rs` in `src/security/capsule_manifest/verify/` |
| `sigOk`, `sigsOk` | `verify/dispatch.rs::run`, once per policy algorithm |
| `Capsule.fresh`, `Step.spawn` | `install::finish`: `install_caps` then `proc_caps::install_spawn` before `add_to_run_queue` |
| `Capsule.grant`, `Effect.capGrant`, `GrantRule.targetEnvelope` | `sys_cap_grant` (`src/syscall/microkernel/capability/handlers.rs`), decision `grant_rule::grant_decision`, effect `process::caps::grant` |
| `Capsule.envelope`, `envelopeOf` | `pcb.caps_envelope`, recorded once by `process::caps_envelope::record` from `install_caps`: `profile_gate::caps((required \| optional) & ceiling) \| installed` |
| `Capsule.revoke`, `Effect.capRevoke`, `Step.quarantine` | `sys_cap_revoke` / `grant_rule::may_revoke`, `process::caps::revoke`; `process::core::isolation` |
| `Token`, `tokenOk` | `CapabilityToken` minted by `process::caps::new_token`; `resolver::check_asid_binding`, `check_revocation_epoch` |
| `gate`, `Step.call`, `Step.deny`, `Step.denyKill` | `syscall::entry::handle_syscall` → `contract::dispatch` → `resolver::resolve` → `handle_syscall_dispatch`; a refused native capsule is killed, a Linux-personality process gets EPERM |
| `Token.list`, `derive`, `allowed`, `mint` | its `src/syscall/contract/allowlist`: `derive.rs` (`derive`, `for_token`, both launcher answers per registry slot from `cap_table::is_allowed`), `decide.rs` (`allowed(list, nr, caller_launches)`, `decide`), `set.rs` (`Allowlist`); derived at every `install_token` |
| `Perm.wx` | `PagePermissions::is_wx_violation`; `ProtectionFlags::is_wx_violation` (`src/process/address_space/protection.rs`) |
| `Step.load`, `Effect.mmap`, `Step.protect` | `map_page`, `map_page_in_asid`, `update_page_flags` (`src/memory/paging/manager/`), ELF `load_segment/validate.rs`, `AddressSpace::map_page` (`src/process/address_space/ops.rs`) |
| `Effect.munmap` | MkMunmap |
| `Effect.ipcSend` | `IpcMessage` (`src/ipc/nonos_channel/message.rs`): bytes only |
| `Surface`, `Effect.surface*` | `src/kernel_core/surface_registry`: `share_surface`, `permit`, `rights::may_attach`, `attach_surface` (views `user_rw`/`user_ro`), `owner_releases`, `rights_rule::revoke_pid` |
| `State.teardown` | `process::exit::teardown` |

### Runtime checks on the real code

`userland/kernel_proofs/src/formal_model/` includes the Rust each invariant
abstracts and checks the predicate the theorem states. Run with
`cd userland/kernel_proofs && cargo +nightly-2026-01-16 test formal_model`.

| Invariant | Test | Real code exercised |
|---|---|---|
| Syscall gate (allowlist) | `allowlist_tests::the_shipped_allowlist_is_the_model_list`: the bitmaps `allowlist::for_token` derives, read through `allowed` and `decide`, give the generated table's answer (`Formal.derive`) for every registry slot and both launcher answers on 338 capability words; a number with no slot is refused by both | `syscall/contract/allowlist/derive.rs`, `decide.rs`, `set.rs` |
| Syscall gate | `gate_tests::the_model_gate_is_the_kernel_gate`: the generated table, evaluated on several hundred permission words, agrees with `is_allowed` and `is_allowed_as_shell` on all 193 registered syscalls for both launcher values; `generated_capability_bits_match_the_enum`, `generated_abi_list_matches_the_registry` | `cap_table::is_allowed`, `Capability`, `abi::REGISTRY` |
| Admission soundness | `admission_tests`: every 3-bit admission (98,304 cases) and 20,000 full words | `within_ceiling`, `grant_within_manifest`, `install_caps`, `image_ceiling::gate` |
| Capability monotonicity | `capability_tests`: `grant_decision` equals the model's grant precondition; a grant outside the target's manifest is refused, inside is allowed; a caller lacking the bit or Admin is refused; a missing caller or target is refused; nothing is grantable to a process without an envelope; the installed word lies inside the envelope; the old `AdminGrant` trace is refused | `grant_rule::grant_decision`, `may_grant`, `within_envelope`, `may_revoke`, `install_caps` |
| W^X | `wx_tests`: all 65,536 permission words: `is_wx_violation` is `Perm.wx`, and an accepted permission never encodes a W+X leaf; surface views are NX; `AddressSpace::map_page`'s rule refuses exactly W+X, passes W-only and X-only, and no accepted request encodes a W+X leaf | `PagePermissions::is_wx_violation`, `to_pte_flags`, `ProtectionFlags::is_wx_violation`, `ProtectionFlags::to_pte_flags`, `pte_flags` |
| Isolation | `isolation_tests`: random sequences of the real share and rights operations against a ghost record of consent; teardown and release drop rights | `share/rule.rs` `Shares`, `rights_rule.rs`, `rights_attach.rs` |

Drift is caught twice: `gen_tables.py --check` fails when the Rust changes
and the generated files do not, and `the_model_gate_is_the_kernel_gate`
fails when the committed table disagrees with the running one. Changing one
arm of `cap_table/mk_trust.rs` trips both.

### Proofs on the extracted code

Two parts of the gate are now proved equal to the model on the code rustc
compiles, translated by Charon and Aeneas, in `verification/extraction/lean`:

| Theorem | States | Code |
|---|---|---|
| `NonosExtraction.CapTable.is_allowed_is_the_model` | for every syscall, launcher answer and process token, the extracted `cap_table::is_allowed` returns `capTableAllows (word ps) l (tag n)`; one row per syscall, generated by `tools/extraction/cap_table_rows.py` | `src/syscall/contract/cap_table`, the token's `grants` and `is_valid`, every `can_*` check |
| `NonosExtraction.CapTable.bit_is_two_pow` | `Capability::bit c` is `2 ^ idx c` for all 41 capabilities, so `word` is the kernel's word | `capability_table!` |
| `NonosExtraction.Allowlist.set_spec`, `plain_spec`, `launcher_spec`, `empty_permits_nothing`, `allowed_refuses_unregistered`, `allowed_reads_its_slot` | the bitmap reads bit `s % 64` of word `s / 64`, `set` raises exactly the asked bit, `EMPTY` permits nothing, `allowed` refuses a number with no slot and otherwise reads the map the caller's standing selects; for any registry count up to 192 and any slot lookup | `src/syscall/contract/allowlist/{set,decide}.rs` |

The library calls these reach (`Option::or_else`, `Option::map_or`,
`Iterator::any`, `Iterator::all`, `Vec::is_empty`, `usize::div_ceil`) are
defined in the crates' `FunsExternal.lean` by their documented behaviour, not
assumed. The kernel clock and the registry index are Lean `opaque`
constants, which add no axiom; every theorem holds whatever they are. The
tests above remain: they run the code on the host and check the parts not yet
extracted, such as `derive`'s loop and the token MAC check.

## Gaps

What the model does not cover, stated so no theorem is read as more than it
says.

1. **The grant envelope is read, not executed, end to end.** The decision
   `grant_rule::grant_decision` runs on the host; the PCB field, its set-once
   record at spawn and the handler's lookups are kernel code the model
   transcribes. Processes with no recorded envelope (kernel-created
   processes, threads) have a zero envelope, so nothing can be granted to
   them.
2. **Capsules only.** Kernel-created processes (`create_process_with_parent`
   with `AMBIENT_CAPS`, pid 1), threads (`spawn_thread_parked`, which carry
   the process's word) and foreign Linux peers (`process::foreign`, the
   MkPeer* calls) are outside the model.
3. **Spawn is one step.** The real spawn creates the process with inherited
   ambient bits, then installs the manifest word. The model treats it as
   atomic because `install::finish` calls `install_caps` before
   `add_to_run_queue`, so no user instruction runs on the inherited word.
4. **Admission composition is transcribed.** The bit rules run on the host;
   the order `preflight::run` composes them in, and `dispatch::run`'s
   signature loop (`sigOk`), are read from the source, not executed.
5. **Tokens.** Only subject, epoch, word and allowlist are modelled.
   Signature, boot session, ASID and expiry checks are taken to pass for the
   capsule's own token; tokens minted by `new_token` carry no expiry, so
   `is_valid` is the non-empty test. The token's MAC is checked once, when
   the allowlist is derived, and a token that fails it gets the empty list
   (with `nonos-syscall-token-mac-every-call`, on every call as well); the model does not distinguish the two
   modes, since it takes the MAC check to pass either way.
6. **Handlers.** The handlers with an effect in the model are grant, revoke,
   mmap, munmap, IPC send and the four surface calls. Every other handler is
   `Handler.other`: its effect on state outside the model is not checked.
   Device memory (MMIO, DMA, PIO), IRQs and the device broker are not in the
   frame model.
7. **Surface registry.** Receivers are pids in the model and thread groups in
   the code (`holder_of`, `same_process`). The share limits (`NAMED_MAX`,
   `MAX_RECEIVERS`) are left out, which only adds behaviours, so the
   theorems still hold for the code.
8. **Paging.** One leaf per (pid, address); no page sizes, no kernel half,
   no TLB. The W^X refusal of every leaf writer is read from the source; the
   refusal rules and the leaf encoders run on the host.
9. **Concurrency.** Steps are atomic. Locks, atomics and SMP interleavings
   are not modelled.
10. **Words.** The model's words are unbounded `Nat`; the code's are `u64`.
    The generator refuses a bit at or above 64.
11. **Teardown** frees private frames at once; the code's deferred reclaim
    and zeroing are covered by `verification/lean` Zeroization, not here.

## Layout

```
formal/
  lakefile.toml, lean-toolchain, lake-manifest.json
  Formal.lean              root import
  Axioms.lean              #print axioms for every headline theorem
  Formal/Bits.lean         capability words
  Formal/Req.lean          the cap table's expression language
  Formal/Data/*.lean       generated from the Rust; do not edit
  Formal/State.lean        kernel state
  Formal/Admission.lean    the admission decision and the Platform
  Formal/Gate.lean         the syscall gate
  Formal/Step.lean         handlers, steps, Reachable
  Formal/Proofs/*.lean     invariants and their preservation
  Formal/Headline.lean     the five properties
  scripts/gen_tables.py    table generator and drift check
  scripts/check_axioms.sh  axiom and escape-hatch gate
```
