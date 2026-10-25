# nsh v1: server layout and what the kernel must add

## Capsules

| Capsule | Holds | Does |
|---|---|---|
| `nsh.front` | CoreExec, IPC, Memory, Network, ProcessControl, a new spawn capability (K2) | Listens on TCP, runs the handshake, checks grants, starts and ends sessions, relays the terminal bytes, writes records |
| `nsh.attest` | CoreExec, IPC, Memory, AttestRead | Answers one call: the document and the registry entries for a 32-byte challenge |
| `nsh.keys` | CoreExec, IPC, Memory, Crypto | Holds the host's hybrid key, sealed to the measured chain (`src/security/tpm/machine_key`). It takes a label number and a 64-byte inner digest: 1 for `nsh/1 host-sig`, 2 for `nsh/1 record-sig`, 3 for `nsh/1 record-ckpt`. It builds `H(label \|\| digest)` itself and signs that. It never signs a finished digest, so a front that has been taken over cannot have any other value signed |
| `nsh.session` | whatever the grant names, within the role's ceiling | A headless shell. Its stdin and stdout belong to `nsh.front`, its parent (`MkProcInput`/`MkProcOutput`) |

TCP sockets cannot be handed from one capsule to another; the handles are
keyed by (pid, handle). So `nsh.front` keeps the connection and relays the
bytes. This is also the safer layout: a session capsule never sees a
socket or a key.

## What the kernel and the stack must add

| Id | Change | Where | Why |
|---|---|---|---|
| K1 | `NONOSATT` version 5, which also quotes PCRs 4, 9 and 14 and carries each quoted PCR value | `src/security/attest_doc/produce.rs`, `document.rs` | Version 3 quotes only PCRs 0, 1, 2 and 7, so it does not bind the loader, the kernel or the proof |
| K2 | Spawn a verified capsule with a mask that is a subset of its manifest's capabilities, gated by a new capability rather than Admin | `src/kernel_core/process_spawn`, `src/syscall/microkernel` | Today a child's capabilities are fixed by its manifest or the tool spec, and `MkCapGrant` needs Admin |
| K3 | A capsule with a hard end time: the kernel ends it when the time comes | the process table and the timer | The escalation expiry must not depend on `nsh.front` staying correct |
| K4 | The parent can read the refused-call events of its child | `src/capabilities/audit`, `src/syscall` | So refusals reach the record |
| N1 | Listening TCP in the server image | the image profile: `net.tcp`, `net.sockets`, `net.ip`, `net.l2` | `net.core`, which the desktop images use, refuses `OP_LISTEN` |
| N2 | A way to wait for a pending accept | `capsule_net_tcp` and `capsule_net_sockets` (`OP_POLL` is sent to `net.tcp`, which does not have it) | Today accept retries 32 times and then gives up |

## Questions for the lead

- **Q1. One ML-DSA-65 wrapper.** The PQClean source is compiled three times
  today: in the kernel, the loader and `nonos-sign`. I propose one root
  crate, `nonos-mldsa65`, that is `no_std`, sign and verify, and built from
  `third_party/pqclean` with the pin check kept. nsh's server and client
  would use it. Folding the kernel, the loader and `nonos-sign` onto it is
  your call; nsh does not need that in 0.9.4.
- **Q2. HKDF-SHA-512 goes into `nonos_hash`.** It is about 20 lines over
  `HmacSha512`, with Wycheproof vectors. Is that acceptable as a change to
  a shared crate?
- **Q3. The command layer.** The command layer of `capsule_terminal`
  (`command/`) is tied to `Scrollback` and `State`. I propose lifting the
  parser and builtins into a crate that both the terminal and
  `nsh.session` use. That touches `capsule_terminal`, which is in the
  0.9.3 seal batch, so I ask before any edit and would start only after
  28 October.
- **Q4. The port number.** Answered: 4422 is the default, and the server
  profile can change it.
- **Q5. K1 to K4 are kernel work.** Do I do them, or does the kernel track,
  with me writing the proofs?
