# A guest family's network, granted without Network

The kernel reports facts, and a userland service decides. This replaces
the respawn into a network role
(`app.linux.term.net`, `app.linux.run.net`), after which the whole
personality held Network.

## What was wrong with the respawn

A run whose program's policy grants NET_ANON started in a role without
Network, proved and bound its program, asked the kernel by `MkLinuxRunNet`,
exited with `LINUX_ASK_NET`, and was started again in a role holding Network,
where it read, proved and bound its program again.

- **Time.** The first role and the second spawn gate cost 0.65 to 0.78 s on
  the least-loaded TCG boot (docs/release-evidence/0.9.3/lock/bm.serial.log
  lines 1886 to 1919), a median 1.1 s on KVM across 11 runs
  (kvm-guest-gate), and 44 to 56 s in the loaded boot-2.
- **Isolation.** The second role held Network for the whole run. An 82K-line
  capsule that answers hostile guests' system calls could then reach
  `net.tcp`, `net.ip`, `net.udp`, `net.dns` and every other direct service:
  the rule that a guest's streams leave only over Nym or Anyone was the
  personality's to keep, and nothing below it kept it.

## The design

```
 guest ─trap─▶ personality (ForeignExec, no Network)
                   │ GRANT, then streams
                   ▼
        net.socks5 / net.anon (Network)  ──MkFamilyFact──▶ kernel
          decide the grant, carry streams                  (facts only)
```

### Kernel: facts and plumbing, nothing about Linux

1. **`MkFamilyFact(pid, what, ptr, len)`**, tag `MFAF`, one new call.
   - `what = 1`: is the 32-byte digest at `ptr` among the measurements the
     kernel recorded for the live guest family `pid` supervises, under one of
     the kinds in `len`? 1 or 0. The list is the seal's own
     (`src/process/foreign/seal/family_rows.rs`), filled only by
     `MkLocalVerify` and `MkCapsuleVerify` acceptances, and beside each entry
     the kernel keeps which of the two accepted it (`seal/kind.rs`: LOCAL 1,
     PUBLISHER 2). The kinds are not in the record: M, the list and the seal
     are byte for byte what they were. `len` 0, or naming a kind the kernel
     does not keep, is EINVAL.
   - `what = 2`: the generation of the live guest family `pid` supervises,
     or 0 for none. Each family the kernel reserves is numbered from 1 and
     no number is given twice (`seal/generation.rs`, set in `seal/open.rs`
     `reserve`). It is not the seal's salt or trace start, which never leave
     the kernel, and not the seal number, which a live family does not have.
   - `what = 3`: `pid`'s argv as the kernel set it at spawn, NUL-separated,
     copied to `ptr` up to `len`; its length. Only the spawn path writes a
     process's argv (`capsule_spawn` admit, the terminal slots' admit, both
     before its first instruction); no system call does, so neither the
     personality nor a guest can change it.
   - Callers: only the processes registered for `net.anon` and
     `net.socks5`, and they also need Network (EPERM otherwise). `pid`
     must hold ForeignExec (ESRCH otherwise), so no other capsule's argv is
     ever read through it.
   - The kernel parses nothing: no policy, no box, no Linux format. It
     answers about bytes it already holds.
2. **The registry rule** (`src/services/registry/policy.rs`,
   `src/syscall/microkernel/ipc/send_caps.rs`). An endpoint that needs
   Network still needs it, with one exception: a holder of ForeignExec may
   send to `net.anon` and `net.socks5` without Network, and to no other
   network service. `net.tcp`, `net.ip`, `net.udp`, `net.dns`,
   `net.dhcp.client`, `net.core`, `net.l2`, `net.sockets` and `net.nym`
   stay closed to it. So Nym/Anyone-only is the kernel's rule, not the
   personality's. The rule is one function, `caps_admit`, applied by every
   capability check a send passes, each through its rule in
   `registry/send_rule.rs`: the endpoint's own (`send_caps.rs`), the gate of
   every endpoint the receiving process serves (`peers_inbox.rs`), which a
   send by name and a send by pid both take, and the route's last check
   before the message is queued (`ipc/kernel_ipc.rs`), which says a refusal
   on serial as `[ROUTE-DENY]` with the pid, the target, what it needs and
   what it holds. Each front's process serves its front name and its reply
   inbox, which needs IPC alone; any other endpoint it served would still
   need its own capabilities.

The sender pid every IPC frame carries is stamped by the kernel. A guest
holds no capability and sends nothing, so a frame's sender is the
supervisor itself: the supervisor's pid is the family's key, and no further
pid-to-family lookup is needed.

### Userland: the decision

The authority is the two anonymity fronts themselves, each deciding with
the same shared crate, `userland/linux_net_grant` (pure, `no_std`, host
tested). A separate grant service was considered and set aside: there is no
capability object to hand over IPC (capabilities are ambient bits), so a
grant service would still have to be asked by each front on each stream,
and it would be one more capsule to sign and enroll, holding the same check.

A **grant** is a row in a front's own table: the supervisor's pid, the
generation of the family it was made for, and what it was granted on.
Nothing is handed to the personality; the front admits or refuses each
request by its row, and asks the kernel the family's generation on every
request: a family sealed and the next begun under the same pid, or a pid
reused, is never served on an earlier family's row. A grant is asked for
once a front, at the family's first stream through it, and decided with the
generation read before and after its facts (`decide.rs`).

- **By policy** (`GRANT_POLICY`, body: the policy's bytes). Granted when
  - the bytes decode as a Linux app policy (`nonos_linux_policy`, exact
    length, its own format checks);
  - its mask has NET_ANON;
  - the kernel says the family's measurements hold, under a local trailer
    (kind LOCAL), the policy's vouched hash, BLAKE3 of `NONOS.linux.policy.v1` then the policy bytes (its
    `policy_hash`, `nonos_linux_policy::vouch`): the personality had the
    kernel verify the policy's local trailer over exactly those bytes inside
    the family (`app/bind.rs`, after the guest exists), and the kernel records
    only what it accepted. This is the trailer check, done by the kernel once
    and read back here, so the fronts need no `MkLocalVerify` (which stays
    ForeignExec's);
  - and the kernel says they hold the policy's `program_blake3` under
    either kind: the program it names was proved in this family.
  - Each is the BLAKE3 of the bytes verified: `MkLocalVerify` hashes the image
    (`src/syscall/microkernel/local_image/digest.rs`, `in_chunks`) and records
    what the verify gives back (`local_verify.rs:52`), which for a local
    trailer is the first 32 bytes of its context, the digest itself
    (`security/local_build/sign.rs:65-66`, `trailer.rs:37`), and for a vendor
    or developer path the digest (`capsule_attest/path.rs:51,76`). Held by
    `kernel_proofs` `measured_plain_tests` and `crypto_proofs`
    `the_context_begins_with_the_digest_unchanged`.
  - A policy grant reaches any destination over the anonymity network, as
    the policy does.
- **By box** (`GRANT_BOX`, no body). Granted when the supervisor's argv,
  read from the kernel, is a Terminal run whose `--box` names `net:`
  destinations (the rule that was `src/userspace/tool_capsules/net_box.rs`,
  moved out of ring 0 into the crate). The person typed that box; the
  Terminal could not have spawned a run with that argv otherwise. The grant
  carries the box's destinations, parsed by the personality's own parser
  (`boxed/dests.rs`, mounted) into the canonical list
  `nonos_linux_policy::dests` matches, and the front opens a stream only to
  one of them: net.anon checks each `OP_OPEN_STREAM`, net.socks5 each CONNECT
  before its tunnel is opened. So a compromised personality on a box grant
  reaches the box's destinations and nothing else. The argv is read only to
  find the box and is never logged or kept by a front.
- **What a front says.** Each decision leaves one serial line,
  `[SOCKS5] grant <code>` or `[ANON] grant <code>`: 0 for granted, else the
  refusal's number (`refusal.rs`). Nothing the decision read is logged.
- **Admission.** A sender holding Network is served as before. Any other
  sender is served only with a row whose generation is its live family's,
  and on net.anon only the four ops a guest's streams take (open, send,
  receive, close). A row dies with its family; the fronts' reaping drops
  rows whose family has ended.

### The personality

- It never holds Network. Every role but install runs without it. The
  install role keeps Network for the package mirror, as before, and is the
  one remaining Network holder in capsule_linux; a later step can give it
  the same grant shape.
- `MkLinuxRunNet`, the ticket, `app.linux.run.net`, `app.linux.term.net`,
  `tool.linux.net`, `LINUX_ASK_NET`, the Terminal's ask-again and
  `run_net::early` all go. The start proves the program and binds its
  policy after the family exists (`start_guest/prove.rs`), so both
  measurements are in the family before its first stream.
- At a family's first stream through a front, the personality asks that
  front for a grant: by policy when the bound program's own policy has
  NET_ANON, by box when the run's box names destinations. A refused grant
  makes the connect `ENETUNREACH`, said as today.
- Destination lists stay the personality's to enforce, as they are today:
  the fronts grant the anonymity network, never a destination.

## The alternative: the kernel decides

In design (a) the kernel would parse the Linux policy to read NET_ANON and
keep the grant itself. Design (b) is the one above.

| | (a) kernel decides | (b) fronts decide |
|---|---|---|
| Ring 0 added | policy decoding, a grant table, a new grant call, the box rule kept in ring 0 | one fact call (three answers), one registry exception; the box rule leaves ring 0 |
| Trusted for | the kernel for Linux formats | the kernel for the measured set and argv it already keeps; the fronts for the grant check, beside the streams they already carry |
| A policy format bug | a ring-0 parser fault | refuses or admits one family's anonymity streams; never a direct socket |
| Nym/Anyone only | registry rule plus kernel grant | the same registry rule; the fronts carry only anonymity streams |
| Failure if a front is compromised | no change | it could admit any personality to its own network; it holds Network already, so it could relay for one anyway |

(b) adds less to ring 0, takes a Linux format out of it, and leaves the
fronts trusted only for what they already carry.

## What each part is trusted for

- **Kernel:** the measured set is only what it verified; argv is what it set
  at spawn; a sender pid is the sender; the registry exception names two
  services.
- **net.anon, net.socks5:** grant only on the checks above; carry only
  anonymity streams (unchanged).
- **The personality:** prove before run; the box's and policy's destination
  lists; no guest reaching another's memory. It is no longer trusted to keep
  a guest off the direct network: it cannot reach it.

## Open questions

- A blob attested as a program that parses as a policy: closed, with no
  ring-0 code. The kernel measures the plain BLAKE3 of whatever it verifies,
  so the policy is told apart by what was signed. A policy is vouched for
  and verified as `NONOS.linux.policy.v1` then its bytes, and the front
  looks only for that hash. The machine mints nothing plain that begins with
  `NONOS.linux.` (`install/record/vouch.rs` `mint`; only `sign_beside_in`
  signs under a domain), and the personality proves no program, local or
  publisher's, that begins with it (`attest_local.rs`,
  `attest_publisher.rs`). So no downloaded file, index or program the
  machine vouched for, and no image a publisher attested and an honest
  personality proved, is measured under the policy's hash. Held by
  `linux_net_grant` `a_program_shaped_attested_blob_that_parses_as_a_policy_is_refused`,
  `capsule_linux_proofs` `policy_domain_tests` and four mutants in
  `verification/host_mutants/grant-net.json`.
  - A compromised personality calling `MkCapsuleVerify` itself on a payload
    a NONOS-certified publisher signed that is exactly the domain followed by
    a policy: closed in the kernel. Each measurement keeps the call that
    verified it (`seal/kind.rs`, `measured(m, kind)` at `local_verify.rs` and
    `capsule_verify/gate.rs`), and the fronts ask for the policy under LOCAL
    alone. Held by `linux_net_grant`
    `a_publisher_verified_blob_that_is_the_domain_and_a_policy_is_refused_as_a_policy`,
    `seal_proofs` `kind_tests`, `kernel_proofs`
    `each_measurement_is_kept_under_the_call_that_verified_it` and five
    mutants in `grant-net.json`.
  - A policy vouched for before the policy domain existed was signed over
    its plain bytes, so it no longer verifies: its program runs refused
    until the package is installed again, which signs it under the domain.
- A pid reused after its process ended, before the front reaps its row:
  closed. The row's generation is checked on every request, and a new
  process's family has a new generation.
