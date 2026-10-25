# nsh v1: threat model

## What is protected

1. **The user's credentials.** These are the user key pair and the role
   grant. Neither may reach a host that has not proved itself.
2. **The session.** This is the keystrokes and the output. They must stay
   confidential against a recorder who keeps the traffic for a later quantum
   computer.
3. **The host's claim about itself.** "This host runs release R on enrolled
   machine M." The claim must be true, or the client must refuse.
4. **The kernel's limits on the login.** The login may do only what its
   role names.
5. **The record.** It must say what was done, and nobody may alter it,
   including the host's own administrators after the fact.

## Who attacks, and what stops them

| Attacker | Can | Stopped by |
|---|---|---|
| Passive recorder, quantum later | keep every byte | ML-KEM-768 inside X-Wing; X25519 alone is not trusted |
| Active network attacker | change, drop, replay or reorder frames | transcript-bound keys; AEAD on every frame after hello; the host signature over the transcript |
| Impostor host, its own machine running a genuine release | pass a quote of a real image | enrolment: the AK must be in a certificate from the fleet's enrolment authority (attest.md) |
| Enrolled host, modified image | an enrolled AK, but other PCRs | the quote must carry PCRs 4, 9, 14 equal to the release's values (needs kernel change K1) |
| Stolen host key, other machine | sign transcripts | the key is accepted only together with a fresh quote naming its hash; the key is sealed to the measured chain |
| Replayed attestation | send an old quote | the challenge is derived from this handshake's transcript |
| User acting within their role | anything the role allows | the role; the record shows what they did |
| User beyond their role | ask for more | the kernel refuses the call; the refusal is recorded |
| Insider with one authority key | mint a grant | escalation can require a second approver's key; roles carry a ceiling set on the host |
| Host administrator, after the fact | edit logs | the record is chained and signed; the client keeps a copy |
| Stolen user key | log in as the user | grant lifetimes; the signed revocation list; the record names the key |

## What v1 does not defend against

- **A compromised client machine.** It holds the user key.
- **Physical attacks on the TPM or the CPU, and side channels on the host.**
- **A bug in the release the client accepts.** Attestation proves what
  runs. It does not prove that what runs is correct. Any proof that the
  release keeps a rule comes from that rule's own proof crate, not from nsh.
- **The P-256 attestation key falling to a quantum attacker.** The TPM's
  quote is classical. A forged quote would have to be made live, inside the
  handshake timeout. A quote recorded today and broken later breaks no
  session. The host's own hybrid key also signs the transcript, so host
  authentication fails only if both fall.
- **Denial of service beyond the limits stated in wire.md.** Those are the
  handshake timeout, the session cap and the frame caps.
- **The host clock.** Grant validity uses the host's RTC. A host whose clock
  has been moved can admit an expired grant. The record carries the clock
  value, so a moved clock is visible afterwards, but v1 does not prevent it
  (see the plan's risk R6).

## Trust roots the client holds

These are kept in the client policy (attest.md):

- The release signers: the Ed25519 and ML-DSA-65 key ids the bootloader
  checks.
- The expected PCR 4, 9 and 14 values for each accepted release.
- The fleet enrolment authority's hybrid key.
- The minimum rollback index.
