# Kernel versus trusted computing base

Leaving ring 0 and leaving the TCB are different things. This page says which userland components remain trusted after the migration, what each is trusted for, and how its trust is bounded and made checkable.

## Definitions

- **Kernel**: code that runs at CPL 0. A bug in it can break every isolation property at once, because it can write any page table, any IOMMU table and any other process's memory.
- **TCB for a property**: every component whose misbehaviour can break that property. The TCB differs by property. Confidentiality of the Linux personality's private files depends on vfs; it does not depend on the audio driver.
- **Bounded trust**: a trusted userland component can break only the properties it is trusted for, because the kernel grants it only the capabilities those properties need. A bug in it is contained to its capabilities; a bug in the kernel is contained by nothing.

The threat model's central adversary is A1, a hostile capsule with only its manifest's capabilities (the audit repository's `docs/threat-model.md`, adversary table). Against A1 the question for each component is: if A1 compromises it, what does A1 gain beyond its own manifest?

## The kernel's own job

After the migration the kernel enforces five things and trusts nothing above it for them:

1. **Isolation of address spaces.** No thread reaches a frame unless a capability it holds maps it there.
2. **Capability integrity.** Capabilities can be created only by the kernel, copied only with equal or fewer rights, and revoked by whoever holds the parent.
3. **The image ceiling.** No process ever holds a capability whose rights the image's ceiling excludes, whoever asks ([migration/spawn-gate.md](migration/spawn-gate.md)). This is a monotone bound enforced in ring 0; no userland verdict can raise it.
4. **The measurement record.** The kernel itself hashes the pages of a new address space at the moment its first thread starts, appends the digest to an append-only log and extends it into PCR 14. A userland loader, verifier or TPM service cannot make that record describe different bytes, drop an entry or reorder it ([09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md)).
5. **Device containment.** A device's DMA reaches only the IOMMU domain of its holder; when the holder dies, the kernel detaches the domain and masks the device's interrupts without asking userland ([10-target-device-authority.md](10-target-device-authority.md)).

Everything else, including deciding which capsule may run, is userland.

## The general pattern: the kernel states facts, userland decides

This is the shape of the Linux personality's network authority ([12-target-network-authority.md](12-target-network-authority.md)), and it is the pattern of the whole target. The kernel records and reports facts it observed itself and cannot be told: who sent a message, what bytes a process started with, which programs a guest family ran, which capabilities a process holds. A userland authority reads those facts and makes the policy decision, and the kernel enforces the decision only through the capabilities the authority hands out and the ceiling it can never exceed.

Two rules keep the facts trustworthy:

1. **A fact is never derived from what the asker says about itself.** Today vfs serves the kernel-internal client as sender 0 and then takes a caller pid from the payload on trust (`userland/capsule_vfs/src/server/handlers/util.rs:53-56`); a fact source must stamp the sender itself. In the target the badge is that stamp.
2. **A fact is never silently dropped because it arrived early.** It is refused, so the operation fails, or buffered against an owner that exists. The seal gap violated this ([09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md), "The rule for kernel-recorded facts").

Where else the pattern applies:

| Decision | Facts the kernel supplies | Authority that decides |
|---|---|---|
| May this image run, with which classes | `D_file` and `D_load` computed by the kernel; the creator's class mask | verifier, bounded by the ceiling |
| Optional capabilities a capsule receives beyond what its proof covers (today bounded by the manifest and not proved, `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:81`) | The proved classes and the approved classes, recorded separately in the verdict and the measurement log | verifier for the grant; any remote verifier, from the log, for whether to trust it |
| Which namespace a vfs request belongs to | The sender's badge | vfs ([migration/vfs-guarded-trees.md](migration/vfs-guarded-trees.md)) |
| Which device a driver may hold | The device table from enumeration; the holder's measured digest | devmgr |
| Whether a guest family may reach the network | The family's supervisor, its generation, its kernel-recorded measured set and the personality's kernel-set argv | `net.anon` and `net.socks5` in the interim; netgrant in the target ([12-target-network-authority.md](12-target-network-authority.md)) |

## Trusted userland components

| Component | Trusted for | Holds | If A1 compromised it, A1 gains | How the trust is bounded | How it is checked |
|---|---|---|---|---|---|
| **init** (root task) | Spawning the boot set in order, restarting watched services, handing each service its initial capabilities | Initially every capability the kernel creates; it delegates and then keeps only spawn authority, the supervision endpoints and the boot package | Any capability init still holds: in the worst case every authority the image allows, up to the ceiling | The kernel's image ceiling caps what init can grant. Init gives away device authority to devmgr, verification to verifier and keeps no device, TPM or file authority after boot. Its own image is part of the signed kernel image, so its identity is the kernel's | Its digest is in the kernel's measurement record and in the bootloader's PCR measurement of the kernel image; its spawn plan is data in the boot package, hashed with it |
| **verifier** | Deciding whether a capsule's certificate, manifest and attestation hold, and which capability set the manifest allows | A badged reply endpoint whose badge the kernel recognises as verdict authority; read access to frames it is handed; nothing else | The ability to approve an image that should be refused, within the image ceiling | It cannot raise any process above the ceiling (kernel rule 3). It cannot make the measurement record lie (kernel rule 4): the record holds what was mapped, so a wrongly approved image is still recorded as itself and shows in the attestation document. It holds no device, file or network authority | The kernel grants verdict authority only to a process whose measured digest equals the verifier digest pinned in the kernel image ([migration/spawn-gate.md](migration/spawn-gate.md)) |
| **loader** | Mapping an approved image's segments into a new address space | During one spawn: the capability to the child's address space, still unstarted, and the image frames | The ability to map different bytes into one child | Any change shows as a digest mismatch when the kernel starts the child, and the start is refused. The loader holds the child's address space only until start and never the child's other capabilities | The kernel's start-time digest check |
| **devmgr** | Assigning devices to drivers, sequencing reset and wipe between holders | The device root: per-device config-window, device-frame, IOMMU-domain and IRQ capabilities | Assignment of devices to the wrong driver; denial of service | DMA stays inside IOMMU domains whatever devmgr does: it can hand a device to a driver, never let a device reach memory outside that driver's domain. Revocation on holder death is the kernel's. Its policy tables are data in the boot package | Device grants are recorded per holder and listed in the attestation document |
| **tpm** | Every TPM command after boot except the kernel's PCR 14 extend: PCR reads, quotes, the machine key, the device secret, enrolment | The TPM's locality-0 register page as a device-frame capability, nothing else | Use of the TPM as the system does: deriving the machine key, quoting PCRs | The TPM itself binds keys to PCR state: a compromised tpm service on a correctly measured boot can derive keys the system would derive, and cannot forge a quote over PCR values the platform did not reach | Remote attestation through `nsh_attest`, a pure userland verifier of the quote that already exists |
| **vfs** | Separation of each client's files | The store's block capability (through storage) and one badged endpoint per client namespace | Every file it serves | Structural namespaces ([migration/vfs-guarded-trees.md](migration/vfs-guarded-trees.md)) reduce what a vfs bug leaks to whole namespaces, which shows in review as a missing root check rather than a missing filter | Host proofs over the real handlers (`userland/fs_proofs`) |
| **volume** | Confidentiality and integrity of the encrypted data volume | Endpoints to storage, tpm and the keyring | The volume's plaintext | Keys come from the TPM through tpm, bound to PCR state; the volume service holds no other authority | The existing `volume_kill` model proofs move with it |
| **names** | Mapping service names to endpoint capabilities | One endpoint capability per registered service | Misdirection of a lookup | A lookup returns a capability; what the capability can do is fixed by whoever created it. Sensitive services (verifier, tpm, devmgr) are not reached through names at all: init hands their capabilities directly | Registration is recorded with the registrant's measured digest |
| **net.anon, net.socks5** (interim grant authority) | Carrying anonymity streams; in the interim also deciding whether a Linux family may use the network | Network; the kernel family-fact call `MkFamilyFact`, which only they may make; the grant rows | Network for any Linux family; the streams of every client of the fronts | The registry rule keeps guests off every other network service; the decision is the shared `linux_net_grant` crate with no I/O; `MkFamilyFact` only reads. In the target netgrant takes the decision and the fronts return to carrying streams and enforcing a badge's destinations ([12-target-network-authority.md](12-target-network-authority.md), "What the fronts are now trusted for") | Host tests of `linux_net_grant`; the existing `net_anon_proofs` and `capsule_socks5_proofs` crates |
| **linux** (personality) | Separation between guests, and guests from everything else | Per guest: address-space and thread capabilities, a namespace in vfs, and narrow network capabilities ([12-target-network-authority.md](12-target-network-authority.md)) | Every guest it serves, and the network capabilities its guests were granted | It no longer holds Network as a role. In the interim the registry rule confines it to the two anonymity fronts, which check a grant row on every stream; in the target it holds per-family factories from netgrant. It holds no device, TPM or verifier authority | Its digest is in the measurement record; its guests' programs are verified through verifier before they run |

Everything else (drivers, the network stack, the desktop, applications) is outside the TCB for every property except the one its own capabilities touch. A compromised NIC driver can read the packets it handles; it cannot read the volume, reach another device's DMA or obtain a capability it was not given.

## What makes userland trust checkable

Three mechanisms, all enforced by the kernel, turn "trusted" into a statement a reviewer or a remote verifier can test:

1. **Identity by measurement.** Every process carries the digest the kernel computed over its mapped pages at first start. Trusted roles (verdict authority, device root) are granted only to the digests pinned in the signed kernel image. A replaced verifier or devmgr is a different digest and gets no authority.
2. **Authority by capability.** A component's damage is bounded by the capabilities it holds, which are listed per process in the attestation document. The list is short and mechanical to read: for the verifier it is one endpoint.
3. **A ceiling the kernel enforces.** Whatever the trusted components decide, the kernel refuses any capability whose rights the image ceiling excludes. The ceiling is built from the capsule catalogue at build time and a release build fails if it is missing ([migration/spawn-gate.md](migration/spawn-gate.md)).

## Today, for comparison

Today every component in the table above is either ring 0 (init, the verifier, the loader, the device policy, the TPM driver, the volume, the name service) or relies on asking ring 0 about a pid's capability word (vfs through `mk_cap_check`, `userland/capsule_vfs/src/server/fs_gate.rs:46`). The TCB for every property is therefore the whole kernel, 349,816 compiled lines, plus the service in question.
