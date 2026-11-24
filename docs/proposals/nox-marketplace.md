# Proposal: a NOX market for confined work

Status: proposal. Nothing here is built, deployed or scheduled.

## The idea in one paragraph

On most systems, "this program only touched what it said it would" is a vendor's promise. On NØNOS it is
a fact the kernel enforces, records and can have a TPM sign. A boxed run is sealed into a 240-byte record
(`src/process/foreign/seal/record.rs`), an AgentProof receipt carries that record under a TPM quote of a
measured release kernel (`docs/agentproof/receipt.md`), a STARK can show that no call outside the declared
set succeeded without revealing what the program did (`userland/isolation_stark`), and a machine can prove
it is an enrolled NØNOS device without saying which one (`userland/capsule_prove`). This proposal is a
market whose goods are work done under those guarantees: jobs run in a declared box with a receipt, time
on an attested host, and proofs. NOX settles them through the contract suite the project is already
redeploying.

## What already exists

The commerce plumbing is largely written. What it lacks is a product only NØNOS can offer.

| Piece | Where | State |
|---|---|---|
| Prefunded payment channels with cumulative vouchers, budgets and settle | `userland/capsule_payment`, `userland/nonos_nox_meter`, `userland/payment_proto` | Capsule written; no profile ships it |
| Escrow fair delivery: 1 KiB chunks, Merkle roots, encrypted delivery, key reveal, dispute | `userland/nonos_nox_chunks` | Library written, tested against independent derivations |
| Chain reads and broadcasts over Nym or Anyone, never direct | `userland/capsule_nox_settle`, `userland/settle_proto` | Capsule written; no profile ships it |
| Pay-per-use entitlements and their broker | `userland/nonos_nox_license`, `userland/nonos_nox_broker` | Libraries only; no capsule links them |
| Contracts: Settlement, Escrow, ListingBond, ListingRegistry, ProofAnchor | `userland/nonos_nox_config` | Being redeployed from scratch; every address is `None` today |
| Price units: install, proof, call, second, byte | `userland/nonos_nox_config` | Defined |
| NOX Shield private payments | `userland/capsule_shield`, `userland/shield_core` | Sepolia only; mainnet pool not deployed |

Because the suite is being redeployed, the listing kinds below can shape what Settlement, Escrow and
ProofAnchor accept, before any of it is fixed on chain.

## What is sold

Three listing kinds, ordered by how little the buyer has to trust the seller.

### 1. Proof work

**The good.** A STARK proof for a statement the buyer names, priced in the existing `proof` unit.

**Who buys.** Publishers who need attestation trailers for their capsules, and anyone who needs a
NØNOS-format proof produced by a machine with more memory than theirs. The device prover alone needs
3 GiB free and peaks near 2.6 GB (`userland/capsule_prove/README.md`).

**Why the seller needs no trust.** A STARK proof checks itself. The buyer verifies it with the same
verifier the kernel or the evaluator uses, and pays only for a proof that verifies. Escrow fair delivery
already supports this: the proof arrives encrypted under a committed key, the key is revealed on chain,
and a chunk that fails its hash is disputable (`userland/nonos_nox_chunks`). A proof that decrypts
correctly yet fails verification needs one addition: a dispute path whose evidence is the failing
verification, judged off chain under a bonded arbiter, or on chain once a verifier contract is affordable.

**The limit.** This is only for statements whose witness the buyer can hand over. A capsule's bytes are
public, so trailers qualify. A private Shield spend does not: handing its witness to a stranger gives
away the payment's privacy, so Shield spends stay on the buyer's own machine.

### 2. Attested sessions

**The good.** Time on someone else's NØNOS machine, over nsh/1, priced per `second`.

**Why NØNOS.** The nsh client checks the host before it sends any credential: a NONOSATT v5 document,
a TPM quote, the release's PCRs, PCR 4's event rules, and an IOMMU that was enforcing
(`docs/nsh/nsh-v1-attest.md`, `userland/nsh_attest`). The session runs under a role grant that names
what it may do (`docs/nsh/nsh-v1-roles.md`). The host signs a chained session record that can be
checked offline (`docs/nsh/nsh-v1-record.md`). A live session keeps nothing after power-off. A rented
Linux VPS offers none of this.

**Payment.** The buyer's meter opens a prefunded channel to the host's payee and signs a cumulative
voucher each minute. The host keeps serving while vouchers keep arriving and stops when they stop. The
signed session record is the evidence if the two disagree about time served.

**What is missing.**
- A payee side for vouchers. Today the meter is the payer and settles its own vouchers through
  `nox_settle`'s DRAIN path (`userland/capsule_payment/README.md`). A host needs to receive, check and
  later redeem them.
- A listing type in `ListingRegistry` and `marketplace_abi` for a host.
- A way for a buyer to choose among hosts without learning who they are (see *Seller identity* below).

### 3. Sealed runs

**The good.** "Run this program, in this box, on my input, and give me the output with a receipt."

**Who buys.**
- Someone who wants to run a program they did not write, such as a model-written agent, on a machine
  that is not theirs, under a box they chose. This is `linux --box` (`README.md`, *Run a program you did
  not write*) sold as a service.
- Someone without the memory for a local model tier.
- Someone who needs a job done over Nym or Anyone from a machine whose route is already up.

**The evidence.** An AgentProof receipt (NAPR v0) binds a release kernel, measured into PCR 9, to a
sealed run of a named program in a named box. Its verifier checks:
- the TPM quote and the release PCRs;
- the IOMMU state;
- every capsule that was running, and which of them held Network;
- that the run finished with no saturated counts (`docs/agentproof/receipt-verify.md`).

An isolation STARK can be added to show that no call outside the declared set succeeded, without
revealing counts (`userland/isolation_stark`).

**What is missing, and is required before this can be sold.**
- **The receipt does not bind the job's input or output.** The NFSR record holds the supervisor, the
  app_id, a trace digest and a histogram commitment (`src/process/foreign/seal/record.rs`). `MkSealNote`
  admits only the APP_ID and DIAG slots (`src/process/foreign/seal/sys_note.rs:34`). A seller could run
  the right program on the wrong input and still produce a valid receipt. A receipt v1 needs a job slot
  carrying BLAKE3 of the input and of the output, written by the vendor supervisor. The receipt then
  trusts `capsule_linux` to report those digests faithfully. That is one more reason the Linux
  personality should be small, and the startup work in this initiative points the same way.
- **Receipts from strangers must chain to real hardware.** v0 pins the attestation key on first use and
  checks no EK certificate chain (`docs/agentproof/receipt-verify.md`, *The trust file*). Between
  strangers, first-use pinning proves nothing: a seller running a modified kernel against a software TPM
  would produce receipts that verify under their own pinned key. Sealed runs and attested sessions need
  either EK certificate validation or the anonymous device proof against a registry of enrolled devices.
- **Confidentiality against the operator is not offered.** The box confines the program. It does not
  confine the person who owns the hardware, who can read memory with physical access (adversary A3 in
  the audit's threat model). The listing must say so, and buyers with secret inputs should not use it.

## Seller identity

Sellers in an anonymous market should be unlinkable across jobs. A TPM quote under one attestation key
links every job a machine does. The anonymous device proof (`userland/capsule_prove`) shows "enrolled
bootloader, enrolled kernel, enrolled device" against a registry whose root the verifier supplies,
without naming the device. The market would publish its own registry root of enrolled seller devices,
anchored in ProofAnchor. A seller proves membership once per job. Enrollment requires a manufacturer EK
certificate, so a software TPM cannot enroll. Who runs enrollment, and how its honesty is checked, is
worked through in [nox-marketplace-seller-enrollment.md](nox-marketplace-seller-enrollment.md).

## What NOX does here

- **The settlement asset** for channels and escrow. The suite's contracts must be fee exempt, because
  NOX takes up to 300 bps on transfer between non-exempt parties (`userland/nonos_nox_config/README.md`).
  The settle capsule already refuses a contract that is not (`userland/capsule_nox_settle/README.md`).
- **The bond asset.** Sellers of sessions and sealed runs post a ListingBond. A receipt that fails
  verification, or a session record showing service the buyer paid for and did not get, is grounds to
  slash it.
- **Private payment** through Shield, once Shield is on mainnet.

This proposal adds no emissions, no staking yield and no governance role for the token.

## Why the kernel work comes first

Every guarantee above is the kernel's. NKA-2026-020 shows an enrolled capsule writing across a physical
frame boundary from the in-kernel ELF relocator. As the owner reports it, the write reaches a forged
page-table entry. A capsule with that primitive can rewrite the seal record, the registry and the
histogram, and every receipt it touches would still verify. A market that sells receipts sells the
kernel's TCB. The TCB reduction under way in this initiative is the precondition for this market. It
cannot ship in parallel.

## Order of work

1. **Precondition.** NKA-2026-020 and the rest of the audit's kernel findings fixed. ELF parsing and
   relocation out of ring 0. The contract suite redeployed with the listing kinds below in mind.
2. **Proof work.** It is the smallest trust surface, because proofs check themselves. It needs escrow, a
   proof listing type and the verification-dispute path.
3. **Attested sessions.** These reuse nsh, its attestation and its records, and the meter. They need a
   payee side for vouchers and a host listing type.
4. **Sealed runs.** These need receipt v1 (the job slot), EK-chained or anonymous device evidence, and
   a seller-side job runner. The runner takes a job, runs it in its box through `capsule_linux`, asks
   `nonos.prove` for the receipt, and delivers through escrow.
5. **Later.** Bonded boxes for software listings: a publisher stakes behind "this program needs nothing
   outside its declared box". This waits on an open problem. A refused call can come from what a user
   asked the program to do as easily as from the program itself, so refusals alone are weak evidence of
   bad faith.

## Risks and open questions

- **Demand.** No evidence yet that people will pay for any of these. The cheapest test is proof work for
  the project's own publishers, before any public listing.
- **Law.** Selling compute and software for a token raises questions about the marketplace operator's
  obligations in each jurisdiction. This needs legal review before anything is listed.
- **Token control.** The mainnet NOX contract lets a Safe change the transfer fee and blacklist
  addresses (`userland/nonos_nox_config/README.md`). Every buyer and seller is exposed to that, and the
  market's documentation must say so.
- **Dispute judging.** Escrow can judge a chunk that fails its hash. It cannot judge "the proof does not
  verify" or "the receipt is invalid" on chain at an acceptable gas cost. Who arbitrates those, and how
  they are bonded, is open.
- **RPC linkability.** Settle's reads share net.anon's circuits, so an RPC operator can link what one
  device reads (`userland/capsule_nox_settle/README.md`). The market inherits this.
- **Pricing unit.** NOX only, or NOX with a stable unit beside it. The fee on transfer and price
  volatility both bear on whether sellers will quote in NOX.

## Contracts: extensible from the first deployment

The redeployed suite carries the extension points for every listing kind
now, even where a kind is switched on later. Redeploying a second time costs more than these extension points do.

- **Listings name a kind and a unit.** The catalogue's commerce block already carries payee, bond id,
  unit, unit price, price kind, settle mode and proof kind (`userland/marketplace_abi/README.md`, schema
  3). Units are 32-byte tags (`userland/nonos_nox_config/src/units.rs`), so a new kind adds a tag
  without changing the format. ListingRegistry should store the kind and unit tags as given, and refuse
  only those not yet activated.
- **Activation is per kind.** Each kind starts inactive. Activating one binds it, once and permanently,
  to the dispute judge it uses. Proof work activates first; sessions and sealed runs stay inactive until
  their gaps above are closed.
- **Escrow takes a dispute judge per kind.** Today's chunk-hash dispute is one judge. A proof-verification
  judge and a receipt judge plug into the same interface later, without moving the escrow.
- **ProofAnchor anchors several kinds of root.** Proofs, receipts and seller-registry roots
  ([seller enrollment](nox-marketplace-seller-enrollment.md)) each get a kind tag.
- **No upgradeable proxies.** A new judge is a new contract bound at activation. Nobody can swap the
  logic behind an escrow that already holds funds.

## Decisions

Date: 2026-11-23.

1. **First listing kind: proof work.** Sealed runs do not go live until two gaps are closed: the
   receipt's binding of job input and output, and the stranger-receipt forgery gap.
2. **Seller enrollment** gets its own design pass, which must not block proof work:
   [nox-marketplace-seller-enrollment.md](nox-marketplace-seller-enrollment.md). It records its own
   decisions.
3. **Contracts carry the extension points from the first deployment,** as described above.
4. **Legal review happens before any mainnet settlement in NOX or ETH goes live.** It does not gate
   design or internal engineering.
