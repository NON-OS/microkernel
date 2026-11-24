# Seller device enrollment for the NOX market

Status: design proposal. It answers who vouches that a seller's machine is real NØNOS
hardware, and how that scales past a handful of machines. Nothing here is built. It supports the
proposal in [nox-marketplace.md](nox-marketplace.md). Proof work, the first listing kind, does not need
it, so it blocks nothing that ships first.

## The question

A buyer of an attested session or a sealed run must be able to tell a real NØNOS machine from a
modified kernel running against a software TPM. Today's evidence cannot do that between strangers:

- AgentProof receipts pin the attestation key (AK) on first use and check no endorsement key (EK)
  certificate chain (`docs/agentproof/receipt-verify.md`, *The trust file*).
- nsh enrollment is per fleet. A fleet operator signs `nsh-enrol/1` certificates for machines they run
  (`docs/nsh/nsh-v1-attest.md`, *Enrolment*). The tool does not show that the AK lives in the machine's
  TPM; that is left to a registrar using MakeCredential and ActivateCredential against the EK
  (`src/security/tpm/enroll`).

A market has no fleet operator who knows every seller. It needs a root of trust that does not depend on
knowing who the seller is.

## What already fits

`nonos-device-attest` proves, in zero knowledge, four things about a device:
- an approved bootloader started it;
- an approved kernel runs on it;
- it holds a secret `s` whose commitment is a leaf of a device registry R;
- `t = tag(s, e)` is its tag for scope `e`.

The proof says which device, bootloader or kernel is none of the verifier's business, and binds itself to
a context `c` (`nonos-device-attest/README.md`). The kernel derives `s` only under a PCR 9 value the
release signed for and under PCRs 0, 4 and 7 as the machine found them. A modified kernel therefore never
sees `s`. R is keyed by BLAKE3 of the EK's public area, and `transcript` and `recompute` let anyone check
that a published root matches its entries.

Three market needs follow from this:
- **Anonymity:** the buyer learns "a real NØNOS device", never which one.
- **Sybil resistance:** one device has one tag per scope. With scope `e = (market, epoch, listing kind)`,
  a device is at most one seller per epoch per kind, and bonds attach to tags.
- **Binding to the job:** with `c` set to the digest of a job's sealed record and its input and output
  digests, the proof says that a genuine kernel's `nonos.prove` bound this device to this run.

What remains open is who keeps R, and how anyone checks that every leaf of R is a real TPM.

## Root of trust

The chain a buyer relies on, from the hardware up:

1. **TPM manufacturer.** The EK certificate chains to a manufacturer root (Infineon, STMicro, Nuvoton,
   Intel PTT, AMD fTPM). This is the only party that can say a TPM is hardware. A software TPM has no
   such certificate.
2. **Registrar.** It checks the EK chain, runs MakeCredential so the device proves it holds the EK, and
   adds `commit(s)` under BLAKE3(EK public area) to R. `enroll` already refuses a revoked key and a
   commitment held under another key.
3. **Release signature.** PCR 9 names a release the vendor signed. The kernel releases `s` only under it.
4. **The device proof itself**, verified by the buyer against a registry root they accept.

The registrar is the one new trusted party. The design goal is to make its honesty publicly checkable,
so that trusting it becomes optional for anyone willing to audit.

## Recommended design: accountable registrars

- **Public transcripts.** Each registrar publishes its registry transcript. Every entry carries the EK
  public area, the EK certificate chain and `commit(s)`. Anyone can recompute the root and check every
  chain against the manufacturer roots. A registrar that enrolls a software TPM leaves a leaf with no
  valid chain, and anyone who reads the transcript can see it.
- **Roots anchored on chain.** A registrar's root is anchored in ProofAnchor (`userland/nonos_nox_config`)
  per epoch. A buyer accepts a proof against any anchored root of a registrar they accept. The market's
  default accepted set is a policy choice, recorded below as a decision.
- **Bonded registrars.** A registrar posts a ListingBond. A transcript leaf whose chain does not verify is
  objective evidence for slashing that bond. Checking it needs only the transcript and the manufacturer
  roots, with no judgement call.
- **Several registrars.** The project runs the first one. Others may join by bonding, so no single party
  can refuse a seller. A seller enrolls with one registrar; buyers see only "some accepted root".
- **Epochs and revocation.** Roots roll per epoch (a day is a reasonable start). An EK found compromised
  goes on the registrar's revoked list and drops out of the next root. Proofs against a root are accepted
  until its epoch ends.

## How it scales

- **Enrollment is a protocol, with no person in the loop for each device.** It is a challenge, an EK
  certificate check and a MakeCredential round trip over the network the seller has chosen.
- **Roots update in batches.** One anchor transaction per registrar per epoch, whatever the number of
  enrollments.
- **Verification stays local.** The buyer checks the device proof against a root they already hold. Its
  cost does not grow with the number of sellers.
- **The registry has a ceiling.** `Registry::new` accepts a depth of at most `MAX_DEVICE_DEPTH`, 20
  (`nonos-device-attest/src/registry.rs:49`). One registry therefore holds at most 2^20 devices, about a
  million, and `enroll` refuses past `1 << depth` (`nonos-device-attest/src/registry.rs:106`). That is ample for launch. Past
  it, the fix is more registrars, which needs no change to the circuit.

## What it costs

- **Seller membership is public until the second circuit exists.** A transcript that carries EK
  certificates says which TPMs are enrolled as sellers. It says nothing about which jobs they run. An EK
  certificate can be tied to a purchase record, so being a seller is visible to a determined observer.
  Hiding membership as well needs the registrar to prove in zero knowledge that every leaf has a valid
  chain, a second circuit. The decisions below make that circuit a precondition for sessions and sealed
  runs.
- **The manufacturers become roots of trust.** fTPM extraction attacks exist (published attacks on AMD
  fTPM), and a physical attacker (A3) can attack a discrete TPM's bus. An attacker who extracts `s` from
  one device can forge proofs for that one device. The per-scope tag limits the damage to one seller
  identity per epoch per kind, and the bond prices it.
- **Revoking a cheating seller across scopes is impossible** without breaking anonymity, because tags in
  different scopes do not link. Within an epoch the market can refuse a tag. Across epochs, slashing the
  bond attached to the tag is the deterrent.

## Changes this needs

| Change | Where |
|---|---|
| EK certificate chain check at enrollment, against pinned manufacturer roots | the registrar tool; `tools/nsh-enrol` is the closest existing code |
| EK chain and certificate carried in each transcript entry | `nonos-device-attest` `Registry` transcript format |
| A transcript auditor anyone can run | a host tool beside `recompute` |
| Registrar roots anchored per epoch | ProofAnchor; a new anchor kind |
| Registrar bond and its slashing condition | ListingBond, or a bond type of its own |
| Receipt v2: the device proof replaces the pinned AK, with `c` bound to the sealed record and the job digests | `userland/agentproof_receipt`, `userland/capsule_prove` |

The device proof has never run on a booted image (`nonos-device-attest/README.md`, *What is not done*).
It needs a booted end-to-end run before anything here depends on it.

## Decisions

Date: 2026-11-23.

1. **The project runs the first registrar, and bonded registrars are admitted from the first day.** The
   project is never the only way in.
2. **Manufacturer roots are accepted per manufacturer and firmware version, from a published list.**
   - A version with a published EK key-extraction attack is not on the list.
   - Firmware TPMs fixed by a firmware update come back once the fixed version is pinned. TPM-Fail
     (2019) affected both Intel PTT and STMicro ST33 firmware before their updates.
   - AMD fTPM is left off at launch. faulTPM (2023) extracts its secrets by voltage fault injection on
     the platform security processor, and no firmware update removes that.
   - The list lives with the registrar policy, so a registrar that enrols an unlisted TPM fails the
     transcript audit like any other bad leaf.
3. **The default accepted set is every bonded registrar whose transcript passes the public audit.**
   There is no curated list: the audit does the curating, and anyone can run it. A registrar whose
   transcript fails is dropped from the next epoch and its bond is slashable.
4. **Seller membership must be private before sessions and sealed runs launch.**
   - Proof work, the first listing kind, needs no enrollment, so it is not delayed.
   - Sessions and sealed runs wait for a registrar's zero-knowledge proof that every leaf in its
     transcript chains to an accepted manufacturer root. Then the transcript publishes commitments
     only, and no EK certificate.
   - This is the expensive option. It is chosen because a list of every seller's TPM is a list of
     the people selling, and this market's premise is that it does not keep such lists.
