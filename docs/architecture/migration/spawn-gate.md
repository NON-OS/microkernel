# Migration: the spawn gate, attestation verification and the image ceiling

Findings: NKA-2026-023 (image ceiling), NKA-2026-026 (`nonos-dev-attest` beside `nonos-production`), NKA-2026-012 (measured-boot verdict). Target design: [09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md).

## Current state

### The gate

`spawn_verified_as` (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:37-79`) runs, in ring 0, for every capsule:

| Step | Where | What it decides |
|---|---|---|
| Built at all | `src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:46-49` | An image with no ELF, certificate and manifest is `FeatureDisabled` |
| Boot profile | `src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:31-40` | Air-gapped, safe and recovery profiles refuse named capsules |
| Identity certificate | `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:35-38` | Signed by the baked trust anchor under `NONOS_PRODUCTION_POLICY`; carries the publisher's ceiling |
| Measurement | `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:54` | `D_file`, BLAKE3 of the ELF |
| Manifest | `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:55-66` | Publisher signature, payload hash equal to `D_file`, target triple, required and optional capabilities within the certificate's ceiling (`src/security/capsule_manifest/verify/caps.rs:21-29`), the requested set within the manifest (`src/security/capsule_manifest/verify/caps.rs:31-39`), declared endpoints |
| Image ceiling | `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:70-74` | Installed capabilities `& !ceiling` must be zero |
| Tier | `src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs:22-28` | Namespace `systems.nonos` or under it is Enrolled, anything else Publisher; a Publisher capsule may not take a system or reserved name (`src/kernel_core/process_spawn/capsule_spawn/runner/publisher_gate.rs:31-35`) |
| Attestation | `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:23-61` | The v4 trailer's Merkle path and STARK proof over `(D_file, required_caps, epoch)`, vendor root first, then enrolled roots (`src/security/capsule_attest/verify.rs:44-73`); under an enrolled root a locally built capsule carries a keyed tag in place of a STARK proof; under `nonos-dev-attest`, a path alone (`src/security/capsule_attest/path.rs:64-66`). Accepted results are cached (`src/security/capsule_attest/path.rs:47-55`) |
| Record | `src/kernel_core/process_spawn/capsule_spawn/runner/preflighted.rs:50-60` | Measurement, capabilities and authority into the attestation registry; PCR 14 extended (`src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:46`) |

The attestation statement covers the manifest's required capabilities, passed at `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:81`. Optional capabilities the spawn site requests are bounded by the manifest and the certificate's ceiling and are not part of the proof; `docs/kernel/processes-and-spawn-gate.md:29` says so.

### The image ceiling

`ceiling()` reads eight bytes staged by the build; a file of the wrong length or a zero value reads back as every capability (`src/security/image_ceiling/value.rs:38-49`). The build copies `nonos-data/trust/policy/image_capability_ceiling.bin`, or writes eight zero bytes when it is absent (`build_trust_stage.rs:29-36`). The committed file holds `0x1fdfffff7d` at `551bbd34`, which refuses bits 40 to 43 that four capsules declare and admits bit 9 that none declares (NKA-2026-023). `scripts/check_image_ceiling.py` detects the disagreement (`scripts/check_image_ceiling.py:37-46`). At `551bbd34` it runs in the make static lane (`nonos-ci/run-static-checks.sh:5325-5333`, reached from `nonos-mk-static` at `mk/40-run.mk:383-384`), but a baseline accepts both known problems until the next seal (`scripts/baselines/image-ceiling.txt`, read from `scripts/check_image_ceiling.py:49`), so it passes; no GitHub workflow runs it. The audit's finding that it was wired into no build is true of the audit's commit, `b3dd1fa37ce2`. The nix build computes its own ceiling per profile from the catalogue (`tools/nix/image.nix:66-71`).

### The development attestation path

`src/lib.rs:44-48` refuses `nonos-dev-attest` only beside `nonos-release`. `nonos-production` does not imply `nonos-release`, so the combination compiles and admits path-only trailers, while three comments say it is refused (`Cargo.toml:343`, `src/security/capsule_attest/path.rs:85-87`; NKA-2026-026).

### The measured-boot verdict

`check_bootloader` decides once whether the loader that ran is the enrolled one (`src/security/boot/loader_check/run.rs:28-55`). With a TCG log, PCR 4 and the rollback floor it calls `nonos_boot_measure::gate::measured`, whose replay keeps only the last `EV_EFI_BOOT_SERVICES_APPLICATION` digest (`nonos-boot-measure/src/tcg/replay.rs:59-60`) and treats it as the loader (`nonos-boot-measure/src/gate/verdict.rs:36-57`). A foreign application that loads the genuine loader without starting it is admitted as `Measured` (NKA-2026-012). A `Refused` or `NoEvidence` verdict halts the boot (`src/kernel_core/init/entry/loader_refusal.rs:27-47`).

## The boundary failures

- **NKA-2026-023**: the one bound the kernel enforces on every capsule fails open when its input is wrong.
- **NKA-2026-026**: what ring 0 admits depends on a compile-time combination, and the guard against the dangerous combination is incomplete.
- **NKA-2026-012**: a verdict is computed from evidence that does not support it. Where it runs does not matter to the bug.

Together they show the spawn gate's two jobs mixed in ring 0: the **bound** (how much authority any capsule may ever hold), which must be the kernel's, and the **decision** (whether this capsule is the one its signatures and proof say), which need not be.

## Target design

### What the kernel enforces itself

1. **The ceiling as a monotone bound.** `class_mask(child) = verdict.class_set ∩ class_mask(creator) ∩ CEILING`, and no capability of a class outside a process's mask is ever transferred into it ([07-target-object-model.md](../07-target-object-model.md)). `CEILING` is a required constant of the kernel image: there is no unrestricted fallback in any build that sets `nonos-production` or `nonos-release`, and a missing, short or zero ceiling file is a build error there.
2. **Verdict authenticity.** A verdict is a kernel object created only through the `VerdictAuthority` capability. The kernel grants that capability once, to the process started from the verifier's pinned entry, whose measured `D_load` equals the digest built into the kernel image.
3. **The measurement.** At `process_start` the kernel measures the image span and refuses a mismatch with `verdict.d_load`, then records it ([09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md)).
4. **Single use.** `process_create` consumes the verdict.

### What the verifier decides

Everything in the table above except the ceiling: certificate, manifest, signatures, endpoint declarations, tier and reserved names, attestation under vendor and enrolled roots, keyed tags of locally built capsules, the acceptance cache, the boot profile's refusals (as data in the verdict request). The verifier is a userland capsule holding one endpoint and the `VerdictAuthority` capability; the verification code is today's, moved into a `no_std` crate so the same source runs in the kernel during the migration and in the verifier afterwards.

### How the kernel trusts the verifier's answer

It trusts it exactly as far as the ceiling and the measurement allow:

| A compromised verifier could | The kernel still guarantees |
|---|---|
| Approve an image whose proof does not verify | The image gets no class outside the ceiling, and the measurement table records its true `D_load` and `D_file` |
| Approve a larger class set than the manifest allows | Not above the ceiling, nor above the creator's own mask |
| Approve a different image than the one being started | Refused: `D_load` must match what is mapped |
| Approve the same verdict twice | Refused: verdicts are single-use |
| Be replaced by a development verifier in a release image | Refused: verdict authority goes only to the pinned digest |

The last row is what removes NKA-2026-026's class from the kernel: no admission policy is compiled into ring 0, and a release kernel pins a release verifier.

### The measured-boot verdict

Moves to the attest service, which reads the handed-over TCG log and asks tpm for PCR 4 and the floor. The fix is the nsh rule the audit names (every application event before the last must be an enrolled boot application or boot manager), applied in `nonos-boot-measure`, which the attest service links. Init refuses to start the boot set and powers off on `Refused` or `NoEvidence`.

## Interface specification

### The verdict

```rust
/// Fields of a Verdict object; written by verdict_create, read by process_create
/// and by cap_identify (so attest can read them).
#[repr(C)]
pub struct VerdictFields {
    pub version: u32,            // 1
    pub authority: u32,          // 1 vendor, 2 enrolled, 3 local, 4 pinned
    pub d_file: [u8; 32],        // BLAKE3 of the image file
    pub d_load: [u8; 32],        // load digest, 09-target-spawn-and-boot-trust.md
    pub span_pages: u32,         // the measured image span
    pub entry_offset: u64,
    pub class_set: u64,          // approved classes: required ∪ requested optional, within the manifest
    pub proved_classes: u64,     // the subset the attestation statement covers (today: required)
    pub namespace_hash: [u8; 32],// BLAKE3 of the manifest namespace, as today's record_namespace
}
```

System calls (target numbering, [08-target-syscall-surface.md](../08-target-syscall-surface.md)):

- `verdict_create(auth: H, fields: *const VerdictFields) -> H`: needs `VerdictAuthority`; returns a Verdict handle the verifier transfers in its reply.
- `process_create(as: H, quota: H, request: *const CreateRequest) -> H`, with

```rust
#[repr(C)]
pub struct CreateRequest {
    pub verdict: CapHandle,      // a Verdict, or 0 with pinned_index set
    pub pinned_index: u32,       // index into the kernel image's pinned table, or u32::MAX
    pub requested_classes: u64,  // further narrowing by the creator
}
```

### The verifier's endpoint

```rust
enum VerifyRequest {
    /// The image is a sealed MemoryObject transferred with the message.
    Capsule { image: CapHandle, cert: Bytes, manifest: Bytes, trailer: Bytes,
              requested_optional: u64, target_triple: Bytes, boot_profile: u8 },
    /// A Linux guest program: no manifest; a policy and a trailer.
    GuestProgram { image: CapHandle, trailer: Bytes, cert: Option<Bytes>, manifest: Option<Bytes>,
                   policy: Bytes },
    LocalSign { image: CapHandle },        // today's MkLocalSign
    EnrolRoot { root: [u8; 32], kind: u8 },// today's MkEnrolDevRoot and MkEnrolLocalRoot, behind consent
}
enum VerifyReply {
    Verdict { verdict: CapHandle },
    Refused { reason: AdmitError },        // today's SpawnError and AttestError codes
}
```

### The pinned table and the ceiling in the kernel image

```rust
/// In a read-only section of the kernel image, produced by the build.
#[repr(C)]
pub struct Pinned { pub d_load: [u8; 32], pub class_set: u64, pub role: u32 } // role 1 verifier, 2 devmgr, 3 names
pub static PINNED: [Pinned; N];
pub static CEILING: u64;   // generated from the catalogue; no fallback in production or release builds
```

## Migration steps

1. **Make the ceiling fail closed.** Generate `image_capability_ceiling.bin` in `build.rs` from the catalogue for the profile being built (the computation `tools/nix/image.nix:70-71` already does), empty its baseline once the committed file is regenerated, run `scripts/check_image_ceiling.py` in a CI workflow as a blocking job, and make `ceiling()` a compile error, not `UNRESTRICTED`, for a missing or zero value when `nonos-production` or `nonos-release` is set.
   *Invariant*: the ceiling equals the union of the shipped capsules' declared ceilings, in every build path. *Test*: the check script exits 0; a build with the file removed fails.
2. **Close the development-attestation guard.** Refuse `nonos-dev-attest` beside `nonos-production` at compile time, unless an explicit development marker feature is also set; correct the three comments.
   *Invariant*: a production build admits no path-only trailer. *Test*: `cargo check` with `nonos-production,nonos-dev-attest` fails.
3. **Fix the boot verdict.** Apply the nsh PCR 4 rule in `nonos-boot-measure`'s replay and verdict.
   *Invariant*: a boot where a foreign application ran before the enrolled loader is not `Measured`. *Test*: the audit's gate-level reproduction as a regression test.
4. **Extract the verification code into a `no_std` crate** (`nonos_admit`): certificate, manifest, attestation, admission cache, dev and local roots, tier and reserved names. The kernel's gate calls it; nothing else changes.
   *Invariant*: the gate admits and refuses exactly the same images. *Test*: run the crate's host tests over every capsule artefact in `nonos-data/trust/capsules`; boot.
5. **Ship the verifier capsule** with today's IPC, serving the `VerifyRequest` set from `nonos_admit`. Move the userland callers of `MkCapsuleVerify`, `MkLocalVerify` and `MkLocalSign` (the installer, `userland/capsule_installer/src/server/handlers/pkg_verify.rs`, and the personality, `userland/capsule_linux/src/linux/attest_publisher.rs`, `userland/capsule_linux/src/linux/attest_local.rs`) to it. The kernel calls stay for one release.
   *Invariant*: every userland verification answer equals the kernel's. *Test*: run both on every request in a test build and compare.
6. **Introduce verdicts with an interim kernel table.** A kernel table of single-use verdicts, written by `MkVerdictCreate` from the process whose measurement equals the pinned verifier digest. `spawn_verified_as` and `MkCapsuleLoad` accept a verdict in place of running `preflight`; they still apply the ceiling and record the measurement.
   *Invariant*: no capsule receives a capability bit outside the ceiling; every capsule's recorded measurement is the digest of the bytes the kernel loaded. *Test*: a test verifier with a different digest is refused verdict authority; a verdict for image A used to spawn image B is refused.
7. **Move every spawn onto verdicts and delete the kernel's verification.** Remove `src/security/{capsule_manifest,nonos_id_cert,nonos_trust_anchor,capsule_attest,admission_cache,dev_roots,local_build,attest_policy}` and the verification crypto from ring 0; delete `MkCapsuleVerify`, `MkLocalVerify`, `MkLocalSign` and the root-enrolment calls; delete `nonos-dev-attest` from the kernel's features (it becomes a feature of the verifier's build, and a release kernel's pinned table never names a development verifier).
   *Invariant*: as step 6. *Test*: boot every profile; compare the attestation registry with step 6's.
8. **Class masks.** With the object model, the ceiling check moves from `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:70-74` to capability transfer, where it binds every path, including grants (`MkCapGrant` today bounds grants by the envelope, `src/syscall/microkernel/capability/handlers.rs:36-57`, which `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:77` derives from the ceiling).
   *Invariant*: no process ever holds a class outside the ceiling. *Test*: a capsule asking a granting service for a capability of a class outside its mask receives `EPERM` from the kernel at transfer.

## Do the in-place fixes stay necessary?

| Fix | After the migration |
|---|---|
| NKA-2026-023: generated, checked, fail-closed ceiling | Necessary, and more important: the ceiling is the one bound left in ring 0 |
| NKA-2026-026: compile guard | Necessary until step 7, then the feature leaves the kernel |
| NKA-2026-012: nsh PCR 4 rule | Necessary; the code moves to the attest service unchanged |

## Risks

- **Bootstrapping order.** Until init runs in ring 3, the verifier is spawned by the ring-0 gate like any capsule, verified by the kernel's own copy of `nonos_admit`, and then pinned. The pinned digest must be the one the build produced for the same image the trust set signs; the build must compute both in one step.
- **Verdict confusion.** A verdict names `D_load`; the kernel compares it at start. A verifier bug that computes `D_load` wrongly refuses good images, which fails closed.
- **Optional capabilities remain unproved.** The verdict carries `proved_classes` separately so the attestation document can show which classes a proof covers. Whether optional capabilities should be proved is an open question ([14-open-questions.md](../14-open-questions.md)).
- **Locally built capsules.** Their keyed tag uses a key the kernel derives from the TPM (`src/security/local_build/identity.rs:41`). In the target the verifier asks tpm for an HMAC under a label bound to the verifier's measured digest, so only the pinned verifier can mint or check such tags.
