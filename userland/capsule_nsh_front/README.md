# capsule_nsh_front

## Role

`capsule_nsh_front` is `nsh.front`, the network-facing half of the nsh/1
server. It listens on a `net.sockets` stream, drives the sans-IO `nsh_server`
for each connection, and serves a minimal shell behind the sealed session. For
the attestation challenge, the host's signatures and the machine's records it
asks `nsh.attest`, which holds the keys. It holds no host secret itself. It
serves one connection at a time; a newcomer meanwhile is told `BUSY`.

The protocol is in [nsh/1](../../docs/nsh/README.md), and the server profile's
first boot, seal and enrolment are in the
[runbook](../../docs/nsh/nsh-server-runbook.md).

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x0000001D
service    nsh.front                     (service:4488)
reply      endpoint.nsh.front.reply      (reply:4489)
```

The manifest grants `CoreExec`, `Network`, `IPC` and `Memory`, and nothing
else. It does not grant `AttestRead`, so it cannot read the attestation
document or the TPM; it can only ask `nsh.attest` for them. `Debug` is granted
only to a build that compiles `capsule-serial-debug`.

The kernel starts the capsule only when the image is built with
`nonos-capsule-nsh-front`, which the `nonos-server` feature turns on. Setup
waits for its two peers, `net.sockets` and `nsh.attest`, and opens the listener
before it takes the first client.

## Layout

| File | What it holds |
|---|---|
| `main.rs`, `setup.rs`, `server.rs` | startup, waiting for the two peers, the listener and the accept loop |
| `session.rs`, `accepted.rs`, `read.rs`, `line.rs` | one connection driven through `nsh_server`, and what each accept, read and input line means |
| `newcomers.rs`, `turn_away.rs`, `unready.rs`, `limits.rs` | `BUSY` for newcomers, why the host cannot serve yet, and the time bounds on a connection |
| `sock.rs`, `net_ask.rs`, `net_rule.rs` | the client of `net.sockets`, and the `nslookup` and `ping` a session may ask for |
| `attest.rs`, `host.rs`, `relay.rs`, `reply.rs` | the calls relayed to `nsh.attest`: the attestation bridge, the host's keys as this capsule reaches them (never the keys), and the reply checks |
| `records.rs`, `marks.rs`, `clock.rs` | the machine's records for one connection, the newest role table and revocation list served, and the host clock |
| `shell.rs`, `shell_host.rs`, `procs.rs` | the shell behind the session, what it may ask of this machine, and the process table for `ps` |
| `rng.rs`, `refused_said.rs`, `relay_said.rs` | the entropy the server signs with, and the serial lines for a refusal and for a failed relay |

## Proofs

The server is proved against the client, in memory and over TCP, by
`userland/nsh_server_proofs`.
