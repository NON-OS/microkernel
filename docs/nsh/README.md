# nsh/1

nsh/1 is the remote shell protocol of NØNOS. A client checks that the host
is an attested NØNOS machine before it sends any credential. A user then
logs in with a role grant signed by a hybrid key.

## Files

| File | Covers | Implemented in |
|---|---|---|
| [nsh-v1.md](nsh-v1.md) | Roles, handshake order, the hybrid suite, versions. | `nsh_wire`, `nsh_crypto`, `nsh_client` |
| [nsh-v1-wire.md](nsh-v1-wire.md) | Framing, stage caps, hellos, transcript, keys, labels, sealed frames, `HOST_PROOF`, `USER_AUTH`, `AUTH_OK`, `AUTH_REFUSED`, `ABORT`. | `userland/nsh_wire`, `userland/nsh_crypto` |
| [nsh-v1-wire-session.md](nsh-v1-wire-session.md) | Session frames, what each side may receive in each phase, the ending order, reason codes. | `userland/nsh_wire`, `userland/nsh_client` |
| [nsh-v1-attest.md](nsh-v1-attest.md) | The client's check of a host, steps 1 to 8, `nsh-enrol/1`, the challenge, PCR rules, enrolment. | `userland/nsh_attest`, `userland/nsh_client`, `tools/nsh-enrol` |
| [nonosatt-v5.md](nonosatt-v5.md) | The `NONOSATT` version 5 attestation document and the 45-byte registry entry. | `src/security/attest_doc`, `src/security/attest_registry`, `userland/nsh_attest` |
| [nsh-v1-roles.md](nsh-v1-roles.md) | `nsh-grant/1`, approvals, `nsh-policy/1`, `nsh-revoke/1`, shipped roles, forbidden capabilities, login and escalation checks. | `userland/nsh_wire` (grant), `userland/nsh_roles` |
| [nsh-v1-record.md](nsh-v1-record.md) | `nsh-record/1`, the chain, `nsh-ckpt/1`, checking a record offline. | `userland/nsh_wire` (record), `userland/nsh_client` (checker) |
| [nsh-v1-refusals.md](nsh-v1-refusals.md) | Every refusal by side, its reason code, what is logged and what is told. | `userland/nsh_client`, `userland/nsh_attest`, `userland/nsh_roles`, `userland/nsh_server`, `userland/nsh_wire` |
| [nsh-server-runbook.md](nsh-server-runbook.md) | The `server` profile: where the host keys live, the seal, a machine's first boot, its enrolment and records, what the client says. | `userland/capsule_nsh_attest`, `userland/capsule_nsh_front`, `userland/nsh_front_wire` |

## Crates

| Crate | Part |
|---|---|
| `userland/nsh_wire` | Byte layouts and parsers: frames, stage caps, hellos, `HOST_PROOF`, `nsh-enrol/1`, `nsh-grant/1`, session bodies, reason codes, `nsh-record/1`. `no_std`, no allocation. |
| `userland/nsh_crypto` | X-Wing exchange, transcript chain, key schedule, labelled digests, attestation challenge, sealed frames. |
| `userland/nsh_client` | The client state machine, the host check, the enrolment check, the offline record checker, `nsh-ckpt/1`, the `nsh-verify-record` tool. |
| `userland/nsh_attest` | The `NONOSATT` version 5 checker: steps 1 to 7. |
| `userland/nsh_roles` | `nsh-policy/1`, `nsh-revoke/1`, the shipped roles, the server's login and escalation checks. |
| `userland/nsh_server` | The host state machine: the hellos, `HOST_PROOF` over the kernel's document, the login through `nsh_roles`, the session, escalations, and the record it writes, checkpoints and signs. |
| `src/security/attest_doc` | The kernel's `NONOSATT` version 5 encoder and the quote binding. |
| `src/security/attest_registry` | The capsule registry, its 45-byte entry and its root. |
| `tools/nsh-enrol` | The fleet operator's enrolment tool. |

Proofs are in `userland/nsh_wire_proofs`, `userland/nsh_crypto_proofs`,
`userland/nsh_client_proofs`, `userland/nsh_attest_proofs`,
`userland/nsh_roles_proofs` and `userland/nsh_server_proofs`, which runs
the server against the client in memory and over TCP. Known-answer vectors are in
`userland/nsh_crypto_proofs/vectors/`.
