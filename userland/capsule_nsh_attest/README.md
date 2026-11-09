# capsule_nsh_attest

## Role

`capsule_nsh_attest` is `nsh.attest`, the half of the nsh/1 server that holds
the machine's secrets. It answers `nsh.front` alone, and gives it:

- the kernel's attestation document for each handshake's challenge, and the
  registry's entries;
- the host's hybrid public keys, and its signatures, made with keys this
  machine made at its first boot and sealed to its TPM;
- the machine's own records for each connection.

No host secret leaves this capsule. The protocol is in
[nsh/1](../../docs/nsh/README.md), and the server profile's first boot, seal
and enrolment are in the [runbook](../../docs/nsh/nsh-server-runbook.md).

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x80000079
service    nsh.attest                    (service:4486)
reply      endpoint.nsh.attest.reply     (reply:4487)
```

The manifest grants `CoreExec`, `IPC`, `Memory`, `Crypto`, `FileSystem` and
`AttestRead`. It does not grant `Network`: the capsule has no way to reach the
network, so a fault in the code that faces it cannot carry the keys away.
`Debug` is granted only to a build that compiles `capsule-serial-debug`.

The kernel starts the capsule only when the image is built with
`nonos-capsule-nsh-attest`, which the `nonos-server` feature turns on. It
answers the one measured process the kernel bound the name `nsh.front` to, and
refuses any other sender.

## Layout

| File | What it holds |
|---|---|
| `main.rs` | startup |
| `server.rs`, `caller.rs` | the service loop, and the one-caller rule: `nsh.front` and no one else |
| `request.rs`, `reply.rs` | the frames to and from `nsh.front` |
| `attest.rs` | the two answers that need `AttestRead`: the `NONOSATT` v5 document over a challenge, and the attestation registry |
| `challenge.rs`, `anchor.rs` | the enrolment request's challenge and nonce text, and the role table's anchor as the operator puts it on the machine |
| `keys/` | the host's hybrid keys, held here and by no other capsule |
| `store.rs` | the machine's store through the VFS: the sealed host keys, the enrolment request, the records |
| `records.rs` | the machine's own records, read for each connection and handed to `nsh.front` |
| `rng.rs`, `said.rs` | the kernel's random source, and the serial lines about the keys and records |

## Proofs

The server is proved against the client, in memory and over TCP, by
`userland/nsh_server_proofs`. The document and quote checks are proved by
`userland/nsh_attest_proofs`.
