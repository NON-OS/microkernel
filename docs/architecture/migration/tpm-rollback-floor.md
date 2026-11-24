# Migration: the TPM and the rollback floor

Finding: NKA-2026-006 (low, adversary A3: physical access or another operating system on the same machine). Target: the TPM is a userland service; [04-classification.md](../04-classification.md) and [09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md).

## Current state

### The rollback floor

The bootloader keeps an anti-rollback floor in two TPM NV indices: a counter at `0x01000020` and a base at `0x01000021`; the floor is counter minus base. It defines the counter with attributes `0x02040014` and an empty index authorisation (`nonos-bootloader/src/security/tpm_nv/consts.rs:22-26`, `nonos-bootloader/src/security/tpm_nv/floor_cmd.rs:25-36`), defines the base under the owner hierarchy and writes and locks it on first boot (`nonos-bootloader/src/security/tpm_nv/floor_cmd.rs:38-46`, `nonos-bootloader/src/security/tpm_nv/floor_seq.rs:78-95`), and refuses any kernel whose signed rollback index is below the floor.

The kernel reads the floor again on its way to userland: two `TPM2_NV_Read` commands authorised by each index itself with an empty password (`src/security/tpm/boot_reads/floor.rs:38-56`), a counter never written read as floor 0 and a counter without a base read as an error (`src/security/tpm/boot_reads/floor.rs:60-66`). The value feeds the measured-boot verdict (`src/security/boot/loader_check/run.rs:42-46`).

### Every TPM operation the kernel performs

The kernel maps the PC Client window, five 4 KiB localities from `0xFED40000` (`src/arch/tpm_window.rs:23-27`, `src/security/tpm/mmio/map.rs:26`), and uses locality 0.

| Operation | Commands | When | Caller |
|---|---|---|---|
| Boot verdict | `PCR_Read` (PCR 4), `NV_Read` (floor) | Once, before userland | `src/kernel_core/init/entry/microkernel_main.rs:26` |
| Capsule event | `PCR_Extend` (PCR 14) | Every capsule admission | `src/security/capsule_measure/extend.rs:44`, from `src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:46` |
| Machine key | `StartAuthSession`, `PolicyPCR`, `PolicyGetDigest`, `CreatePrimary`, `HMAC`, `FlushContext` | On demand | Volume key (`src/fs/blockfs_volume/key_pick.rs:38`), local-build key (`src/security/local_build/identity.rs:41`), `CryptoMachineKey` (`src/syscall/dispatch/crypto/machine_key.rs`) |
| Device secret | `StartAuthSession`, `PolicyPCR`, `PolicyGetDigest`, `LoadExternal`, `VerifySignature`, `PolicyAuthorize`, `CreatePrimary`, `HMAC`, `FlushContext` (`src/security/tpm/device_secret/derive.rs:50-54`, `src/security/tpm/device_secret/authorized.rs:42-53`) | On demand | Volume key v2 (`src/fs/blockfs_volume/key_pick.rs:60`), handover (`src/fs/blockfs_volume/handover/sector.rs:43`), `MkDeviceSecret` |
| Enrolment | `NV_ReadPublic`, `NV_Read` (EK certificates), `ReadPublic`, `ActivateCredential`, `PolicySecret`, `Sign`, `Hash` | On demand | `MkEnroll` |
| Attestation document | `CreatePrimary` (AK), `PCR_Read`, `Quote` | On demand | `MkAttestDoc` |

None of these needs ring 0 after boot. Each is a command buffer written to the TPM's FIFO or CRB registers and a response read back. What makes them sensitive is who may ask (a vendor-authority capsule for the device secret and enrolment, any Crypto holder for its namespace's machine key), and deciding who may ask is an authorisation question that needs no CPU privilege.

## The boundary failure

NKA-2026-006 is a failure at the firmware-to-kernel boundary, and the kernel is not where it fails. Anyone who can send TPM commands with the default empty owner and index authorisation, for example by booting another operating system, can increment the counter (raising the floor until every shipped kernel is refused), undefine the base (resetting the floor to 0), or define the base first with a value of their choosing. The audit reproduced all three against `swtpm` with the loader's own code.

The kernel's part is small: its read path assumes the indices it reads are the bootloader's, and authorises the read with an empty password. That assumption is what the attack breaks.

## Target design

### The floor

What the TPM allows decides the design. An index the owner hierarchy defines cannot carry `TPMA_NV_POLICY_DELETE`: the TPM 2.0 library specification (Part 2, `TPMA_NV`; Part 3, `TPM2_NV_DefineSpace`) refuses `POLICY_DELETE` unless `PLATFORMCREATE` is also set, and `PLATFORMCREATE` needs the platform hierarchy, which PC firmware normally disables before it starts the loader. The repository already shows this: the swtpm test `an_owner_counter_cannot_carry_policy_delete` defines the counter with the loader's own command and `POLICY_DELETE` added, and the TPM answers `0x2C2`, `TPM_RC_ATTRIBUTES` on `publicInfo` (`userland/tpm_enroll_proofs/src/security/tpm/live/floor_tests.rs:82-102`); the loader's header says the same (`nonos-bootloader/src/security/tpm_nv/floor_seq.rs:31`). The loader defines both indices under `TPM_RH_OWNER` (the `authHandle` in `nonos-bootloader/src/security/tpm_nv/consts.rs:22-26`).

So an owner index is deleted with `TPM2_NV_UndefineSpace` under the owner authorisation and by nothing narrower, and the whole owner hierarchy, both indices with it, is deleted by `TPM2_Clear` under the lockout or the platform authorisation. The design protects each path where the TPM allows it and records the floor outside the TPM for the path it cannot close.

- **Deletion is stopped by the owner and lockout authorisations.** After the loader has defined the indices on the first boot, the installer, through the tpm service, sets the owner and lockout authorisations (`TPM2_HierarchyChangeAuth`) to random values and keeps neither. Then no software on the machine can undefine either index or clear the owner hierarchy; only the firmware's own clear remains. Keeping neither value means NØNOS cannot redefine the indices later either; a later change of the indices goes through a TPM clear, which the floor shadow below makes safe.
- **Increment is bound to the genuine loader.** The counter is defined with `TPMA_NV_POLICYWRITE` and an `authPolicy` of `PolicyAuthorize(release_policy_key)`. The release key signs an approved policy of `PolicyCommandCode(TPM2_CC_NV_Increment)` and `PolicyPCR` over PCR 4 as it stands when the enrolled loader runs. A signed approval without the PCR term would be public and replayable, so it would stop nobody; the PCR term is what confines increments to the loader. PCR 4 differs between firmware boot paths, because firmware also measures a failed attempt at another boot option, so the release signs one approval per platform family and per boot path (the live stick and the installed disk), and the installer puts the NØNOS entry first in `BootOrder` so the installed path stays fixed ([15-decisions.md](../15-decisions.md), question 6). Increments only raise the floor, so what this guards against is denial of boot, a lesser harm than rollback.
- **The base is written only under the same policy.** It stays written once and write-locked until deleted, as today (`nonos-bootloader/src/security/tpm_nv/floor_cmd.rs:38-46`), but its write moves from `AUTHWRITE` with an empty password to `TPMA_NV_POLICYWRITE` with the counter's `authPolicy`; the release key's approved policies cover `TPM2_NV_Write` and `TPM2_NV_WriteLock` under the same PCR 4 term. An index's `authValue` is chosen by whoever defines it and is not part of its name, so with `AUTHWRITE` a squatter that defined a correctly named base would know the password and could have written it. With `POLICYWRITE` the `authValue` opens no write.
- **Reads stay authorised by the index with an empty password** (`TPMA_NV_AUTHREAD`, empty `authValue`), so the floor can be read by anyone, which leaks nothing, and the read path of `src/security/tpm/boot_reads/floor.rs:38-56` keeps working unchanged.

#### Squatted indices

Before NØNOS is installed, the owner authorisation is empty, so anyone can define `0x01000022` or `0x01000023` first. The loader therefore never trusts an index it finds: it reads each index's public area with `TPM2_NV_ReadPublic` and compares the index's name with the name of its own definition. The name is `nameAlg` followed by the hash of the public area, which holds the index handle, `nameAlg`, the attributes, `authPolicy` and `dataSize`; the TPM returns the public area and its name together, and the loader checks both.

- **A different name is refused.** The floor is then unreadable, and today's rule applies: refuse under a profile that requires a TPM, boot with protection off otherwise. The installer reports it.
- **An equal name with `TPMA_NV_WRITTEN` clear is adopted.** The index is exactly the loader's own definition and holds nothing yet, so the loader writes it as on a fresh TPM.
- **`TPMA_NV_WRITTEN` set means the write policy was satisfied.** For both indices that policy is the loader-bound one above, so a squatter cannot have written either. For the counter, written means incremented under it.
- **The state bits are part of the name.** `TPMA_NV_WRITTEN` and, for the base, `TPMA_NV_WRITELOCKED` change the attributes and with them the name. The loader compares the public area with those state bits masked, or equivalently holds the expected name for each allowed state (unwritten; written; written and locked).
- **What a squatter keeps.** It can still undefine the index, which is the deletion case again, closed by the owner authorisation. It chose the `authValue`; with `POLICYWRITE` that opens no write, and with `AUTHREAD` a non-empty value only makes the loader's read fail. That is a refusal to boot or an unprotected boot, never a rollback.

#### When the owner authorisation is already taken

Windows sets the owner authorisation to a random value and does not keep it. On a machine where another system has done that before NØNOS is installed, the loader's `TPM2_NV_DefineSpace` under an empty owner authorisation fails, no index exists, and the floor read returns nothing. Today's loader then refuses to boot under a profile that requires a TPM and otherwise boots with "rollback protection is off" (`nonos-bootloader/src/security/anti_rollback/floor_rule.rs:34-40`, `nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-60`). The target keeps that rule, with two additions:

- The installer reads `ownerAuthSet` from `TPM_PT_PERMANENT` before installing and says, before anything is written, that the TPM floor is unavailable unless the TPM is cleared, and that a clear ends the other system's TPM-bound keys (for Windows, BitLocker's TPM protector).
- The floor shadow below still holds, so such a machine has rollback protection against anyone who cannot write its disk.

The reverse order has the same shape: once NØNOS holds the owner authorisation, another system that wants it asks for a TPM clear, and the clear deletes both indices.

#### The delete-both reset

Undefining both indices, or a TPM clear (`TPM2_Clear` under the lockout authorisation, or the firmware's own clear under the platform authorisation, which no operating system can prevent), leaves no counter and no base. On the next boot the loader defines a counter, which the TPM starts above every value a counter on it has held, writes the base equal to it and reads floor 0 (`nonos-bootloader/src/security/tpm_nv/floor_seq.rs:98-104`). The TPM state then matches a machine never installed, and the swtpm test `a_tpm_whose_counters_counted_starts_the_floor_at_zero_through_a_clear` pins that behaviour on purpose (`userland/tpm_enroll_proofs/src/security/tpm/live/floor_tests.rs:44-67`). Deleting only the base has the same effect. With the authorisations set, only the firmware's clear remains, and an A3 attacker with physical access can use it.

#### The floor shadow

An "enrolled" marker in the encrypted data volume's header has been proposed (`docs/security/rollback-floor-authorisation.md`), so that an installed machine whose floor has restarted at 0 is caught. This specification keeps the aim and changes three things.

1. **The loader must read it.** The rollback check happens before the kernel runs. A rolled-back kernel will not check a marker it predates, so a check in the kernel or the volume service comes too late. The loader has no data-volume code at `551bbd34` (its file code finds `kernel.bin` on its own volume, `nonos-bootloader/src/loader/file/own_volume.rs:29-32`), so this adds a small, fixed-offset read through UEFI block I/O.
2. **It must be outside the encryption.** The loader holds no volume key, and after a TPM clear no key exists to derive one. The marker is a plaintext field of the header.
3. **It should carry a value.** A bit saying "enrolled" can only turn a reset into a refusal, and a legitimate clear (a firmware update, another system asking for ownership) then stops every boot until someone re-enrols. A **floor shadow**, the highest rollback index this installation has accepted, lets the loader hold `floor = max(TPM floor, shadow)`, boot the current kernel, whose index is at least the shadow, and raise the fresh TPM counter back to it with today's `raise_with`. A legitimate clear then heals itself on the next boot, and only an older kernel is refused. The volume service writes the shadow after a successful boot, from the index the loader passes in the handoff (a new field; the handoff carries no rollback index today). The loader only reads the shadow.

What it does not do: the shadow is unauthenticated, and no key survives a clear to authenticate it. An attacker who can write the disk can erase or lower it, and A3, which includes booting another system, can. So the shadow stops a reset by anyone who does not also edit the disk, and turns every accidental reset into a non-event; against an A3 attacker who both clears the TPM and edits the disk, nothing short of a measured, write-protected store would do, and the platform offers none to the OS. An attacker who raises the shadow refuses current kernels, which needs disk write and so adds nothing to wiping the disk. A reinstall formats the volume and starts a fresh shadow, which is the intended meaning of a reinstall.

The new counter index avoids redefining a live index whose attributes cannot be changed. Migration defines the new pair and sets the new base to the new counter's value less the old floor, so the floor carries across.

### The TPM service

The `tpm` userland service holds the TPM's locality-0 page as a device MemoryObject, received from devmgr, and the ACPI TPM2 table as a read-only MemoryObject from acpi (today the transport reads the start method from that table, `src/security/tpm/transport/detect.rs`). It serialises every command, as the kernel's `transact` lock does today, and serves:

```rust
enum TpmRequest {
    PcrRead { pcrs: u32 },                          // bitmap of PCR indices
    PcrExtend { pcr: u8, digests: Bytes },          // PCRs other than 14; PCR 14 is the kernel's
    FloorRead,                                      // -> { counter: u64, base: Option<u64> }
    MachineKey { label: Bytes },                    // the label is prefixed by the caller's badge-derived namespace
    DeviceSecret { purpose: u32 },                  // only from callers whose badge is vendor-authority
    Enrol(EnrolStep),                               // ek_public, ek_certificate, ak_public, activate_credential, ak_sign
    Quote { pcrs: u32, nonce: [u8; 32] },           // only from attest
}
```

Who may ask what is decided by the badge on the caller's endpoint capability, which init mints: attest gets `PcrExtend` and `Quote`, volume gets `MachineKey` under its own label space and `DeviceSecret`, verifier gets `MachineKey` under `local_build/`, and so on. Today the label is bound to the caller's attested namespace inside the kernel (`src/syscall/dispatch/crypto/machine_key.rs`); the badge carries the same binding.

### What stays in ring 0

One command: `PCR_Extend` of PCR 14, issued by the kernel at `process_start` from its append-only measurement log, at locality 1, through a minimal FIFO or CRB command path of about 300 lines. Everything else in `src/security/tpm` leaves, and the kernel maps only the locality-1 register page. [09-target-spawn-and-boot-trust.md](../09-target-spawn-and-boot-trust.md), "PCR 14 and receipts", gives the reason: AgentProof receipts and the nsh attestation replay PCR 14, and a userland service that decided what reaches it could skip, reorder or invent capsule events. With the kernel extending, a compromised tpm service can only add extra extends, which break the replay and fail closed.

The kernel and the tpm service share the TPM through its locality arbitration, so neither waits on a lock the other holds. Whether the shipped platforms expose locality 1 to the OS is a platform check ([14-open-questions.md](../14-open-questions.md)).

## Interface specification

The floor's NV definitions, as `TPM2_NV_DefineSpace` parameters, both under `TPM_RH_OWNER`:

```
counter  nvIndex 0x01000022  nameAlg SHA256  dataSize 8
         attributes  COUNTER | POLICYWRITE | AUTHREAD | NO_DA    (PLATFORMCREATE and POLICY_DELETE clear: the TPM refuses them under the owner)
         authPolicy  PolicyAuthorize(release_policy_key)
         approved    PolicyCommandCode(NV_Increment) then PolicyPCR(PCR 4 of the enrolled loader), signed per release, platform family and boot path
         approved    PolicyCommandCode(NV_Write), PolicyCommandCode(NV_WriteLock), each then PolicyPCR(PCR 4 of the enrolled loader), signed per release, platform family and boot path
base     nvIndex 0x01000023  nameAlg SHA256  dataSize 8
         attributes  ORDINARY | POLICYWRITE | AUTHREAD | WRITEDEFINE | NO_DA
         authPolicy  PolicyAuthorize(release_policy_key), as the counter
         authValue   empty; written once, then write-locked
on every boot, before using either index:
         TPM2_NV_ReadPublic; the name must equal the name of the definition above, with WRITTEN and WRITELOCKED masked
after first boot, by the installer through tpm:
         TPM2_HierarchyChangeAuth(TPM_RH_OWNER, random) and TPM2_HierarchyChangeAuth(TPM_RH_LOCKOUT, random); neither value kept
```

The release policy key is the P-256 key the build already stages for the device secret (`build_trust_stage.rs:43-57`).

### Interim: sealing the hierarchies before the tpm service exists

Step 1 lands before the tpm service (step 3), so the installer seals the owner and lockout hierarchies through one interim kernel call, `MkTpmOwner`, which is deleted when the service takes the TPM over. Sealing cannot be undone short of a firmware clear, and whoever can do it can lock every other system out of the TPM, so the call is gated more tightly than any existing one:

```
MkTpmOwner(op, flags) -> state word or errno          interim: deleted in step 3
  op STATE   no capability. Returns ownerAuthSet, lockoutAuthSet, and whether the new pair
             exists and passes the name check.
  op SEAL    refused unless, in this order:
             1. the caller is system tier and its attested identity (its measurement in the
                attestation registry, not only its registered name) is the installer's;
             2. the boot is a live boot;
             3. ownerAuthSet and lockoutAuthSet are both clear (EALREADY otherwise; never attempted);
             4. the kernel's own TPM2_NV_ReadPublic of both new indices matches their definitions;
             5. flags carries the consent bit, set by the installer only after the person said yes
                to: NØNOS takes ownership of this TPM; another system then needs a firmware TPM
                clear to use it; a clear does not lower NØNOS's rollback protection.
             6. the TPM implements TPM2_SetPrimaryPolicy (TPM_CAP_COMMANDS; ENOTSUP otherwise).
             Then, in this order, stopping at the first failure and reporting the exact state:
             a. TPM2_SetPrimaryPolicy(TPM_RH_OWNER) to PolicyCommandCode(TPM2_CC_CreatePrimary);
             b. a probe: one key derived through the empty owner password and again through a
                CreatePrimary policy session; the two must be byte-identical, or SEAL stops
                here with both hierarchies still unsealed;
             c. HierarchyChangeAuth on OWNER, then on LOCKOUT, each with 32 fresh bytes from the
                kernel RNG that never leave the kernel and are zeroised after use.
             Nothing is retried.
```

Every key the kernel derives (the machine keys, the device secret, the volume keys and the local-build keys) comes from `TPM2_CreatePrimary` under `TPM_RH_OWNER` (`src/security/tpm/machine_key/create.rs:38`, reused by `src/security/tpm/device_secret/authorized.rs:51`), authorised today by the empty owner password. Changing the owner authorisation alone would refuse every one of those derivations, and the data volume would never open again. The primary policy keeps exactly one command open under the owner hierarchy: after the seal, the kernel creates its primaries through a policy session that satisfies `PolicyCommandCode(CreatePrimary)`. The storage seed does not change, so every derived key keeps its value and no volume is migrated. `NV_UndefineSpace`, `NV_DefineSpace`, `EvictControl`, `HierarchyControl` and `Clear` stay closed.

The loader never takes ownership on its own: after a deliberate clear, for example to hand the TPM to another system, NØNOS boots with the shadow's protection and seals again only if the person reinstalls and agrees.

The floor shadow, a plaintext field of the data volume's header:

```
offset   fixed, in the header's plaintext part
magic    u32le 0x5744_4853   /* "SHDW" */
index    u64le               /* highest rollback index accepted by this installation */
```

The bootloader's sequence changes in `nonos-bootloader/src/security/tpm_nv/floor_seq.rs`: on finding the old indices and not the new ones, define the new pair and set the new base to the new counter less the old floor; from then on read and increment only the new pair; take the floor as the larger of the TPM floor and the shadow; after verifying a kernel, raise the TPM counter to its index as today. The handoff gains the accepted rollback index.

## Migration steps

1. **Protected indices in the bootloader and the installer.** Define the new pair as above and migrate the floor; the installer sets the owner and lockout authorisations after the first boot and reports an `ownerAuthSet` TPM before installing; correct the header comment in `floor_seq.rs` and the rollback documentation.
   *Invariant*: no software without the owner or lockout authorisation can delete either index; no increment or base write succeeds outside the enrolled loader's PCR 4 state; the loader uses an index only if its name equals the loader's own definition. *Test*: the audit's `swtpm` harness as a host test beside `userland/tpm_enroll_proofs/src/security/tpm/live/floor_tests.rs`: undefine, clear under lockout, increment and base write from a different PCR 4 all fail; an index pre-defined with a different name (other attributes, `authPolicy` or size) is refused; an index pre-defined with the right name and `TPMA_NV_WRITTEN` clear is adopted and the floor starts as on a fresh TPM; `an_owner_counter_cannot_carry_policy_delete` keeps passing.
2. **The floor shadow.** The handoff field, the volume service's write after a successful boot, and the loader's read and `max` rule.
   *Invariant*: after any TPM reset on an installed machine, a kernel below the shadow is refused and the current kernel boots and restores the TPM floor. *Test*: under `swtpm`, raise the floor to 5, clear the TPM, offer index 4 (refused) and then index 5 (boots; the floor reads 5 again). The kernel's read of the new pair (`src/security/tpm/boot_reads/floor.rs`, now `0x01000022` and `0x01000023`, still an empty-password index read) lands in the same step, and the verdict's floor equals the loader's log line.
3. **The tpm service.** Ship the userland service holding the locality-0 page, with today's command builders moved into a `no_std` crate. Move `MkDeviceSecret`, `MkEnroll`, `MkAttestDoc`, `CryptoMachineKey` and the volume's and local build's key derivations to it, one at a time; each kernel call stays until its last user moves.
   *Invariant*: every key derived is the same bytes as before (same label, same PCR policy). *Test*: derive each key both ways in a test build and compare; open an existing data volume.
4. **Boot verdict to userland, PCR 14 at locality 1.** The attest service computes the boot verdict through the tpm service; init gates the boot set on it. The kernel's PCR 14 extend moves from the full driver to the minimal locality-1 command path, fed from the append-only measurement log.
   *Invariant*: the same PCR 14 value for the same boot set, in the same order. *Test*: quote PCR 14 before and after for a fixed boot set.
5. **Remove the rest of the kernel's TPM code.** Delete `src/security/tpm` apart from the extend path, `src/security/boot`, `src/security/capsule_measure` and `src/security/attest_doc`; the kernel maps only the locality-1 page.
   *Invariant*: the only TPM command ring 0 sends is `PCR_Extend` of PCR 14. *Test*: a kernel build contains one TPM command code, `0x182`; boot, receipts and nsh attestation verify through the service.

## Does the in-place fix stay necessary?

Yes. It is the fix: the authorisations on the owner and lockout hierarchies, the write and increment policies, the name check and the shadow are what stop a local TPM user, wherever the code that reads the floor runs. Moving the read to userland neither helps nor hurts. The audit's own remediation, `POLICY_DELETE` on the base, cannot be applied to an owner index, as above.

## Risks

- **TPM sharing.** Two drivers for one TPM corrupt each other's command sequences. The kernel's driver must be gone, or idle and unmapped, before the service maps the window; step 5 follows step 3 with no overlap in which both send commands.
- **Machines already in the field.** Their indices were defined without policy. Step 1 leaves the old indices in place and stops trusting them; an attacker can still play with the old pair, which nothing reads.
- **Owner authorisation.** Discarding the owner and lockout values is deliberate: nothing in NØNOS needs them after the first boot. A user who later wants the TPM for another system clears it from the firmware, and the shadow restores the floor on the next NØNOS boot.
- **Dual boot.** A machine where another system holds the owner authorisation has no TPM floor; under a profile that requires one it does not boot, and otherwise it relies on the shadow. The installer says so before it writes anything.
- **The PCR term of the increment policy.** A firmware update changes PCR 4 and, until a release approves the new value, stops increments. The loader then cannot raise the floor; it still boots kernels at or above it, and the shadow keeps the accepted index.
